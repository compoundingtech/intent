use std::process::ExitCode;

// The binary is a thin shell over the library entry point on purpose: `axe vrs`
// calls `intent::run` directly, so anything that lived here would be behavior the
// embedded caller silently does not get.
//
// Parsing goes through the same `Defaults` that resolve the arguments afterwards, so
// what `--help` advertises is what the run will actually use. Building the command
// tree separately from the defaults is what let the two disagree in the first place.
fn main() -> ExitCode {
    let defaults = intent::Defaults::default();
    intent::run_with(defaults.parse(), &defaults)
}
