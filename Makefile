.PHONY: lint fmt fmt_check

lint:
	flutter analyze

fmt:
	flutter format lib/**

fmt_check:
	flutter format -n lib/**
