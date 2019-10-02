.PHONY: build build_debug

all: build

build:
	cargo build --target i686-linux-android --release
	cp target/i686-linux-android/release/libcore_ffi.so ../android/app/src/main/jniLibs/x86/libcore_ffi.so

build_debug:
	cargo build --target i686-linux-android
