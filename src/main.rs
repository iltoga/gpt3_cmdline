#[allow(non_snake_case, dead_code)]
#[allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::io::{self, Write};
// use futures_util::future::TryFutureExt;

#[derive(Serialize)]
struct TextGenerationRequest {
    model: String,
    prompt: String,
    max_tokens: usize,
    temperature: f64,
    n: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct TextGenerationResponse {
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Serialize)]
struct ImageGenerationRequest {
    prompt: String,
    n: usize,
    size: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImageGenerationResponse {
    data: Vec<ImageData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImageData {
    url: String,
}


#[derive(Deserialize)]
struct ErrorResponse {
    error: Error,
}

#[derive(Deserialize)]
struct Error {
    message: String,
    r#type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key_string = std::env::var("OPENAI_KEY").unwrap();
    let api_key = api_key_string.as_str();
    let question = prompt_user("Enter your question: ", true);
    let output_format = prompt_user("Do you want the result as text or image? (Enter T for text or I for image): ", false);

    match output_format.trim().to_uppercase().as_str() {
        "T" => {
            let model_name = match prompt_user("Is your question relative to code? (Y or N): ", false).trim().to_lowercase().as_str() {
                "n" => "text-davinci-003",
                "y" => "code-davinci-002",
                _ => {
                    println!("Invalid answer. Please ender either 'y' or 'n'.");
                    return Ok(());
                }
            };
            let max_tokens_str = prompt_user("Enter maximum number of tokens to use: ", false);
            let max_tokens = max_tokens_str.trim().parse::<usize>().unwrap();
        
            let response = generate_text(api_key, question, max_tokens, model_name).await;
            match response {
                Ok(text_generation_response) => {
                    println!("AI: {}", text_generation_response.choices[0].text);
                    // println!(
                    //     "Stats:\n\tToken usage:\n\t{}: {}\n\t{}: {}\n\t{}: {}", 
                    //     "prompt", text_generation_response.usage.prompt_tokens, 
                    //     "completion", text_generation_response.usage.completion_tokens, 
                    //     "total", text_generation_response.usage.total_tokens
                    // );
                }
                Err(error_message) => {
                    println!("{}", error_message);
                }
            }
        }
        "I" => {
            let n_images = prompt_user("Enter number of images to create: ", false).parse::<usize>().unwrap();
            let response = generate_image(api_key, question, n_images).await?;
            let mut i = 0;
            for data in response.data {
                i += 1;
                println!("Generated image {}: {}\n\t\n\t", i, data.url);
            }
        }
        _ => {
            println!("Invalid output format. Please enter T or I.");
        }
    }

    Ok(())
}

fn prompt_user(message: &str, multiline: bool) -> String {
    print!("{} ", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    let stdin = io::stdin();

    if multiline {
        loop {
            let mut line = String::new();
            stdin.read_line(&mut line).expect("Failed to read line");

            // vim style
            if line.trim() == ":q" {
                break;
            } else {
                input.push_str(&line);
            }
        }
    } else {
        stdin.read_line(&mut input).expect("Failed to read line");
    }

    input.trim().to_string()
}

async fn generate_text(
    api_key: &str,
    question: String,
    max_tokens: usize,
    model_name: &str,
) -> Result<TextGenerationResponse, String> {
    let request = TextGenerationRequest {
        model: model_name.to_string(),
        prompt: question.trim().to_owned(),
        max_tokens,
        temperature: 0.0,
        n: 1,
    };
    send_request(request, api_key, "completions").await
}

async fn generate_image(
    api_key: &str,
    prompt: String,
    n_images: usize,
) -> Result<ImageGenerationResponse, String> {
    let request = ImageGenerationRequest {
        prompt,
        n: n_images,
        size: "512x512".to_string(),
    };

    send_request(request, api_key, "images/generations").await
}

async fn send_request<T: Serialize + DeserializeOwned>(
    request: impl Serialize,
    api_key: &str,
    endpoint: &str,
) -> Result<T, String> {
    // let response = reqwest::Client::new()
    // .post(&format!("https://api.openai.com/v1/{}", endpoint))
    // .header("Content-Type", "application/json")
    // .header("Authorization", format!("Bearer {}", api_key))
    // .json(&request)
    // .send()
    // .await.and_then(|resp| Ok(resp.text().map_err(|err| Box::from(err) as Box<dyn std::error::Error>)));


    let response = reqwest::Client::new()
        .post(&format!("https://api.openai.com/v1/{}", endpoint))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                let json = resp.json::<T>().await;
                match json {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        let err_resp = format!("JSON decoding error: {}", e);
                        Err(err_resp)
                    }
                }
            } else {
                parse_request_error(resp).await
            }
        }
        Err(e) => {
            Err(format!("Request error: {}", e))
        }
    }
}


async fn parse_request_error<T: DeserializeOwned>(resp: reqwest::Response) -> Result<T, String> {
    let err_resp = resp.json::<ErrorResponse>().await;
    match err_resp {
        Ok(ErrorResponse { error }) => {
            let message = error.message;
            let err_type = error.r#type;
            let err_resp = format!("{}: {}", err_type, message);
            Err(err_resp)
        }
        Err(e) => {
            let err_resp = format!("JSON decoding error: {}", e);
            Err(err_resp)
        }
    }
}
