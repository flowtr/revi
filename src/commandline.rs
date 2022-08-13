// TODO: Move this file into main.
use clap::{values_t, App, Arg};

pub fn args() -> Vec<String> {
	let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("files").multiple(true))
        .after_help("Pass in any number of files to ReVi to be placed in the Buffer list.")
        .get_matches();

	values_t!(matches, "files", String).unwrap_or_default()
}
