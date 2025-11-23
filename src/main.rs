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
    #[clap(short = 'u', long = "url", default_value = "http://localhost:11434")]
    url: String,
}

#[derive(Deserialize, Debug)]
struct LLMResponse {
    //model: String,
    response: String,
    //done: bool,
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
    let result = rt.block_on(async { get_llm_response(&cli.message, &cli.model, &cli.url).await });
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
    url: &str,
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

    #[cfg(debug_assertions)]
    dbg!(&request);

    let response = client
        .post(format!("{}/api/generate", url))
        .json(&request)
        .send()
        .await?;

    #[cfg(debug_assertions)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_text_respects_max_width() {
        let text = "This is a sample message that should wrap nicely";
        let wrapped = wrap_text(text, 10);
        let lines: Vec<&str> = wrapped.lines().collect();

        assert_eq!(
            lines,
            vec![
                "This is a",
                "sample",
                "message",
                "that",
                "should",
                "wrap",
                "nicely"
            ]
        );

        assert!(lines.iter().all(|line| line.len() <= 10));
    }

    #[test]
    fn format_bubble_aligns_all_lines() {
        let text = "This longer bit of text should wrap across multiple lines \
                    and still be aligned inside the bubble.";
        let bubble = format_bubble(text);
        let wrapped = wrap_text(text, 60);
        let wrapped_lines: Vec<&str> = wrapped.lines().collect();
        let bubble_lines: Vec<&str> = bubble.lines().collect();
        let max_width = wrapped_lines.iter().map(|l| l.len()).max().unwrap_or(0);

        assert_eq!(bubble_lines.len(), wrapped_lines.len() + 2);
        assert_eq!(
            bubble_lines.first().unwrap(),
            &format!(" {}", "_".repeat(max_width + 2))
        );
        assert_eq!(
            bubble_lines.last().unwrap(),
            &format!(" {}", "-".repeat(max_width + 2))
        );

        for (idx, wrapped_line) in wrapped_lines.iter().enumerate() {
            let body_line = bubble_lines[idx + 1];
            assert_eq!(
                body_line,
                format!("/ {:<width$} \\", wrapped_line, width = max_width)
            );
        }
    }

    #[test]
    fn generate_character_defaults_to_ferris() {
        colored::control::set_override(false);
        let ferris = generate_character("ferris").to_string();
        let fallback = generate_character("unknown").to_string();

        assert_eq!(fallback, ferris);
    }
}
