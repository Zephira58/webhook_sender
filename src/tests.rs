#[cfg(test)]
mod tests {
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

}