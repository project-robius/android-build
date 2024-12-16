use std::{env, path::{Path, PathBuf}};
use self::find_java::find_java_home;

mod find_android_sdk;
mod find_java;


pub const ANDROID_HOME:                 &str = "ANDROID_HOME";
pub const ANDROID_SDK_ROOT:             &str = "ANDROID_SDK_ROOT";
pub const ANDROID_BUILD_TOOLS_VERSION:  &str = "ANDROID_BUILD_TOOLS_VERSION";
pub const ANDROID_PLATFORM:             &str = "ANDROID_PLATFORM";
pub const ANDROID_SDK_VERSION:          &str = "ANDROID_SDK_VERSION";
pub const ANDROID_API_LEVEL:            &str = "ANDROID_API_LEVEL";
pub const ANDROID_SDK_EXTENSION:        &str = "ANDROID_SDK_EXTENSION";
pub const ANDROID_D8_JAR:               &str = "ANDROID_D8_JAR";
pub const ANDROID_JAR:                  &str = "ANDROID_JAR";
pub const JAVA_HOME:                    &str = "JAVA_HOME";
pub const ANDROID_SOURCE_VERSION:       &str = "ANDROID_SOURCE_VERSION";
pub const ANDROID_TARGET_VERSION:       &str = "ANDROID_TARGET_VERSION";

/// An extension trait for checking if a path exists.
pub trait PathExt {
    fn path_if_exists(self) -> Option<Self> where Self: Sized;
}
impl<P: AsRef<Path>> PathExt for P {
    fn path_if_exists(self) -> Option<P> {
        if self.as_ref().as_os_str().is_empty() {
            return None;
        }
        match self.as_ref().try_exists() {
            Ok(true) => Some(self),
            _ => None,
        }
    }
}


/// Returns the path to the Android SDK directory.
///
/// The path is determined by an ordered set of attempts:
/// * The `ANDROID_HOME` environment variable, if it is set and if the directory exists.
/// * The `ANDROID_SDK_HOME` environment variable, if it is set and if the directory exists.
/// * The default installation location for the Android SDK, if it exists.
///   * On Windows, this is `%LOCALAPPDATA%\Android\Sdk`.
///   * On macOS, this is `~/Library/Android/sdk`.
///   * On Linux, this is `~/Android/Sdk`.
#[doc(alias("ANDROID_HOME", "ANDROID_SDK_ROOT", "home", "sdk", "root"))]
pub fn android_sdk() -> Option<PathBuf> {
    env::var(ANDROID_HOME).ok()
        .and_then(PathExt::path_if_exists)
        .or_else(|| env::var(ANDROID_SDK_ROOT).ok()
            .and_then(PathExt::path_if_exists)
        )
        .map(PathBuf::from)
        .or_else(|| find_android_sdk::find_android_sdk().and_then(PathExt::path_if_exists))
}

/// Returns the path to the `android.jar` file for the given API level.
///
/// If the `ANDROID_JAR` environment variable is set and points to a file that exists,
/// that path is returned.
///
/// Otherwise, `platform_string` is used to find the `android.jar` file from the
/// `platforms` subdirectory in the Android SDK root directory.
///
/// If `platform_string` is `None`, the value of the following environment variables
/// are used to calculate the platform string:
/// * `ANDROID_PLATFORM`
/// * `ANDROID_API_LEVEL`
/// * `ANDROID_SDK_VERSION`
/// * `ANDROID_SDK_EXTENSION`
pub fn android_jar(platform_string: Option<&str>) -> Option<PathBuf> {
    env::var(ANDROID_JAR).ok()
        .and_then(PathExt::path_if_exists)
        .map(PathBuf::from)
        .or_else(|| android_sdk()
            .and_then(|sdk| sdk
                .join("platforms")
                .join(platform_string.map(ToString::to_string)
                    .unwrap_or_else(|| env_android_platform_api_level()
                        .expect("either ANDROID_JAR or [ANDROID_PLATFORM, ANDROID_API_LEVEL, ANDROID_SDK_VERSION] must be set")
                    )
                )
                .join("android.jar")
                .path_if_exists()
            )
        )
}

