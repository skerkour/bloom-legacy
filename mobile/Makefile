.PHONY: lint fmt fmt_check test icons dev build assets

build:
	flutter build apk --release --target-platform android-arm,android-arm64 --split-per-abi

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

icons:
	- flutter pub run flutter_launcher_icons:main

assets:
	rm -rf assets
	mkdir -p assets
	cp -r ../assets/icons assets/icons
	cp -r ../assets/mobile/images assets/images
