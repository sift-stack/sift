gen:
	bash scripts/gen.sh

gen-go:
	bash scripts/gen.sh go

gen-python:
	bash scripts/gen.sh python

gen-rust:
	bash scripts/gen.sh rust 

remove-go-pkg:
	bash scripts/remove_go_pkg.sh

git-hooks:
	git config core.hooksPath .githooks
