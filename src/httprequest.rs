use futures::executor;
use reqwest;

pub async fn shorter() {
    let response_text = reqwest::get("")
        .await
        .expect("Error calling page") // this will panic
        .text()
        .await
        .expect("Error reading text");
    println!("{}", response_text);
}
pub fn run() {
    let vals = async {
        //shorter().await;
        match reqwest::get("").await {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::OK {
                    match response.text().await {
                        Ok(text) => println!("Response Text: {}", text),
                        Err(_) => println!("Could not read text"),
                    }
                } else {
                    println!("Response was not 200.")
                }
            }
            Err(_) => {
                println!("Error");
            }
        }
    };
    executor::block_on(vals);
}
