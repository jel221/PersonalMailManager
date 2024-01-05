mod cli;
mod matcher;
mod mail;

#[tokio::main]
async fn main() {
	let matches = cli::build_cli().get_matches();
	matcher::execute(matches).await;
}