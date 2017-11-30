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
                  (about: "This is a command line shell with helm-like search interface")
                  (@arg DEBUG_FILE: -D --debug +takes_value "Output debug info to file"))
        .get_matches();
    pub static ref LOGGER: Logger = match MATCHES.value_of("DEBUG_FILE") {
        Some(s) => FileLoggerBuilder::new(s).level(Severity::Debug).build(),
        None => NullLoggerBuilder{}.build(),
    }.ok().unwrap();
}

pub fn is_file_executable(fdata: &Metadata) -> bool {
    (fdata.mode() & 0o111) != 0
}
