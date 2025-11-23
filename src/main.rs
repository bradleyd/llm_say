use clap::Parser;
use colored::{self, ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
mod characters;

#[derive(Parser)]
struct Cli {
    message: String,
    #[clap(short = 'm', long = "model", default_value = "llama3.2")]
    model: String,
    #[clap(short = 'c', long = "character", default_value = "ferris")]
    character: String,
}

#[derive(Deserialize, Debug)]
struct LLMResponse {
    model: String,
    response: String,
    done: bool,
}
#[derive(Serialize, Debug)]
struct LLMRequest {
    model: String,
    prompt: String,
    stream: bool,
}

fn main() {
    let cli = Cli::parse();
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async { get_llm_response(&cli.message, &cli.model).await });
    match result {
        Ok(response) => {
            display_llm_say(&response.response, &cli.character);
        }
        Err(err) => println!("Error getting llm response with {:?}", err),
    }
}

async fn get_llm_response(
    message: &str,
    model: &str,
) -> Result<LLMResponse, Box<dyn std::error::Error>> {
    // build reqwest
    let client = reqwest::Client::new();
    let system_prompt = String::from(
        "Answer concisely in 2-3 sentences with a touch of humor. \
     Plain text only, no formatting or quotes.",
    );

    let request = LLMRequest {
        model: model.to_string(),
        prompt: format!("{}, Answer the following\n {}", system_prompt, message),
        stream: false,
    };
    dbg!(&request);

    let response = client
        .post(format!(
            "{}/api/generate",
            "http://localhost:11434".to_string()
        ))
        .json(&request)
        .send()
        .await?;

    dbg!(&response);
    let resp: LLMResponse = response.json().await?;

    Ok(resp)
}

fn display_llm_say(message: &str, character: &str) {
    let cleaned = message.trim().trim_matches('"').trim();
    let bubble = format_bubble(cleaned);

    println!("{}", bubble.bright_white());
    println!("{}", "         \\  ".bright_black());
    println!("{}", "          \\  ".bright_black());
    println!("{}", generate_character(character));
}

fn format_bubble(text: &str) -> String {
    let wrapped = wrap_text(text, 60);
    let lines: Vec<&str> = wrapped.lines().collect();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let top = format!(" {}", "_".repeat(max_width + 2));
    let bottom = format!(" {}", "-".repeat(max_width + 2));
    let mut results = vec![top];
    for line in lines {
        results.push(format!("/ {:<width$} \\", line, width = max_width));
    }

    results.push(bottom);
    results.join("\n")
}

fn wrap_text(text: &str, max_width: usize) -> String {
    let mut result = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        // Check if adding this word would exceed width
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        if test_line.len() <= max_width {
            current_line = test_line;
        } else {
            // Line would be too long, save current and start new
            if !current_line.is_empty() {
                result.push(current_line);
            }
            current_line = word.to_string();
        }
    }

    // Don't forget the last line
    if !current_line.is_empty() {
        result.push(current_line);
    }

    result.join("\n")
}

fn generate_character(character: &str) -> ColoredString {
    match character {
        "ferris" => characters::ferris(),
        "cow" => characters::cow(),
        "dragon" => characters::dragon(),
        "bunny" => characters::bunny(),
        _ => characters::ferris(),
    }
}
