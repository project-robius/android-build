//! Supports Android-specific build tasks for Java code in Rust projects
//! as part of a Cargo build script.
//!
//! ## Tools exposed by this crate
//! * javac: use the [`Javac`] struct.
//! * d8: through the [`Dexer`] struct.
//!
//! ## Environment variables in use
//! * `ANDROID_HOME` or `ANDROID_SDK_ROOT`: the Android SDK directory.
//! * `ANDROID_SDK_VERSION`: the version of the Android SDK.
//! * `ANDROID_API_LEVEL`: the API level of the Android SDK.
//! * `ANDROID_D8_JAR`: the path to the `d8.jar` file.
//! * `ANDROID_JAR`: the path to the `android.jar` file.
//! * `JAVA_HOME`: the Java SDK directory.
//!
//! ## Acknowledgments
//! This crate simplifies some code found in other crates:
//! * [`dirs-sys`](https://github.com/dirs-dev/dirs-sys-rs/blob/c0fd66cb08f1f97ebf670914253a34bd42d284fb/src/lib.rs#L151)
//!   for the Windows-specific home directory lookup.
//! * [`java-locator`](https://github.com/astonbitecode/java-locator/)
//!   for finding the Java home directory on macOS, Linux, and Windows.
//! * [`jerk`](https://github.com/MaulingMonkey/jerk)
//!   for arguments that can be passed into `java` and `javac` commands.
//!


mod java_build;
mod java_run;
mod env_paths;

pub use java_build::*;
pub use java_run::*;
pub use env_paths::*;
