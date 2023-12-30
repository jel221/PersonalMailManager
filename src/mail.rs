extern crate google_gmail1 as gmail1;
use gmail1::api::Message;
use gmail1::{Result, Error};
use std::fs;
use std::default::Default;
use gmail1::{Gmail, oauth2, hyper, hyper_rustls, chrono, FieldMask};
use std::path::Path;

pub fn login() -> Result<()> {    
	// Your Google Cloud Platform credentials file (JSON format)
    let credentials_file = Path::new("../credentials.json");

    // Create a new hyper-based transport
    let transport = Default::default().build().unwrap();

    // Create a Gmail hub
    let hub = Gmail::new(
        transport,
        hyper::Client::builder().build(Default::default()),
    );

    // Authenticate
    let mut key = oauth2::read_service_account_key(credentials_file)
        .await
        .expect("failed to read credentials");
    key.set_subject("user@example.com"); // Replace with the user's email address

    let token = key.token(&["https://www.googleapis.com/auth/gmail.readonly"])
        .await
        .expect("failed to obtain token");

    // Set the authorization token
    let hub = hub.set_auth_token(&token);

    // Example: List the user's messages
    let response = hub.users().messages_list("me").doit().await?;
    if let Some(messages) = response.1.messages {
        for message in messages {
            println!("Message ID: {}", message.id.unwrap_or_default());
        }
    } else {
        println!("No messages found.");
    }

    // Example: Get a specific message by ID
    if let Ok(message) = get_message(&hub, "message_id_here").await {
        println!("Message snippet: {}", message.snippet.unwrap_or_default());
    }
	Ok(())
}