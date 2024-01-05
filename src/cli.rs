extern crate clap;
use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("mail-manager")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
		.arg(
			Arg::with_name("operation")
			.short("op")
			.help("operation: get | delete")
			.takes_value(true)
			.required(true)
			)
		.arg(
			Arg::with_name("aggregate")
			.short("a")
			.help("aggregator: (sender | date) filter")
			.takes_value(true)
			.required(false)
		)
		.arg(
			Arg::with_name("filter")
			.short("f")
			.help("filter")
			.takes_value(true)
			.required(false)
		)
		.arg(
			Arg::with_name("dump")
			.short("d")
			.help("dump: output everything at once")
			.takes_value(false)
			.required(false)
		)

}