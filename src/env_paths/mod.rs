// so I think the only real instructive cmake files within the NDK is `cmake/android-legacy.toolchain.cmake` which names various cmake-level env vars like so:
// * `ANDROID_PLATFORM`, has a value like "android-33" (not sure how "-ext4" suffixes are represented, which we do need)
// * `ANDROID_PLATFORM_LEVEL`, has a value like "33"
// * `ANDROID_NATIVE_API_LEVEL`, which seems to be an alternate for the above two, and has a value like "android-33"
// * `ANDROID_TOOLCHAIN_NAME`, has a value like "aarch64-linux-android-"
// * `ANDROID_ABI`, has a value like "arm64-v8a" (don't think we need thisi)
// 
// 
// and then for NDK stuff, there's:
// * `ANDROID_NDK`, already widely known and standardized. 
// * `ANDROID_NDK_MAJOR`, MINOR, BUILD, BETA, which have possible formats:
//   *  r16, build 1234: 16.0.1234
//   * r16b, build 1234: 16.1.1234
//   * r16 beta 1, build 1234: 16.0.1234-beta1
//
//
// ok, so my takeaways from digging into this is that we need to change some of our exports:
// * ANDROID_API_LEVEL, e.g., either "android-33" or just "33", aka ANDROID_PLATFORM
// * a build tools version, e.g., "33.0.1"
// * an SDK Extension number, which is an *optional* integer value like "4" that gets appended to the android api level as "android-33-ext4" to form a full platform string.
//   * here we could accept either "ext4", "-ext4", or just "4", for ultimate flexibility.
//
//
// more env vars are documented here: https://github.com/taka-no-me/android-cmake
//

use std::{env, path::{Path, PathBuf}};
use self::find_java::find_java_home;

mod find_android_sdk;
mod find_java;

pub const ANDROID_HOME: &str = "ANDROID_HOME";
pub const ANDROID_SDK_ROOT: &str = "ANDROID_SDK_ROOT";
pub const ANDROID_SDK_VERSION: &str = "ANDROID_SDK_VERSION";
pub const ANDROID_API_LEVEL: &str = "ANDROID_API_LEVEL";
pub const ANDROID_D8_JAR: &str = "ANDROID_D8_JAR";
pub const ANDROID_JAR: &str = "ANDROID_JAR";
pub const JAVA_HOME: &str = "JAVA_HOME";

/// An exentions trait for checking if a path exists.
pub trait PathExt {
    fn path_if_exists(self) -> Option<Self> where Self: Sized;
}
impl<P: AsRef<Path>> PathExt for P {
    fn path_if_exists(self) -> Option<P> {
        if self.as_ref().as_os_str().is_empty() {
            return None;
        }
        self.as_ref().exists().then_some(self)
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

pub fn android_jar(api_level: Option<&str>) -> Option<PathBuf> {
    env::var(ANDROID_JAR).ok()
        .and_then(PathExt::path_if_exists)
        .map(PathBuf::from)
        .or_else(|| android_sdk()
            .and_then(|sdk| sdk
                .join("platforms")
                .join(api_level.map(ToString::to_string)
                    .unwrap_or_else(|| env::var(ANDROID_API_LEVEL)
                        .expect("either ANDROID_JAR or ANDROID_API_LEVEL must be set")
                    )
                )
                .join("android.jar")
                .path_if_exists()
            )
        )
}

pub fn android_d8_jar(build_tools_version: Option<&str>) -> Option<PathBuf> {
    env::var(ANDROID_D8_JAR).ok()
        .and_then(PathExt::path_if_exists)
        .map(PathBuf::from)
        .or_else(|| android_sdk()
            .and_then(|sdk| sdk
                .join("build-tools")
                .join(build_tools_version.map(ToString::to_string)
                    .unwrap_or_else(|| env::var(ANDROID_SDK_VERSION)
                        .expect("either ANDROID_D8_JAR or ANDROID_SDK_VERSION must be set")
                    )
                )
                .join("lib")
                .join("d8.jar")
                .path_if_exists()
            )
        )
}

pub fn java() -> Option<PathBuf> {
    java_home().and_then(|jh| jh
        .join("bin")
        .join("java")
        .path_if_exists()
    )
}

pub fn javac() -> Option<PathBuf> {
    java_home().and_then(|jh| jh
        .join("bin")
        .join("javac")
        .path_if_exists()
    )
}

pub fn java_home() -> Option<PathBuf> {
    env::var(JAVA_HOME).ok()
        .and_then(PathExt::path_if_exists)
        .map(PathBuf::from)
        .or_else(find_java_home)
}

