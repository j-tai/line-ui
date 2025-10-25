[private]
help:
    @just --list

# Analyze code coverage.
coverage:
    rm -rf target/package
    cargo tarpaulin --all-features --out=html --output-dir=target --skip-clean --target-dir=target/_tarpaulin
    #
    # Code coverage information has been written to target/tarpaulin-report.html
    #

version_file := source_directory() / "target" / ".new_version"
publish version="$(read -p 'New version: ' v; echo $v)":
    grep '^version =' Cargo.toml
    @echo {{version}} > {{version_file}}
    sed -i 's/^version = "[^"]*"$/version = "'"$(cat {{version_file}})"'"/' Cargo.toml
    grep '^version =' Cargo.toml
    cargo test
    git add Cargo.*
    git commit -m "Bump version to $(cat {{version_file}})"
    git tag "v$(cat {{version_file}})" -m "v$(cat {{version_file}})"
    cargo publish
    git push
    git push --tags
