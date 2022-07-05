extern crate reqwest;

fn main() {
    match reqwest::get("https://example.com") {
        Ok(response) => {
            // Check if 200 OK
            if response.status() == reqwest::StatusCode::OK {
                match response.text() {
                    Ok(text) => println!("Responce Text: {}", text)
                }
            } else {
                println!("Responce was not 200 OK");
            }
        }
        Err(_) => println!("Could not make the request!")
    }
}