use clap::Parser;
use rcli::{Cli, Commands};
use std::fs;

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Encode { text }) => {
            let encoded = base64_encode(text);
            println!("Encoded: {}", encoded);
        }
        Some(Commands::Decode { text }) => {
            match base64_decode(text) {
                Ok(decoded) => println!("Decoded: {}", decoded),
                Err(e) => eprintln!("Decode error: {}", e),
            }
        }
        Some(Commands::Genpwd { length, special }) => {
            use rand::Rng;
            let chars = if *special {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{}|;:',.<>?/`~"
            } else {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
            };
            let mut rng = rand::thread_rng();
            let pwd: String = (0..*length)
                .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
                .collect();
            println!("Generated password: {}", pwd);
        }
        Some(Commands::Count { input, file }) => {
            let content = if *file {
                match fs::read_to_string(input) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error reading file: {}", e);
                        return;
                    }
                }
            } else {
                input.clone()
            };
            let lines = content.lines().count();
            let words: usize = content.split_whitespace().count();
            let chars = content.chars().count();
            println!("Lines: {}, Words: {}, Chars: {}", lines, words, chars);
        }
        Some(Commands::Bench) => {
            rcli::util::cal_time::cal_time();
        }
        None => {}
    }
}

fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let combined = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((combined >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((combined >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(CHARS[((combined >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(CHARS[(combined & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

fn base64_decode(input: &str) -> Result<String, String> {
    const DECODE: [i8; 128] = {
        let mut table = [-1i8; 128];
        let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut i = 0;
        while i < 64 {
            table[chars[i] as usize] = i as i8;
            i += 1;
        }
        table
    };

    let bytes: Vec<u8> = input
        .bytes()
        .filter(|&b| b != b'\n' && b != b'\r' && b != b' ')
        .collect();

    let padding = bytes.iter().filter(|&&b| b == b'=').count();
    let valid_bytes = bytes.len() - padding;

    let mut result = Vec::new();

    for chunk in bytes[..valid_bytes].chunks(4) {
        let mut buf = [0u8; 4];
        for (i, &b) in chunk.iter().enumerate() {
            if b as usize >= 128 || DECODE[b as usize] == -1 {
                return Err(format!("Invalid base64 character: {}", b as char));
            }
            buf[i] = DECODE[b as usize] as u8;
        }

        let combined = (buf[0] as u32) << 18
            | (buf[1] as u32) << 12
            | (buf[2] as u32) << 6
            | (buf[3] as u32);
        result.push((combined >> 16) as u8);
        if chunk.len() > 2 {
            result.push((combined >> 8) as u8);
        }
        if chunk.len() > 3 {
            result.push(combined as u8);
        }
    }

    String::from_utf8(result).map_err(|e| format!("UTF-8 error: {}", e))
}
