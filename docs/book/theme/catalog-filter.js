(function () {
  const root = document.querySelector("[data-catalog-browser]");
  if (!root) {
    return;
  }

  const pathRoot = window.path_to_root || "";

  fetch(`${pathRoot}assets/catalog.json`)
    .then((response) => {
      if (!response.ok) {
        throw new Error(`catalog.json returned ${response.status}`);
      }
      return response.json();
    })
    .then((catalog) => renderCatalog(root, catalog))
    .catch((error) => {
      root.innerHTML = `<p class="catalog-empty">Catalog data is not available: ${escapeHtml(error.message)}</p>`;
    });

  function renderCatalog(container, catalog) {
    const state = {
      family: "all",
      kind: "all",
      proof: "all",
      difficulty: "all",
    };

    const controls = document.createElement("div");
    controls.className = "catalog-controls";

    const results = document.createElement("div");
    results.className = "catalog-results";

    const selectors = [
      ["family", "Family", unique(catalog.map((item) => item.family))],
      ["kind", "Kind", unique(catalog.flatMap((item) => item.kind || []))],
      ["proof", "Proof", unique(catalog.flatMap(proofLabels))],
      ["difficulty", "Difficulty", unique(catalog.map((item) => item.difficulty))],
    ];

    selectors.forEach(([key, label, options]) => {
      controls.appendChild(buildSelect(label, options, (value) => {
        state[key] = value;
        drawResults(results, catalog, state);
      }));
    });

    container.replaceChildren(controls, results);
    drawResults(results, catalog, state);
  }

  function buildSelect(labelText, options, onChange) {
    const label = document.createElement("label");
    label.textContent = labelText;

    const select = document.createElement("select");
    select.appendChild(new Option("All", "all"));
    options.forEach((option) => select.appendChild(new Option(option, option)));
    select.addEventListener("change", () => onChange(select.value));

    label.appendChild(select);
    return label;
  }

  function drawResults(container, catalog, state) {
    const filtered = catalog.filter((item) => {
      return (
        matches(state.family, item.family) &&
        matchesAny(state.kind, item.kind || []) &&
        matchesAny(state.proof, proofLabels(item)) &&
        matches(state.difficulty, item.difficulty)
      );
    });

    if (filtered.length === 0) {
      container.innerHTML = '<p class="catalog-empty">No algorithms match the selected filters.</p>';
      return;
    }

    container.replaceChildren(
      ...filtered.map((item) => {
        const card = document.createElement("article");
        card.className = "catalog-card";

        const title = document.createElement("h3");
        const link = document.createElement("a");
        link.href = item.url;
        link.textContent = item.name;
        title.appendChild(link);

        const summary = document.createElement("p");
        summary.textContent = item.summary;

        const meta = document.createElement("div");
        meta.className = "catalog-meta";
        [item.family, item.difficulty, ...(item.kind || []), ...proofLabels(item)].forEach((label) => {
          const chip = document.createElement("span");
          chip.className = "catalog-chip";
          chip.textContent = label;
          meta.appendChild(chip);
        });

        card.append(title, summary, meta);
        return card;
      })
    );
  }

  function proofLabels(item) {
    const labels = [];
    if (item.rust && item.rust.status) {
      labels.push(`Rust: ${item.rust.status}`);
    }
    if (item.proofs && item.proofs.lean) {
      labels.push(`Lean: ${item.proofs.lean.status}`);
    }
    if (item.proofs && item.proofs.tla) {
      labels.push(`TLA+: ${item.proofs.tla.status}`);
    }
    return labels;
  }

  function matches(selected, value) {
    return selected === "all" || selected === value;
  }

  function matchesAny(selected, values) {
    return selected === "all" || values.includes(selected);
  }

  function unique(values) {
    return [...new Set(values.filter(Boolean))].sort((left, right) => left.localeCompare(right));
  }

  function escapeHtml(value) {
    return String(value).replace(/[&<>"']/g, (char) => {
      return {
        "&": "&amp;",
        "<": "&lt;",
        ">": "&gt;",
        '"': "&quot;",
        "'": "&#39;",
      }[char];
    });
  }
})();
