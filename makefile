SHELL := /bin/zsh

gen:
	bash scripts/gen.sh

gen-go:
	bash scripts/gen.sh go

gen-python:
	bash scripts/gen.sh python

gen-rust:
	bash scripts/gen.sh rust 

sanitize:
	echo "Error: sanitize has been removed from this repo. Please see Confluence on proto updating."
	exit 1

git-hooks:
	git config core.hooksPath .githooks
