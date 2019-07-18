.PHONY: lint fmt fmt_check test

lint:
	flutter analyze

fmt:
	flutter format lib/**

fmt_check:
	flutter format -n lib/**

test:
	- flutter test