/// Returns the path to the `d8.jar` file for the given build tools version.
///
/// If the `ANDROID_D8_JAR` environment variable is set and points to a file that exists,
/// that path is returned.
/// If `build_tools_version`is `None`, the value of the `ANDROID_BUILD_TOOLS_VERSION` environment variable is used
/// to find the `d8.jar` file from the Android SDK root directory.
pub fn android_d8_jar(build_tools_version: Option<&str>) -> Option<PathBuf> {
    env::var(ANDROID_D8_JAR).ok()
        .and_then(PathExt::path_if_exists)
        .map(PathBuf::from)
        .or_else(|| android_sdk()
            .and_then(|sdk| sdk
                .join("build-tools")
                .join(build_tools_version.map(ToString::to_string)
                    .unwrap_or_else(|| env::var(ANDROID_BUILD_TOOLS_VERSION)
                        .expect("either ANDROID_D8_JAR or ANDROID_BUILD_TOOLS_VERSION must be set")
                    )
                )
                .join("lib")
                .join("d8.jar")
                .path_if_exists()
            )
        )
}

/// Returns the platform version string (aka API level, SDK version) being targeted for compilation.
///
/// This deals with environment variables `ANDROID_PLATFORM`, `ANDROID_API_LEVEL`, and `ANDROID_SDK_VERSION`,
/// as well as the optional `ANDROID_SDK_EXTENSION`.
fn env_android_platform_api_level() -> Option<String> {
    let mut base = env::var(ANDROID_PLATFORM).ok()
        .or_else(|| env::var(ANDROID_API_LEVEL).ok())
        .or_else(|| env::var(ANDROID_SDK_VERSION).ok())?;
    
    if base.is_empty() {
        return None;
    }

    if !base.starts_with("android-") {
        base = format!("android-{}", base);
    }

    if base.contains("-ext") {
        return Some(base);
    }

    if let Some(raw_ext) = env::var(ANDROID_SDK_EXTENSION).ok() {
        let ext_num = raw_ext
            .trim_start_matches("-")
            .trim_start_matches("ext");
        if !ext_num.is_empty() {
            base = format!("{}-ext{}", base, ext_num);
        }
    }
    
    Some(base)
}

/// Returns the path to the `java` executable by looking for `$JAVA_HOME/bin/java`.
pub fn java() -> Option<PathBuf> {
    java_home().and_then(|jh| jh
        .join("bin")
        .join("java")
        .path_if_exists()
    )
}

/// Returns the path to the `javac` compiler by looking for `$JAVA_HOME/bin/javac`.
pub fn javac() -> Option<PathBuf> {
    java_home().and_then(|jh| jh
        .join("bin")
        .join("javac")
        .path_if_exists()
    )
}

/// Returns the JAVA_HOME path by attempting to discover it.
/// 
/// First, if the `$JAVA_HOME` environment variable is set and points to a directory that exists,
/// that path is returned.
/// Otherwise, a series of common installation locations is used,
/// based on the current platform (macOS, Linux, Windows).
pub fn java_home() -> Option<PathBuf> {
    env::var(JAVA_HOME).ok()
        .and_then(PathExt::path_if_exists)
        .map(PathBuf::from)
        .or_else(find_java_home)
}

/// Returns the source version for compilation
/// from `ANDROID_SOURCE_VERSION`,
pub fn android_source_version() -> Option<String> {
    env::var(ANDROID_SOURCE_VERSION).ok()
}

/// Returns the target version for compilation
/// from `ANDROID_TARGET_VERSION`,
pub fn android_target_version() -> Option<String> {
    env::var(ANDROID_TARGET_VERSION).ok()
}
