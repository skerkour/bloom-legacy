.PHONY: build build_debug clean fmt lint

all: build

clean:
	cargo clean

build:
	cargo build --target i686-linux-android --release
	i686-linux-android-strip -s target/i686-linux-android/release/libcore_ffi.so
	cp target/i686-linux-android/release/libcore_ffi.so ../android/app/src/main/jniLibs/x86/libcore_ffi.so

build_debug:
	cargo build --target i686-linux-android
	cp target/i686-linux-android/debug/libcore_ffi.so ../android/app/src/main/jniLibs/x86/libcore_ffi.so
# OPENSSL_LIB_DIR=/usr/local/opt/openssl/lib OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/includ
# e  OPENSSL_STATIC=1 make build_debug

dev:
	cargo watch -s 'make build_debug'

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings -A clippy::needless_return
