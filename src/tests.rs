#[cfg(test)]
mod tests {
    use self_update::cargo_crate_version;
    use std::collections::HashMap;

    #[test]
    fn test_get_insult() {
        println!("\nFetching insult from https://insult.mattbas.org/api/insult...");
        //View more details on how this works here (https://www.reddit.com/r/learnrust/comments/wz9flc/how_to_get_request_to_return_as_a_string/)
        let x = reqwest::blocking::get("https://insult.mattbas.org/api/insult")
            .expect("Get failed")
            .text()
            .expect("Couldn't get response body");
        println!("Insult fetched!");
        assert_eq!(x, x);
    }

    #[test]
    fn test_get_affirmation() {
        println!("\nFetching affirmation from https://www.affirmations.dev/...");
        let x = reqwest::blocking::get("https://www.affirmations.dev/")
            .expect("Get failed")
            .text()
            .expect("Couldn't get response body")
            .replace(['{', '}', ':'], "")
            .replace("affirmation", "")
            .replace('"', "");
        println!("Affirmation fetched!");
        assert_eq!(x, x);
    }

    #[test]
    fn test_send_message() {
        println!("\nSending message...");
        let mut request_body = HashMap::new();
        request_body.insert("content", "unit test");
        request_body.insert("username", "xanthus");
        request_body.insert("avatar_url", "https://cdn.discordapp.com/avatars/292971545956188160/2a77b119a3f8ccedfbd513825eec97a5.png?size=1024");

        reqwest::Client::new()
            .post("https://discord.com/api/webhooks/1071132108392775720/uYFy-Iyxfi8pO98_PNF3f8fgATHNKzYkSDJ1c4S9_5lvB-axPyTQlF5iHl29cW-6JyPO")
            .json(&request_body)
            .send();
        println!("Message sent!");
    }

    #[test]
    pub fn download_update() -> Result<(), Box<dyn (::std::error::Error)>> {
        let status = self_update::backends::github::Update::configure()
            .repo_owner("Xanthus58")
            .repo_name("webhook_sender")
            .bin_name("github")
            .show_download_progress(true)
            .no_confirm(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;
        println!("Update status: `{}`!", status.version());
        Ok(())
    }
}
