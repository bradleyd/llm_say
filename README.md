# llmsay

Command-line cowsay clone that sends a prompt to a local LLM server and prints the reply inside an ASCII speech bubble, optionally signed by a character such as Ferris the crab.

## Prerequisites

- Rust toolchain (tested with stable 1.80+)
- Local llm server like Ollama exposing `POST /api/generate` on `http://localhost:11434`

## Usage

```bash
cargo run -- "What is Rust ownership?" -m llama3.2 -c ferris -u http://127.0.0.01:11434
```

### CLI options

- `message` (positional) – the prompt to send to the LLM.
- `-m, --model <name>` – model identifier forwarded to the API (default `llama3.2`).
- `-c, --character <name>` – ASCII art to display (`ferris`, `cow`, `dragon`, `bunny`; defaults to `ferris`).
- -u, --url <name> - URL, with http://, for the LLM enpoint.

Responses are trimmed, wrapped to 60 characters per line, and displayed in bright white. The ASCII art lives in `src/characters.rs` if you want to add new mascots.

## Characters

- ferris
- cow
- dragon
- bunny

## Testing

Run the unit tests (bubble formatting, wrapping, and character selection) with:

```bash
cargo test
```
