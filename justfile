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
