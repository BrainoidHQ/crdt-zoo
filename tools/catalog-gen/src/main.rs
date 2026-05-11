use anyhow::{anyhow, bail, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::process;
use tera::{Context as TeraContext, Tera};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize)]
struct Algorithm {
    id: String,
    slug: String,
    name: String,
    long_name: Option<String>,
    family: String,
    difficulty: String,
    summary: String,
    kind: Vec<String>,
    tags: Vec<String>,
    docs: Docs,
    rust: RustMeta,
    proofs: Proofs,
    delivery: Option<Delivery>,
    features: Option<Features>,
    properties: Option<Properties>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Docs {
    readme: String,
    spec: String,
    examples: String,
    proofs: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RustMeta {
    #[serde(rename = "crate")]
    crate_name: String,
    module: String,
    #[serde(rename = "type")]
    type_name: String,
    status: Option<String>,
    source: Option<String>,
    api_docs_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Proofs {
    lean: Option<LeanProof>,
    tla: Option<TlaProof>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LeanProof {
    status: String,
    file: String,
    theorems: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TlaProof {
    status: String,
    file: String,
    model: Option<String>,
    checked_bounds: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Delivery {
    causal_required: bool,
    duplicate_tolerant: bool,
    drop_tolerant: bool,
    exactly_once_required: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Features {
    serde: bool,
    no_std: bool,
    delta: bool,
    tombstone_free: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Properties {
    laws: Vec<String>,
}

#[derive(Serialize)]
struct CatalogJsonEntry<'a> {
    #[serde(flatten)]
    algorithm: &'a Algorithm,
    url: String,
}

fn main() -> Result<()> {
    let check_only = std::env::args().skip(1).any(|arg| arg == "--check");
    let repo = Utf8PathBuf::from(".");
    let catalog_dir = repo.join("catalog");
    let out_dir = repo.join("docs/book/src");

    let algorithms = load_algorithms(&catalog_dir)?;
    validate_algorithms(&catalog_dir, &algorithms)?;

    if check_only {
        let check_dir = std::env::temp_dir().join(format!("catalog-gen-check-{}", process::id()));
        let check_dir = Utf8PathBuf::from_path_buf(check_dir)
            .map_err(|path| anyhow!("non-utf8 temp path: {}", path.display()))?;
        if check_dir.exists() {
            fs::remove_dir_all(&check_dir)
                .with_context(|| format!("failed to clean {check_dir}"))?;
        }
        render_site(&catalog_dir, &check_dir, &algorithms)?;
        fs::remove_dir_all(&check_dir).with_context(|| format!("failed to clean {check_dir}"))?;
        return Ok(());
    }

    clean_generated(&out_dir)?;
    render_site(&catalog_dir, &out_dir, &algorithms)?;

    Ok(())
}

fn load_algorithms(catalog_dir: &Utf8Path) -> Result<Vec<Algorithm>> {
    let mut algorithms = Vec::new();

    for entry in WalkDir::new(catalog_dir).min_depth(2).max_depth(2) {
        let entry = entry?;
        if !entry.file_type().is_file() || entry.file_name() != "algorithm.toml" {
            continue;
        }

        let path = Utf8PathBuf::from_path_buf(entry.path().to_path_buf())
            .map_err(|path| anyhow!("non-utf8 path: {}", path.display()))?;
        let text = fs::read_to_string(&path).with_context(|| format!("failed to read {path}"))?;
        let algorithm: Algorithm =
            toml::from_str(&text).with_context(|| format!("failed to parse {path}"))?;

        let dir_name = path
            .parent()
            .and_then(Utf8Path::file_name)
            .ok_or_else(|| anyhow!("algorithm.toml has no parent directory: {path}"))?;
        if algorithm.id != dir_name {
            bail!(
                "catalog directory and algorithm id differ: directory={dir_name}, id={}",
                algorithm.id
            );
        }

        algorithms.push(algorithm);
    }

    algorithms.sort_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)));
    Ok(algorithms)
}

fn validate_algorithms(catalog_dir: &Utf8Path, algorithms: &[Algorithm]) -> Result<()> {
    if algorithms.is_empty() {
        bail!("no algorithms found under {catalog_dir}");
    }

    let allowed_difficulties = ["beginner", "intermediate", "advanced", "research"];
    let allowed_kinds = [
        "state-based",
        "operation-based",
        "delta-state",
        "hybrid",
        "CvRDT",
        "CmRDT",
    ];
    let allowed_proof_statuses = [
        "proved",
        "partial",
        "planned",
        "not-provided",
        "model-checked",
        "unchecked",
        "failed",
    ];
    let allowed_rust_statuses = ["implemented", "planned", "prototype", "not-provided"];

    let mut ids = BTreeSet::new();
    let mut slugs = BTreeSet::new();

    for algorithm in algorithms {
        ensure_unique(&mut ids, "id", &algorithm.id)?;
        ensure_unique(&mut slugs, "slug", &algorithm.slug)?;
        ensure_slug(&algorithm.slug)?;
        ensure_allowed(
            "difficulty",
            &algorithm.difficulty,
            &allowed_difficulties,
            &algorithm.id,
        )?;
        ensure_non_empty("summary", &algorithm.summary, &algorithm.id)?;

        for kind in &algorithm.kind {
            ensure_allowed("kind", kind, &allowed_kinds, &algorithm.id)?;
        }

        if let Some(status) = &algorithm.rust.status {
            ensure_allowed("rust.status", status, &allowed_rust_statuses, &algorithm.id)?;
        }

        let dir = catalog_dir.join(&algorithm.id);
        validate_doc(&dir, &algorithm.docs.readme, algorithm)?;
        validate_doc(&dir, &algorithm.docs.spec, algorithm)?;
        validate_doc(&dir, &algorithm.docs.examples, algorithm)?;
        validate_doc(&dir, &algorithm.docs.proofs, algorithm)?;

        if let Some(source) = &algorithm.rust.source {
            validate_relative_path(&dir, source, &algorithm.id, "rust.source")?;
        }

        if let Some(lean) = &algorithm.proofs.lean {
            ensure_allowed(
                "proofs.lean.status",
                &lean.status,
                &allowed_proof_statuses,
                &algorithm.id,
            )?;
            validate_relative_path(&dir, &lean.file, &algorithm.id, "proofs.lean.file")?;
            if lean.theorems.is_empty() {
                bail!("{} has Lean metadata but no theorem names", algorithm.id);
            }
        }

        if let Some(tla) = &algorithm.proofs.tla {
            ensure_allowed(
                "proofs.tla.status",
                &tla.status,
                &allowed_proof_statuses,
                &algorithm.id,
            )?;
            validate_relative_path(&dir, &tla.file, &algorithm.id, "proofs.tla.file")?;
            if let Some(model) = &tla.model {
                validate_relative_path(&dir, model, &algorithm.id, "proofs.tla.model")?;
            }
        }

        if let Some(properties) = &algorithm.properties {
            if properties.laws.is_empty() {
                bail!("{} has an empty properties.laws list", algorithm.id);
            }
        }
    }

    Ok(())
}

fn ensure_unique(set: &mut BTreeSet<String>, field: &str, value: &str) -> Result<()> {
    if !set.insert(value.to_owned()) {
        bail!("duplicate {field}: {value}");
    }
    Ok(())
}

fn ensure_slug(slug: &str) -> Result<()> {
    if slug.is_empty()
        || !slug
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
    {
        bail!("invalid slug: {slug}");
    }
    Ok(())
}

fn ensure_allowed(field: &str, value: &str, allowed: &[&str], algorithm_id: &str) -> Result<()> {
    if !allowed.contains(&value) {
        bail!(
            "{algorithm_id} has unsupported {field}: {value}; expected one of {}",
            allowed.join(", ")
        );
    }
    Ok(())
}

fn ensure_non_empty(field: &str, value: &str, algorithm_id: &str) -> Result<()> {
    if value.trim().is_empty() {
        bail!("{algorithm_id} has empty {field}");
    }
    Ok(())
}

fn validate_doc(dir: &Utf8Path, doc: &str, algorithm: &Algorithm) -> Result<()> {
    ensure_relative(doc, &algorithm.id, "docs")?;
    let path = dir.join(doc);
    if !path.exists() {
        bail!("missing doc for {}: {path}", algorithm.id);
    }
    let text = fs::read_to_string(&path).with_context(|| format!("failed to read {path}"))?;
    if text.trim().is_empty() {
        bail!("empty doc for {}: {path}", algorithm.id);
    }
    Ok(())
}

fn validate_relative_path(
    algorithm_dir: &Utf8Path,
    relative_path: &str,
    algorithm_id: &str,
    field: &str,
) -> Result<()> {
    ensure_relative(relative_path, algorithm_id, field)?;
    let path = algorithm_dir.join(relative_path);
    if !path.exists() {
        bail!("{algorithm_id} references missing {field}: {path}");
    }
    Ok(())
}

fn ensure_relative(path: &str, algorithm_id: &str, field: &str) -> Result<()> {
    let path = Utf8Path::new(path);
    if path.is_absolute() {
        bail!("{algorithm_id} has absolute path in {field}: {path}");
    }
    Ok(())
}

fn clean_generated(out_dir: &Utf8Path) -> Result<()> {
    remove_if_exists(&out_dir.join("SUMMARY.md"))?;
    remove_if_exists(&out_dir.join("algorithms"))?;
    remove_if_exists(&out_dir.join("indexes"))?;
    remove_if_exists(&out_dir.join("assets/catalog.json"))?;
    Ok(())
}

fn remove_if_exists(path: &Utf8Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    if path.is_dir() {
        fs::remove_dir_all(path).with_context(|| format!("failed to remove {path}"))?;
    } else {
        fs::remove_file(path).with_context(|| format!("failed to remove {path}"))?;
    }
    Ok(())
}

fn render_site(catalog_dir: &Utf8Path, out_dir: &Utf8Path, algorithms: &[Algorithm]) -> Result<()> {
    fs::create_dir_all(out_dir.join("algorithms"))?;
    fs::create_dir_all(out_dir.join("indexes"))?;
    fs::create_dir_all(out_dir.join("assets"))?;

    let tera = Tera::new("tools/catalog-gen/templates/**/*.tera")
        .context("failed to load catalog-gen templates")?;

    for algorithm in algorithms {
        render_algorithm_page(&tera, catalog_dir, out_dir, algorithm)?;
    }
    render_summary(out_dir, algorithms)?;
    render_indexes(out_dir, algorithms)?;
    render_catalog_json(out_dir, algorithms)?;

    Ok(())
}

fn render_algorithm_page(
    tera: &Tera,
    catalog_dir: &Utf8Path,
    out_dir: &Utf8Path,
    algorithm: &Algorithm,
) -> Result<()> {
    let dir = catalog_dir.join(&algorithm.id);
    let readme = read_doc_without_h1(&dir.join(&algorithm.docs.readme))?;
    let spec = read_doc_without_h1(&dir.join(&algorithm.docs.spec))?;
    let examples = read_doc_without_h1(&dir.join(&algorithm.docs.examples))?;
    let proof_docs = read_doc_without_h1(&dir.join(&algorithm.docs.proofs))?;

    let rust_source_path = optional_repo_path(&dir, algorithm.rust.source.as_deref())?;
    let lean_file_path = optional_repo_path(
        &dir,
        algorithm
            .proofs
            .lean
            .as_ref()
            .map(|lean| lean.file.as_str()),
    )?;
    let tla_file_path = optional_repo_path(
        &dir,
        algorithm.proofs.tla.as_ref().map(|tla| tla.file.as_str()),
    )?;
    let tla_model_path = optional_repo_path(
        &dir,
        algorithm
            .proofs
            .tla
            .as_ref()
            .and_then(|tla| tla.model.as_deref()),
    )?;

    let mut context = TeraContext::new();
    context.insert("alg", algorithm);
    context.insert("readme", &readme);
    context.insert("spec", &spec);
    context.insert("examples", &examples);
    context.insert("proof_docs", &proof_docs);
    context.insert("rust_source_path", &rust_source_path);
    context.insert("lean_file_path", &lean_file_path);
    context.insert("tla_file_path", &tla_file_path);
    context.insert("tla_model_path", &tla_model_path);

    let rendered = tera
        .render("algorithm.md.tera", &context)
        .with_context(|| format!("failed to render {}", algorithm.id))?;
    fs::write(
        out_dir
            .join("algorithms")
            .join(format!("{}.md", algorithm.slug)),
        rendered,
    )
    .with_context(|| format!("failed to write generated page for {}", algorithm.id))?;

    Ok(())
}

fn read_doc_without_h1(path: &Utf8Path) -> Result<String> {
    let text = fs::read_to_string(path).with_context(|| format!("failed to read {path}"))?;
    Ok(demote_markdown_headings(strip_initial_h1(&text).trim()))
}

fn strip_initial_h1(text: &str) -> &str {
    let Some(first_line_end) = text.find('\n') else {
        return if text.starts_with("# ") { "" } else { text };
    };
    let first_line = &text[..first_line_end];
    if !first_line.starts_with("# ") {
        return text;
    }

    let rest = &text[first_line_end + 1..];
    rest.strip_prefix('\n').unwrap_or(rest)
}

fn demote_markdown_headings(text: &str) -> String {
    let mut demoted = String::new();
    let mut in_fence = false;

    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
        }

        if !in_fence && is_markdown_heading(trimmed) {
            let indent_len = line.len() - trimmed.len();
            demoted.push_str(&line[..indent_len]);
            demoted.push('#');
            demoted.push_str(trimmed);
        } else {
            demoted.push_str(line);
        }
        demoted.push('\n');
    }

    demoted.trim_end().to_owned()
}

fn is_markdown_heading(trimmed: &str) -> bool {
    let hashes = trimmed.chars().take_while(|ch| *ch == '#').count();
    (1..6).contains(&hashes) && trimmed.as_bytes().get(hashes) == Some(&b' ')
}

fn optional_repo_path(algorithm_dir: &Utf8Path, path: Option<&str>) -> Result<Option<String>> {
    let Some(path) = path else {
        return Ok(None);
    };
    Ok(Some(repo_relative_path(algorithm_dir, path)?))
}

fn repo_relative_path(algorithm_dir: &Utf8Path, path: &str) -> Result<String> {
    let full_path = algorithm_dir.join(path);
    let normalized = normalize_path(&full_path);
    let relative = normalized
        .strip_prefix("./")
        .unwrap_or(normalized.as_str())
        .to_owned();
    Ok(relative)
}

fn normalize_path(path: &Utf8Path) -> String {
    let mut parts = Vec::new();

    for component in path.components() {
        match component.as_str() {
            "." => {}
            ".." => {
                parts.pop();
            }
            part => parts.push(part.to_owned()),
        }
    }

    parts.join("/")
}

fn render_summary(out_dir: &Utf8Path, algorithms: &[Algorithm]) -> Result<()> {
    let mut summary = String::new();
    summary.push_str("# Summary\n\n");
    summary.push_str("[Introduction](README.md)\n\n");
    summary.push_str("# Indexes\n\n");
    summary.push_str("- [By family](indexes/by-family.md)\n");
    summary.push_str("- [By kind](indexes/by-kind.md)\n");
    summary.push_str("- [By proof status](indexes/by-proof-status.md)\n");
    summary.push_str("- [By difficulty](indexes/by-difficulty.md)\n\n");
    summary.push_str("# Algorithms\n\n");

    for algorithm in algorithms {
        summary.push_str(&format!(
            "- [{}](algorithms/{}.md)\n",
            algorithm.name, algorithm.slug
        ));
    }

    fs::write(out_dir.join("SUMMARY.md"), summary)
        .with_context(|| format!("failed to write {}", out_dir.join("SUMMARY.md")))?;
    Ok(())
}

fn render_indexes(out_dir: &Utf8Path, algorithms: &[Algorithm]) -> Result<()> {
    render_grouped_index(
        out_dir,
        "by-family.md",
        "By family",
        group_by(algorithms, |algorithm| vec![algorithm.family.clone()]),
    )?;
    render_grouped_index(
        out_dir,
        "by-kind.md",
        "By kind",
        group_by(algorithms, |algorithm| algorithm.kind.clone()),
    )?;
    render_grouped_index(
        out_dir,
        "by-difficulty.md",
        "By difficulty",
        group_by(algorithms, |algorithm| vec![algorithm.difficulty.clone()]),
    )?;
    render_grouped_index(
        out_dir,
        "by-proof-status.md",
        "By proof status",
        group_by(algorithms, proof_status_groups),
    )?;

    Ok(())
}

fn group_by<F>(algorithms: &[Algorithm], mut groups_for: F) -> BTreeMap<String, Vec<&Algorithm>>
where
    F: FnMut(&Algorithm) -> Vec<String>,
{
    let mut groups: BTreeMap<String, Vec<&Algorithm>> = BTreeMap::new();

    for algorithm in algorithms {
        for group in groups_for(algorithm) {
            groups.entry(group).or_default().push(algorithm);
        }
    }

    groups
}

fn proof_status_groups(algorithm: &Algorithm) -> Vec<String> {
    let mut groups = Vec::new();

    if let Some(status) = &algorithm.rust.status {
        groups.push(format!("Rust: {status}"));
    }
    if let Some(lean) = &algorithm.proofs.lean {
        groups.push(format!("Lean: {}", lean.status));
    }
    if let Some(tla) = &algorithm.proofs.tla {
        groups.push(format!("TLA+: {}", tla.status));
    }

    if groups.is_empty() {
        groups.push("No proof metadata".to_owned());
    }

    groups
}

fn render_grouped_index(
    out_dir: &Utf8Path,
    filename: &str,
    title: &str,
    groups: BTreeMap<String, Vec<&Algorithm>>,
) -> Result<()> {
    let mut markdown = format!("# {title}\n\n");

    for (group, algorithms) in groups {
        markdown.push_str(&format!("## {group}\n\n"));
        for algorithm in algorithms {
            markdown.push_str(&format!(
                "- [{}](../algorithms/{}.md) - {}\n",
                algorithm.name, algorithm.slug, algorithm.summary
            ));
        }
        markdown.push('\n');
    }

    fs::write(out_dir.join("indexes").join(filename), markdown)
        .with_context(|| format!("failed to write index {filename}"))?;
    Ok(())
}

fn render_catalog_json(out_dir: &Utf8Path, algorithms: &[Algorithm]) -> Result<()> {
    let entries = algorithms
        .iter()
        .map(|algorithm| CatalogJsonEntry {
            algorithm,
            url: format!("algorithms/{}.html", algorithm.slug),
        })
        .collect::<Vec<_>>();
    let json = serde_json::to_string_pretty(&entries)?;
    fs::write(out_dir.join("assets/catalog.json"), format!("{json}\n"))
        .with_context(|| "failed to write catalog.json")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{demote_markdown_headings, normalize_path, strip_initial_h1};
    use camino::Utf8Path;

    #[test]
    fn strips_only_initial_h1() {
        let text = "# Title\n\n## Section\n\nBody";

        assert_eq!(strip_initial_h1(text), "## Section\n\nBody");
    }

    #[test]
    fn leaves_non_h1_documents_unchanged() {
        let text = "## Section\n\nBody";

        assert_eq!(strip_initial_h1(text), text);
    }

    #[test]
    fn normalizes_relative_paths() {
        let path = Utf8Path::new("catalog/gcounter/../../crates/example/src/lib.rs");

        assert_eq!(normalize_path(path), "crates/example/src/lib.rs");
    }

    #[test]
    fn demotes_headings_outside_code_fences() {
        let text = "## Section\n\n```text\n## Not a heading\n```\n\n### Child";

        assert_eq!(
            demote_markdown_headings(text),
            "### Section\n\n```text\n## Not a heading\n```\n\n#### Child"
        );
    }
}
