# Astronaut

Astronaut is a command-line tool that crawls a website and searches for secrets in JavaScript files.

## Features

- Crawls a website starting from the provided URL.
- Searches for secrets in JavaScript files.
- Prints the name of the secret and its value when found.

## Getting Started

To get started with the Secret Finder CLI, follow these steps:

Clone the repository:

   ```bash
   git clone https://github.com/yourusername/secret-finder-cli.git
   ```

## Build the project:

```bash
cargo build --release
```

## Run the CLI tool with a URL:

``` bash
cargo run -- <URL>
```
Example
```bash
cargo run -- https://example.com
```

Replace <URL> with the URL of the website you want to scan for secrets.

## Dependencies
reqwest
select
url
regex
structopt

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
Built with Rust.
