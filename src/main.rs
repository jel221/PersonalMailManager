mod cli;
mod matcher;
mod mail;

fn main() {
	let matches = cli::build_cli().get_matches();
	mail::login().expect("Login failed");
	matcher::execute(matches);
}