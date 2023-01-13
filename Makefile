ios:
	cargo lipo --release --targets aarch64-apple-ios -p leaf-ffi
	cbindgen --config leaf-ffi/cbindgen.toml leaf-ffi/src/lib.rs > target/universal/release/leaf.h

ios-dev:
	cargo lipo --targets aarch64-apple-ios -p leaf-ffi
	cbindgen --config leaf-ffi/cbindgen.toml leaf-ffi/src/lib.rs > target/universal/debug/leaf.h

ios-sim:
	cargo lipo --targets aarch64-apple-ios-sim -p leaf-ffi
	cbindgen --config leaf-ffi/cbindgen.toml leaf-ffi/src/lib.rs > target/universal/debug/leaf.h

ios-opt:
	cargo lipo --release --targets aarch64-apple-ios --manifest-path leaf-ffi/Cargo.toml --no-default-features --features "default-openssl"
	cbindgen --config leaf-ffi/cbindgen.toml leaf-ffi/src/lib.rs > target/universal/release/leaf.h

android:
	cross build --target aarch64-linux-android --release --manifest-path leaf-jni/Cargo.toml

android-dev:
	cross build --target aarch64-linux-android --manifest-path leaf-jni/Cargo.toml

lib:
	cargo build -p leaf-ffi --release
	cbindgen --config leaf-ffi/cbindgen.toml leaf-ffi/src/lib.rs > target/release/leaf.h

lib-dev:
	cargo build -p leaf-ffi
	cbindgen --config leaf-ffi/cbindgen.toml leaf-ffi/src/lib.rs > target/debug/leaf.h

local:
	cargo build -p leaf-bin --release

local-dev:
	cargo build -p leaf-bin

mipsel:
	./misc/build_cross.sh mipsel-unknown-linux-musl

mips:
	./misc/build_cross.sh mips-unknown-linux-musl

test:
	cargo test -p leaf -- --nocapture

# Force a re-generation of protobuf files.
proto-gen:
	touch leaf/build.rs
	PROTO_GEN=1 cargo build -p leaf
