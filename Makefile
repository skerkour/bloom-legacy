.PHONY: dev build release

build:
	mkdocs build

dev:
	mkdocs serve

release:
	git checkout master
	git merge dev
	git push
	git checkout dev
