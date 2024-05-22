//! Supports Android-specific Java build and run tasks in Rust projects from a Cargo build script.
//!
//! ## Tools exposed by this crate
//! * javac: use the [`JavaBuild`] struct.
//! * java: use the [`JavaRun`] struct.
// //! * d8: through the [`Dexer`] struct.
//!
//! ## Environment variables in use
//! * `ANDROID_HOME` or `ANDROID_SDK_ROOT`: path to the Android SDK directory.
//! * `ANDROID_BUILD_TOOLS_VERSION`: the version of the Android build tools.
//!   * Examples: `33.0.1`, `34.0.0-rc2`.
//!   * This must be fully specified all in one string.
//! * `ANDROID_PLATFORM`, `ANDROID_API_LEVEL`, or `ANDROID_SDK_VERSION`:
//!   the platform version string (aka API level, SDK version) being targeted for compilation.
//!   * All three of these environment variables are treated identically.
//!   * Examples: `34`, `android-34`, `android-33`, `33`.
//!   * If an SDK extension must be specified, use the full string with the `android` prefix
//!     like so: `android-33-ext4`.
//!   * This may or may not include the SDK extension level as a suffix
//!     (see `ANDROID_SDK_EXTENSION` below).
//! * `ANDROID_SDK_EXTENSION`: the extension of the Android SDK.
//!   * To specify `android-33-ext4`, this can be set to `-ext4`, `ext4`, or just `4`.
//!     All of these will be treated identically.
//!   * If `ANDROID_PLATFORM`/`ANDROID_API_LEVEL`/`ANDROID_SDK_VERSION`
//!     already includes an extension, this will be ignored.
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
