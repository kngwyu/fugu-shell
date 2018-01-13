// Command Line arguments and debug file
use clap::ArgMatches;
use slog::Logger;
use sloggers::types::Severity;
use sloggers::Build;
use sloggers::file::FileLoggerBuilder;
use sloggers::null::NullLoggerBuilder;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
lazy_static!{
    pub static ref MATCHES: ArgMatches<'static> =
        clap_app!(fugu_shell =>
                  (version: "0.0.1")
                  (author: "kngwyu")
                  (about: "A command line shell with search interface inspired by helm")
                  (@arg DEBUG_FILE: -D --debug +takes_value "Output debug info to file")
                  (@arg DEBUG_LEVEL: -L --level +takes_value "Debug Level")
                  (@arg EXEC_FILE: -E --exec +takes_value "Exec command in")
                  (@arg SETTING_FILE: -S --setting +takes_value "Setting file")
        )
        .get_matches();
    pub static ref LOGGER: Logger = match MATCHES.value_of("DEBUG_FILE") {
        Some(s) => {
            let mut builder = FileLoggerBuilder::new(s);
            let level_str = MATCHES.value_of("DEBUG_LEVEL").unwrap_or("Critical");
            let level = match level_str {
                "1" | "Critical" | "critical" => Severity::Critical,
                "2" | "Error" | "error" => Severity::Error,
                "3" | "Warning" | "warning" => Severity::Warning,
                "4" | "Info" | "info" => Severity::Info,
                "5" | "Debug" | "debug" => Severity::Debug,
                "6" | "Trace" | "trace" => Severity::Trace,
                _ => Severity::Warning,
            };
            builder.level(level);
            builder.truncate();
            builder.build()
        }
        None => NullLoggerBuilder{}.build(),
    }.ok().unwrap();
}

pub fn is_file_executable(fdata: &Metadata) -> bool {
    (fdata.mode() & 0o111) != 0
}

#[macro_export]
macro_rules! ok_or_continue {
    ($val:expr) => (
        if let Ok(v) = $val {
            v
        } else {
            continue;
        }
    )
}

#[macro_export]
macro_rules! some_or_continue {
    ($val:expr) => (
        if let Some(v) = $val {
            v
        } else {
            continue;
        }
    )
}
