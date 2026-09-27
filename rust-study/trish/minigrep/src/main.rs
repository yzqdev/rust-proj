use clap::Parser;
use colored::*;
use minigrep::options::Options;
use minigrep::*;
use std::fs;
use std::io::Read;
use std::process;

/// A fast and simple grep-like text search tool
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Query string to search for
    query: String,

    /// File path to search in
    file: Vec<String>,

    /// Case insensitive search
    #[arg(short, long)]
    insensitive: bool,

    /// Exact word match (whole word only)
    #[arg(short = 'w', long)]
    exact_word: bool,

    /// Count matches instead of showing them
    #[arg(short, long)]
    count: bool,

    /// Show line numbers
    #[arg(short = 'n', long)]
    line_number: bool,
}

pub fn main() {
    let args = Args::parse();

    let _options = Options::new(args.insensitive, args.exact_word);
    // Build options string for legacy parse_config
    let mut opt_str = String::new();
    if args.insensitive {
        opt_str.push('i');
    }
    if args.exact_word {
        opt_str.push('w');
    }

    // If no files specified, try to read from stdin
    let files: Vec<String> = if args.file.is_empty() {
        vec!["-".to_string()]
    } else {
        args.file.clone()
    };

    for file_path in &files {
        if file_path == "-" {
            // Read from stdin
            let mut content = String::new();
            if std::io::stdin().read_to_string(&mut content).is_err() {
                eprintln!("minigrep: error reading stdin");
                process::exit(1);
            }
            search_and_print(&args, &content, file_path, files.len() > 1);
        } else {
            match fs::read_to_string(file_path) {
                Ok(content) => {
                    search_and_print(&args, &content, file_path, files.len() > 1);
                }
                Err(e) => {
                    eprintln!("minigrep: {}: {}", file_path, e);
                    process::exit(1);
                }
            }
        }
    }
}

fn search_and_print(args: &Args, content: &str, file_path: &str, multi_file: bool) {
    let query = &args.query;
    let mut matched_count = 0;

    for (line_num, line) in content.lines().enumerate() {
        let matched = if args.insensitive {
            line.to_lowercase().contains(&query.to_lowercase())
        } else if args.exact_word {
            let pattern = format!(r"\b{}\b", regex::escape(query));
            if let Ok(re) = regex::Regex::new(&pattern) {
                re.is_match(line)
            } else {
                line.contains(query)
            }
        } else {
            line.contains(query)
        };

        if matched {
            matched_count += 1;
            if args.count {
                continue;
            }

            // Highlight matched text
            let highlighted = if args.insensitive {
                let lower_line = line.to_lowercase();
                let lower_q = query.to_lowercase();
                let mut result = String::new();
                let mut pos = 0;
                while let Some(idx) = lower_line[pos..].find(&lower_q) {
                    result.push_str(&line[pos..pos + idx]);
                    result.push_str(
                        &line[pos + idx..pos + idx + query.len()]
                            .green()
                            .bold()
                            .to_string(),
                    );
                    pos += idx + query.len();
                }
                result.push_str(&line[pos..]);
                result
            } else {
                let mut result = String::new();
                let mut pos = 0;
                while let Some(idx) = line[pos..].find(query) {
                    result.push_str(&line[pos..pos + idx]);
                    result.push_str(
                        &line[pos + idx..pos + idx + query.len()]
                            .green()
                            .bold()
                            .to_string(),
                    );
                    pos += idx + query.len();
                }
                result.push_str(&line[pos..]);
                result
            };

            if file_path != "-" && multi_file {
                print!("{}:", file_path.blue().bold());
            }
            if args.line_number {
                print!("{}:", (line_num + 1).to_string().yellow().bold());
            }
            println!("{}", highlighted);
        }
    }

    if args.count {
        println!("{}: {} match(es)", file_path, matched_count);
    }
}

// Legacy entry point kept for compatibility
fn _legacy_main() {
    let args: Vec<String> = std::env::args().collect();

    let mut config = parse_config(&args).unwrap_or_else(|err| {
        eprintln!("Error occurred while processing: {}", err);
        process::exit(1);
    });

    let mut file_content = String::new();
    config
        .get_file()
        .read_to_string(&mut file_content)
        .expect("Something went wrong while reading the file");
    let matched_indices = search(&mut config);

    if matched_indices.is_empty() {
        process::exit(0);
    }

    let mut start = 0;
    let query_length: usize = config.get_query().len();
    let mut matched_indices_as_iter = matched_indices.iter();

    loop {
        let matched_index = matched_indices_as_iter.next();

        if matched_index.is_none() {
            let normal = &file_content[start..];
            print!("{}", normal);
            break;
        }

        let matched_index = matched_index.unwrap();
        let normal = &file_content[start..*matched_index];
        let highlight_end_pos = matched_index + query_length;
        let highlight = &file_content[*matched_index..highlight_end_pos];

        print!("{}", normal);
        print!("{}", highlight.green().bold());
        start = highlight_end_pos;
    }
}
