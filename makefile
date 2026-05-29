SHELL := /bin/zsh

.PHONY: gen
gen:
	bash scripts/gen.sh

.PHONY: gen-go
gen-go:
	bash scripts/gen.sh go

.PHONY: gen-python
gen-python:
	bash scripts/gen.sh python

.PHONY: gen-rust
gen-rust:
	bash scripts/gen.sh rust 

.PHONY: sanitize
sanitize:
	echo "Error: sanitize has been removed from this repo. Please see Confluence on proto updating."
	exit 1

.PHONY: git-hooks
git-hooks:
	git config core.hooksPath .githooks

.PHONY: serve-cli-docs
serve-cli-docs:
	mdbook serve ./rust/crates/sift_cli/assets/docs -d ./rust/crates/sift_cli/assets/docs/book-dev

.PHONY: build-cli-docs
build-cli-docs:
	mdbook build ./rust/crates/sift_cli/assets/docs -d ./rust/crates/sift_cli/assets/docs/book-dev
