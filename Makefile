.PHONY: lint fmt fmt_check test icon

lint:
	flutter analyze

fmt:
	flutter format lib/**

fmt_check:
	flutter format -n lib/**

test:
	- flutter test

icon:
	- flutter pub run flutter_launcher_icons:main
