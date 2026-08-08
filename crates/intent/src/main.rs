use clap::Parser;
use std::process::ExitCode;

// The binary is a thin shell over the library entry point on purpose: `axe vrs`
// calls `intent::run` directly, so anything that lived here would be behavior the
// embedded caller silently does not get.
fn main() -> ExitCode {
    intent::run(intent::VrsCli::parse())
}
