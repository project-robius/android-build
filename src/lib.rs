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
mod env_paths;

pub use java_build::*;
pub use env_paths::*;


/*/
fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "android" {
        println!("cargo:rerun-if-changed=AuthenticationCallback.java");

        let out_dir = env::var("OUT_DIR").unwrap();
        let java_file = format!(
            "{}/AuthenticationCallback.java",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        );

        let android_sdk_home = env::var("ANDROID_HOME")
            .or_else(|_| env::var("ANDROID_SDK_ROOT"))
            .expect("ANDROID_HOME or ANDROID_SDK_ROOT must be set");
        let android_sdk_version = env::var("ANDROID_SDK_VERSION");
        let android_api_level = env::var("ANDROID_API_LEVEL");

        // Try to find the d8 jar using an env var or from the SDK root directory.
        let d8_jar_path = env::var("ANDROID_D8_JAR").unwrap_or_else(|_| {
            format!(
                "{android_sdk_home}/build-tools/{}/lib/d8.jar",
                android_sdk_version
                    .expect("ANDROID_SDK_VERSION must be set if ANDROID_D8_JAR is not set")
            )
        });

        // Try to find the android JAR using an env var or from the SDK root directory.
        let android_jar_path = env::var("ANDROID_JAR").unwrap_or_else(|_| {
            format!(
                "{android_sdk_home}/platforms/{}/android.jar",
                android_api_level.expect("ANDROID_API_LEVEL must be set if ANDROID_JAR is not set"),
            )
        });

        // Try to find `javac` in "JAVA_HOME/bin/", otherwise use the `javac` in the
        // current path.
        let javac_path = env::var("JAVA_HOME")
            .map(|java_home| format!("{}/bin/javac", java_home))
            .unwrap_or_else(|_| "javac".to_owned());

        // Try to find `java` in "JAVA_HOME/bin/", otherwise use the `java` in the
        // current path.
        let java_path = env::var("JAVA_HOME")
            .map(|java_home| format!("{}/bin/java", java_home))
            .unwrap_or_else(|_| "java".to_owned());

        // Compile the .java file into a .class file.
        assert!(
            Command::new(javac_path)
                .args(["-cp", &android_jar_path, &java_file, "-d", &out_dir])
                .output()
                .unwrap()
                .status
                .success(),
            "javac invocation failed"
        );

        let class_file = format!("{out_dir}/robius/authentication/AuthenticationCallback.class");

        // Compile the .class file into a .dex file.
        assert!(
            Command::new(java_path)
                .args([
                    "-cp",
                    &d8_jar_path,
                    "com.android.tools.r8.D8",
                    "--classpath",
                    &android_jar_path,
                    "--output",
                    &out_dir,
                    &class_file,
                ])
                .output()
                .unwrap()
                .status
                .success(),
            "java d8.jar invocation failed"
        );
    }
}
*/