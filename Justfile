set dotenv-load := false

help:
    @just --list --unsorted

build:
    cargo build
alias b := build

run *args:
    cargo run {{args}}
alias r := run

release:
    cargo build --release

_download_library:
    #!/usr/bin/env bash
    set -euxo pipefail
    export OS={{ if os() == "macos" { "mac" } else if os() == "linux" { "linux" } else { "unrecognized OS" } }}
    export ARCH={{ if arch() == "aarch64" { "arm64" } else { "unrecognized ARCH" } }}
    export INSTALL_DIR=/usr/local
    export DOWNLOAD_DIR=$(mktemp -d)

    curl https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-$OS-$ARCH.tgz -L -o "$DOWNLOAD_DIR/pdfium.tgz"
    tar xzvf "$DOWNLOAD_DIR/pdfium.tgz" -C "$DOWNLOAD_DIR"

    export PDFIUM_INCLUDE="$INSTALL_DIR/include/pdfium"
    sudo mkdir -p "$PDFIUM_INCLUDE"
    sudo cp -r "$DOWNLOAD_DIR/include/" "$PDFIUM_INCLUDE"

    export PDFIUM_LIB="$INSTALL_DIR/lib"
    sudo mkdir -p "$PDFIUM_LIB"
    sudo cp -r "$DOWNLOAD_DIR/lib/" "$PDFIUM_LIB"

install_with_library:
    @just _download_library
    @just install

install:
    cargo install --path .

bootstrap:
    cargo install cargo-edit

test *args:
    cargo test {{args}}

check:
    cargo check
alias c := check

fix:
    cargo clippy --fix

# Bump version. level=major,minor,patch
version level:
    git diff-index --exit-code HEAD > /dev/null || ! echo You have untracked changes. Commit your changes before bumping the version.
    cargo set-version --bump {{level}}
    cargo update # This bumps Cargo.lock
    VERSION=$(rg  "version = \"([0-9.]+)\"" -or '$1' Cargo.toml | head -n1) && \
        git commit -am "Bump version {{level}} to $VERSION" && \
        git tag v$VERSION && \
        git push origin v$VERSION
    git push

publish:
    cargo publish

patch: test
    just version patch
    just publish
