NAME := bloomcore
# VERSION := $(shell cat Cargo.toml | grep '^version =' | cut -d '"' -f2)
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")


.PHONY: all
all: desktop android

.PHONY: fmt
fmt:
	go fmt ./...

.PHONY: test
test:
	go test ./...

.PHONY: desktop
desktop:
	go build -o dist/$(NAME)server server/*.go
	cp dist/$(NAME)server ../desktop/$(NAME)server

.PHONY: dev_desktop
dev_desktop:
	gowatch -build="make desktop"


.PHONY: mobile
mobile: android

PHONY: android
android: android_armv8 android_armv7 android_x86 # android_amd64

PHONY: android_armv8
android_armv8:
	GOOS=android GOARCH=arm64 CGO_ENABLED=1 CC=aarch64-linux-android28-clang go build --tags "android" -o dist/android/armv8/lib$(NAME).so -buildmode=c-shared ffi/*.go
	cp dist/android/armv8/lib$(NAME).so ../mobile/android/app/src/main/jniLibs/arm64-v8a/lib$(NAME).so

PHONY: android_armv7
android_armv7:
	GOOS=android GOARM=7 GOARCH=arm CGO_ENABLED=1 CC=arm-linux-androideabi-clang go build --tags "android" -o dist/android/armv7/lib$(NAME).so -buildmode=c-shared ffi/*.go
	cp dist/android/armv7/lib$(NAME).so ../mobile/android/app/src/main/jniLibs/armeabi-v7a/lib$(NAME).so

PHONY: android_x86
android_x86:
	GOOS=android GOARCH=386 CGO_ENABLED=1 CC=i686-linux-android18-clang go build --tags "android" -o dist/android/x86/lib$(NAME).so -buildmode=c-shared ffi/*.go
	cp dist/android/x86/lib$(NAME).so ../mobile/android/app/src/main/jniLibs/x86/lib$(NAME).so

# PHONY: android_amd64
# android_amd64:
# 	GOOS=android GOARCH=amd64 CGO_ENABLED=1 CC=x86_64-linux-android-clang go build --tags "android" -o dist/amd64/lib$(NAME).so -buildmode=c-shared ffi/*.go

.PHONY: clean
clean:
	rm -rf dist

.PHONY: tidy
tidy:
	go mod tidy
