.PHONY: all
all: build

.PHONY: build
build:
	flutter build apk --release --target-platform android-arm,android-arm64 --split-per-abi

.PHONY: clean
clean:
	flutter clean

.PHONY: lint
lint:
	flutter analyze

.PHONY: dev
dev:
	flutter run

.PHONY: fmt
fmt:
	flutter format lib/**

.PHONY: fmt_check
fmt_check:
	flutter format -n lib/**

.PHONY: install
install:
	flutter pub get

.PHONY: test
test:
	- flutter test

.PHONY: icons
icons:
	- flutter pub run flutter_launcher_icons:main

.PHONY: assets
assets:
	rm -rf assets
	mkdir -p assets
	cp -r ../assets/icons assets/icons
	cp -r ../assets/mobile/images assets/images
