use clap::ArgMatches;
use crate::mail;

pub async fn execute(matches: ArgMatches<'_>) {
	let aggr : Vec<_> = match matches.values_of("aggregate") {
		Some(a) => {a.collect()}
		None => vec![]
	};
	let dump = matches.is_present("dump");
	match matches.value_of("operation") {
		Some("get") => { mail::get(aggr, dump).await; }
		Some("delete")=> { mail::delete(aggr, dump).await; }
		Some(o) => { println!("Unexpected arguments: (get | delete), instead got {}", o) }
		None => { println!("Unexpected error") }
	}
}