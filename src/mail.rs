extern crate google_gmail1 as gmail1;
use gmail1::api::Message;
use gmail1::hyper::client::HttpConnector;
use gmail1::hyper::client::connect::dns::GaiResolver;
use gmail1::hyper_rustls::HttpsConnector;
use gmail1::{Result, Error};
use gmail1::{Gmail, oauth2, hyper, hyper_rustls, chrono, FieldMask};
use std::path::Path;
use std::fs;

async fn login() -> Gmail<HttpsConnector<HttpConnector>> {    
	// Your Google Cloud Platform credentials file (JSON format)
    let credentials_file = Path::new("../../credentials.json");

	let secret: oauth2::ApplicationSecret = oauth2::read_application_secret(credentials_file)
		.await
		.expect("credentials file error");

	let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    	).build().await.unwrap();

	Gmail::new(
		hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()),
		auth
	)
}

pub async fn get(aggr: Vec<&str>, dump: bool) -> Result<()> {
	let mut hub = login().await;
	
	let email = fs::read_to_string("../../email.txt")
		.expect("Please put your email in email.txt");
	let mail_list = hub.users().messages_list(&email)
		.q("is:unread");


	Ok(())
}

pub async fn delete(aggr: Vec<&str>, _dump: bool) -> Result<()> {
	let mut hub = login().await;

	Ok(())
}

fn query_str(aggr: Vec<&str>) -> Result<&str> {
	Ok(&"lol")
}