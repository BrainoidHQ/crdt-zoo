gen-catalog:
    cargo run -p catalog-gen

book: gen-catalog
    mdbook build docs/book

serve: gen-catalog
    mdbook serve docs/book --open

check-catalog:
    cargo run -p catalog-gen -- --check

pages-preview: book
    python3 -m http.server --directory docs/book/book 8000
