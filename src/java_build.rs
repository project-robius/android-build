//! 

use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use crate::env_paths::{self, PathExt};

/// A builder for a `javac` command that can be invoked.
///
/// If you need to customize the `javac` command beyond what is provided here,
/// you can use the [`JavaBuild::command()`] method to get [`Command`]
/// that can be further customized with additional arguments.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JavaBuild {
    /// Override the default `JAVA_HOME` path.
    /// Otherwise, the default path is found using the `JAVA_HOME` env var.
    pub java_home:                  Option<PathBuf>,
    /// Debug info to include in the output ("-g" flag).
    pub debug_info:                 Option<DebugInfo>,
    /// If `true`, all warnings are disabled.
    pub nowarn:                     bool,
    /// Enable verbose output.
    pub verbose:                    bool,
    /// If `true`, show full descriptions of all places where
    /// deprecated members/classes are used or overridden.
    /// If `false`,  is to show only a summary on a per-source file basis).
    pub deprecation:                bool,
    /// If `true`, enable preview language features.
    pub enable_preview_features:    bool,
    /// Specify where to find user class files and annotation processors.
    /// If not provided, the current directory will be used.
    pub class_paths:                Vec<PathBuf>,
    /// Specify where to find input source files.
    /// If not specified, `class_paths` will be searched for source files.
    pub source_paths:               Vec<PathBuf>,
    /// Override the location of bootstrap class files.
    pub boot_class_paths:           Vec<PathBuf>,
    /// Override the location of installed extensions.
    pub extension_dirs:             Vec<PathBuf>,
    /// Override the location of endorsed standards path.
    pub endorsed_dirs:              Vec<PathBuf>,
    /// Specify names of the annotation processors to run.
    /// Setting this will bypass the default discovery process.
    pub annotation_processors:      Vec<String>,
    /// Specify where to find annotation processors.
    /// If not provided, the `class_paths` will be searched.
    pub annotation_processor_paths: Vec<PathBuf>,
    /// Enable generation of metadata on method parameters
    /// such that the reflection API can be used to retrieve parameter info.
    pub method_paramater_metadata:       bool,
    /// Specify where to place generated class files.
    /// If not provided, class files will be placed
    /// in the same directory as the source files.
    #[doc(alias("-d"))]
    pub classes_out_dir:                Option<PathBuf>,
    /// Specify where to place generated source files.
    #[doc(alias("-s"))]
    pub sources_out_dir:                Option<PathBuf>,
    /// Specify where to place generated native header files.
    #[doc(alias("-h"))]
    pub headers_out_dir:                Option<PathBuf>,

    /// Valid key-value pairs include:
    /// * "-implicit"
    /// * "-encoding"
    /// * "-source", "<release>"
    /// * "-target", "<release>"
    /// * "-profile", "<profile>"
    /// * "-version"
    /// * "-help"
    #[doc(alias("-A"))]
    pub annotation_parameters:      Vec<(String, String)>,

    /// If `true`, warnings are treated as compilation errors.
    pub warnings_as_errors:         bool,

    /// Paths to the java source files to be compiled.
    pub files:                      Vec<PathBuf>,
}

/// Debug information to include in the output.
///
/// The default value for this struct is for everything to be `true`,
/// meaning all debug information is included.
/// This is only relevant *if* you set the `debug_info` field in [`JavaBuild`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugInfo {
    pub line_numbers: bool,
    pub variables: bool,
    pub source_files: bool,
}
impl Default for DebugInfo {
    fn default() -> Self {
        Self {
            line_numbers: true,
            variables: true,
            source_files: true,
        }
    }
}
impl DebugInfo {
    fn add_as_args_to<'c>(&self, cmd: &'c mut Command) -> &'c mut Command {
        if self.line_numbers {
            cmd.arg("-g:lines");
        }
        if self.variables {
            cmd.arg("-g:vars");
        }
        if self.source_files {
            cmd.arg("-g:source");
        }
        if !self.line_numbers && !self.variables && !self.source_files {
            cmd.arg("-g:none");
        }
        cmd
    }
}

impl JavaBuild {
    /// Creates a new `JavaBuild` instance with default values.
    pub fn new() -> Self {
        Default::default()
    }

    /// Executes the `javac` command based on this `JavaBuild` instance.
    pub fn compile(&self) -> std::io::Result<ExitStatus> {
        self.command()?
            .status()
    }

    /// Returns a [`Command`] based on this `JavaBuild` instance
    /// that can be inspected or customized before being executed.
    pub fn command(&self) -> std::io::Result<Command> {
        let jh_clone = self.java_home.clone();
        let java_home = jh_clone
            .and_then(PathExt::path_if_exists)
            .or_else(env_paths::java_home)
            .ok_or_else(|| std::io::Error::other(
                "JAVA_HOME not provided, and could not be auto-discovered."
            ))?;

        let mut cmd = Command::new(java_home.join("bin").join("javac"));
        if let Some(d) = self.debug_info.as_ref() {
            d.add_as_args_to(&mut cmd);
        }

        self.class_paths     .iter().for_each(|p| { cmd.arg("-cp").arg(p); });
        self.source_paths    .iter().for_each(|p| { cmd.arg("-sourcepath").arg(p); });
        self.boot_class_paths.iter().for_each(|p| { cmd.arg("-bootclasspath").arg(p); });
        self.extension_dirs  .iter().for_each(|p| { cmd.arg("-extdirs").arg(p); });

        let processors = self.annotation_processors.join(",");
        if processors.len() != 0 {
            cmd.arg("-processor").arg(processors); 
        }

        self.annotation_processor_paths.iter()
            .for_each(|p| { cmd.arg("-processorpath").arg(p); });

        for (flag, dir) in [
            ("-d", self.classes_out_dir.as_ref()),
            ("-s", self.sources_out_dir.as_ref()),
            ("-h", self.headers_out_dir.as_ref()),
        ].iter() {
            if let Some(dir) = dir {
                cmd.arg(flag).arg(dir);
            }
        }

        for (flag, cond) in [
            ("-nowarn",          self.nowarn),
            ("-verbose",         self.verbose),
            ("-deprecation",     self.deprecation),
            ("-parameters",      self.method_paramater_metadata),
            ("-Werror",          self.warnings_as_errors),
            ("--enable-preview", self.enable_preview_features)
        ].into_iter() {
            if cond { cmd.arg(flag); }
        }

        self.annotation_parameters.iter()
            .for_each(|(k,v)| { cmd.arg(format!("-A{}={}", k, v)); });
        self.files.iter().for_each(|f| { cmd.arg(f); });

        Ok(cmd)
    }
}
