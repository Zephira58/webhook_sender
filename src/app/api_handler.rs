#![allow(clippy::all)]
#![allow(unused_must_use)]
use self_update::cargo_crate_version;
use std::collections::HashMap;

pub fn get_insult() -> String {
    println!("Fetching insult from https://insult.mattbas.org/api/insult...");
    //View more details on how this works here (https://www.reddit.com/r/learnrust/comments/wz9flc/how_to_get_request_to_return_as_a_string/)
    let x = reqwest::blocking::get("https://insult.mattbas.org/api/insult")
        .expect("Get failed")
        .text()
        .expect("Couldn't get response body");
    println!("Insult fetched!\n");
    return x;
}

pub fn get_affirmation() -> String {
    println!("Fetching affirmation from https://www.affirmations.dev/...");
    let x = reqwest::blocking::get("https://www.affirmations.dev/")
        .expect("Get failed")
        .text()
        .expect("Couldn't get response body")
        .replace("{", "")
        .replace("}", "")
        .replace(":", "")
        .replace("affirmation", "")
        .replace('"', "");
    println!("Affirmation fetched!\n");
    return x;
}

#[tokio::main]
pub async fn send_message(msg: &str, webhook: &str, username: &str, avatar_url: &str) {
    println!("Sending message...");
    let message = msg.to_owned();
    let message = message.as_str();

    let mut request_body = HashMap::new();
    request_body.insert("content", message);
    request_body.insert("username", username);
    request_body.insert("avatar_url", avatar_url);
    println!("Message sent!\n");

    reqwest::Client::new()
        .post(webhook)
        .json(&request_body)
        .send()
        .await;
}

pub fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("Xanthus58")
        .repo_name("webhook_sender")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!\n", status.version());
    Ok(())
}
