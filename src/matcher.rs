use clap::ArgMatches;

pub fn execute(matches: ArgMatches) {
	let agg = matches.values_of("aggregator");
	match agg {
		{Some(a), None} =>
	}
	match (matches.value_of("operation"), matches.value_of("aggregate")) {
		(Some("get"), Some("sender")) => { println!("done") }
		(Some("get"), Some("date")) => { println!("done") }
		(Some("delete"), Some("sender")) => { println!("done") }
		(Some("delete"), Some("date")) => { println!("done") }
		(Some(o), None) => { println!("Unexpected arguments: (get | delete), instead got {}", o) }
		(Some(o), Some(a)) => { println!("Unexpected arguments: (get | delete), (sender | date), instead got {}, {}", o, a) }
		_ => { println!("Unexpected error") }
	}
}