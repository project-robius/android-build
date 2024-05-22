# android-build

Use this crate from your Cargo `build.rs` build script to compile Java source files and to run Java commands as part of a Rust build.

It is specifically designed with Android in mind, to help build Java files targeted for Android and to run Android-specific Java commands.

This crate aims to behave similarly to [`cc-rs`](https://github.com/rust-lang/cc-rs/tree/main), but for Java (primarily on Android) instead of C/C++.

## Usage
Add this crate as a build dependency to your `Cargo.toml`:
```toml
[build-dependencies]
android-build = "0.1.0"
```

Then add this to your `build.rs` build script:
```rust
fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "android" {
        let output_dir = std::env::var("OUT_DIR").unwrap();
        let android_jar_path = android_build::android_jar(None)
            .expect("Failed to find android.jar");

        android_build::JavaBuild::new()
            .class_path(android_jar_path)
            .classes_out_dir(std::path::PathBuf::from(output_dir))
            .file("YourJavaFile.java")
            .compile()
            .expect("java build failed!");

        // `YourJavaFile.class` will be the Cargo-specified OUT_DIR.
    }
}
```

This code will automatically run when you build your crate using `cargo build`.

## Examples
Check out the [`robius-authentication` build script](https://github.com/project-robius/robius-authentication/blob/main/build.rs) to see how we use this crate for more complicated build steps:
* Compiling Java classes against the main `android.jar` jarfile.
* Invoking Android's `d8` DEXer tool.
