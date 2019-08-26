.PHONY: dev build release

build:
	mdbook build

dev:
	mdbook serve

release:
	git checkout master
	git merge dev
	git push
	make release
	git checkout dev
