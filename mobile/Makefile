.PHONY: lint fmt fmt_check test icon dev build assets

build:
	# make -C core_ffi build # commented because currently the mobile CICD job does not have a Rust toolchain installed
	flutter build apk --release --target-platform android-arm,android-arm64 --split-per-abi

lint:
	# make -C core_ffi lint # commented because currently the mobile CICD job does not have a Rust toolchain installed
	flutter analyze

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

assets:
	rm -rf assets/icons && cp -r ../assets/icons assets/icons
