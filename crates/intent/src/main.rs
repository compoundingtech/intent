use clap::Parser;
use std::process::ExitCode;

// The binary is a thin shell over the library entry point so standalone and
// embedded callers execute the same checker behavior.
fn main() -> ExitCode {
    intent::run(intent::IntentCli::parse())
}
