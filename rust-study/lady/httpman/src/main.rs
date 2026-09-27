use std::process::ExitCode;

use clap::CommandFactory;
use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;
use colored::Colorize;
use httpman::{KvPair, build_request, parse_kv_pair, parse_url};
use reqwest::{Client, Response, header};

#[derive(Parser)]
#[command(
    version,
    author,
    about = "HTTP client CLI - make HTTP requests from the terminal",
    long_about = None
)]
struct Httpie {
    #[command(subcommand)]
    pub methods: Method,
}

#[derive(Subcommand)]
enum Method {
    /// Send a GET request
    Get(Get),
    /// Send a POST request
    Post(Post),
    /// Send a PUT request
    Put(Put),
    /// Send a DELETE request
    Delete(Delete),
    /// Send a HEAD request
    Head(Head),
    /// Send a PATCH request
    Patch(Patch),
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(short, long, value_enum)]
        shell: Shell,
    },
}

#[derive(Args)]
struct Get {
    #[arg(value_parser = parse_url)]
    url: String,
}

#[derive(Args)]
struct Post {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Args)]
struct Put {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Args)]
struct Delete {
    #[arg(value_parser = parse_url)]
    url: String,
}

#[derive(Args)]
struct Head {
    #[arg(value_parser = parse_url)]
    url: String,
}

#[derive(Args)]
struct Patch {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

async fn run(client: Client, method: Method) -> anyhow::Result<()> {
    match method {
        Method::Get(args) => print_resp(client.get(&args.url).send().await?).await,
        Method::Post(args) => {
            let (body, headers) = build_request(&args.body)?;
            let resp = client
                .post(&args.url)
                .headers(headers)
                .json(&body)
                .send()
                .await?;
            print_resp(resp).await
        }
        Method::Put(args) => {
            let (body, headers) = build_request(&args.body)?;
            let resp = client
                .put(&args.url)
                .headers(headers)
                .json(&body)
                .send()
                .await?;
            print_resp(resp).await
        }
        Method::Delete(args) => print_resp(client.delete(&args.url).send().await?).await,
        Method::Head(args) => print_resp(client.head(&args.url).send().await?).await,
        Method::Patch(args) => {
            let (body, headers) = build_request(&args.body)?;
            let resp = client
                .patch(&args.url)
                .headers(headers)
                .json(&body)
                .send()
                .await?;
            print_resp(resp).await
        }
        Method::Completions { shell } => {
            let mut cmd = Httpie::command();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(shell, &mut cmd, name, &mut out);
            print!("{}", String::from_utf8_lossy(&out));
            Ok(())
        }
    }
}

async fn print_resp(resp: Response) -> anyhow::Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (k, v) in resp.headers() {
        println!("{}: {:?}", k.to_string().green(), v);
    }
    println!();
}

fn print_body(mime: Option<mime::Mime>, body: &str) {
    match mime {
        Some(v) if v == mime::APPLICATION_JSON => match jsonxf::pretty_print(body) {
            Ok(formatted) => println!("{}", formatted.cyan()),
            Err(_) => println!("{}", body),
        },
        _ => print!("{}", body),
    }
}

fn get_content_type(resp: &Response) -> Option<mime::Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
}

#[tokio::main]
async fn main() -> ExitCode {
    let httpie = Httpie::parse();
    let method = httpie.methods;
    let client = Client::new();
    match run(client, method).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:#}");
            ExitCode::FAILURE
        }
    }
}
