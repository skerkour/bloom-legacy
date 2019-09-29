.PHONY: lint fmt fmt_check test icon dev

lint:
	flutter analyze

dev:
	flutter run

fmt:
	flutter format lib/**

fmt_check:
	flutter format -n lib/**

install:
	flutter pub get

test:
	- flutter test

icon:
	- flutter pub run flutter_launcher_icons:main
