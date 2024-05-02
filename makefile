gen:
	bash scripts/gen.sh

gen-go:
	bash scripts/gen.sh go

gen-python:
	bash scripts/gen.sh python

gen-rust:
	bash scripts/gen.sh rust 

git-hooks:
	git config core.hooksPath .githooks

sync:
	bash scripts/sync_protos.sh
