.PHONY: lint fmt fmt_check test icon dev build

build:
	make -C core_ffi build
	flutter build apk --release --target-platform android-arm,android-arm64 --split-per-abi

lint:
	flutter analyze
	make -C core_ffi lint

dev:
	flutter run

fmt:
	flutter format lib/**
	make -C core_ffi fmt

fmt_check:
	flutter format -n lib/**

install:
	flutter pub get

test:
	- flutter test

icon:
	- flutter pub run flutter_launcher_icons:main
