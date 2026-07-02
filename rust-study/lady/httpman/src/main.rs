use std::collections::HashMap;
use std::str::FromStr;

use anyhow::anyhow;
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use reqwest::{header, Client, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Url;

#[derive(Parser)]
#[command(version, author, about = "HTTP client CLI - make HTTP requests from the terminal", long_about = None)]
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
    #[arg(value_parser = parse_kv_pairs)]
    body: Vec<KvPair>,
}

#[derive(Args)]
struct Put {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv_pairs)]
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
    #[arg(value_parser = parse_kv_pairs)]
    body: Vec<KvPair>,
}

#[derive(Debug, Clone)]
enum KvPairType {
    Header,
    Param,
}

#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
    t: KvPairType,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair_type: KvPairType;
        let split_char = if s.contains(':') {
            pair_type = KvPairType::Header;
            ':'
        } else {
            pair_type = KvPairType::Param;
            '='
        };

        let mut split = s.split(split_char);
        let err = || anyhow!("failed to parse pairs {}", s);
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
            t: pair_type,
        })
    }
}

fn parse_url(s: &str) -> anyhow::Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn parse_kv_pairs(s: &str) -> anyhow::Result<KvPair> {
    Ok(s.parse()?)
}

fn build_request<'a>(_client: &Client, _url: &str, body: &'a [KvPair]) -> anyhow::Result<(HashMap<&'a str, &'a str>, HeaderMap)> {
    let mut params = HashMap::new();
    let mut headers = HeaderMap::new();
    for pair in body {
        match pair.t {
            KvPairType::Param => {
                params.insert(pair.k.as_str(), pair.v.as_str());
            }
            KvPairType::Header => {
                if let Ok(name) = HeaderName::from_str(pair.k.as_str()) {
                    if let Ok(value) = HeaderValue::from_str(pair.v.as_str()) {
                        headers.insert(name, value);
                    } else {
                        eprintln!("Invalid header value for key: {}", pair.v);
                    }
                } else {
                    eprintln!("Invalid header key: {}", pair.k);
                }
            }
        }
    }
    Ok((params, headers))
}

async fn get(client: Client, args: &Get) -> anyhow::Result<()> {
    let resp = client.get(&args.url).send().await?;
    print_resp(resp).await
}

async fn post(client: Client, args: &Post) -> anyhow::Result<()> {
    let (body, headers) = build_request(&client, &args.url, &args.body)?;
    let resp = client
        .post(&args.url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;
    print_resp(resp).await
}

async fn put(client: Client, args: &Put) -> anyhow::Result<()> {
    let (body, headers) = build_request(&client, &args.url, &args.body)?;
    let resp = client
        .put(&args.url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;
    print_resp(resp).await
}

async fn delete(client: Client, args: &Delete) -> anyhow::Result<()> {
    let resp = client.delete(&args.url).send().await?;
    print_resp(resp).await
}

async fn head(client: Client, args: &Head) -> anyhow::Result<()> {
    let resp = client.head(&args.url).send().await?;
    print_resp(resp).await
}

async fn patch(client: Client, args: &Patch) -> anyhow::Result<()> {
    let (body, headers) = build_request(&client, &args.url, &args.body)?;
    let resp = client
        .patch(&args.url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;
    print_resp(resp).await
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
        Some(v) if v == mime::APPLICATION_JSON => {
            match jsonxf::pretty_print(body) {
                Ok(formatted) => println!("{}", formatted.cyan()),
                Err(_) => println!("{}", body),
            }
        }
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
async fn main() -> anyhow::Result<()> {
    let httpie = Httpie::parse();
    let client = Client::new();
    match httpie.methods {
        Method::Get(ref args) => get(client, args).await?,
        Method::Post(ref args) => post(client, args).await?,
        Method::Put(ref args) => put(client, args).await?,
        Method::Delete(ref args) => delete(client, args).await?,
        Method::Head(ref args) => head(client, args).await?,
        Method::Patch(ref args) => patch(client, args).await?,
    }
    Ok(())
}
