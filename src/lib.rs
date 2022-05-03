/// It's a type alias for the Result type.
pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

/// It makes a GET request to the Kanye REST API, parses the JSON response, and returns the quote as a
/// string
/// 
/// Arguments:
/// 
/// * `client`: &reqwest::Client - This is the client that we created earlier.
/// 
/// Returns:
/// 
/// A Result<String>
async fn get_kuote(client: &reqwest::Client) -> Result<String> {
    let response = client
        // Making a GET request to the Kanye REST API.
        .get("https://api.kanye.rest/")
        .send()
        .await?

        // Parsing the JSON response into a serde_json::Value.
        .json::<serde_json::Value>()
        .await?;

    // Getting the value of the key "quote" from the JSON response.
    let json_kuote = response.get("quote");
    // Converting the JSON value to a string.
    serde_json::to_string(&json_kuote)
        .map_err(|e| { 
            e.into() 
        })
}

/// It gets the number of kuotes specified by the user, and returns them as a string
/// 
/// Arguments:
/// 
/// * `n`: the number of kuotes to get
/// * `remove_quotes`: whether or not to remove the quotes from the kuotes
/// 
/// Returns:
/// 
/// A Result<String>
pub async fn get(n: i32, remove_quotes: bool) -> Result<String> {
    let client = reqwest::Client::new();

    let mut kuotes: Vec<String> = Vec::new();
    for _ in 0..n {
        // Getting the kuote, and if it is successful, it is pushing it to the kuotes vector. If it is not
        // successful, it returns an error.
        match get_kuote(&client).await {
            Ok(s) => kuotes.push(
                // Removing the quotation marks from the string if the user specified that they wanted to.
                if remove_quotes {
                    let mut str = s; 
                    str.remove(0);
                    str.pop();
                    str
                } else {
                    s
                }
            ),
            Err(e) => return Err(e),
        }
    }

    Ok(kuotes.join("\n"))
}