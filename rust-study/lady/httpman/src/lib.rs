//! httpman - HTTP client CLI library.
//!
//! Contains the request building blocks (URL / key-value pair parsing) so
//! they can be unit tested independently of the binary.
//!
//! `util` holds the `macro_rules!` teaching examples and is re-exported here.

pub mod util;

use std::collections::HashMap;
use std::str::FromStr;

use anyhow::anyhow;
use reqwest::Url;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

/// What a parsed key-value pair means for the request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KvPairType {
    /// Goes into the request headers (`key:value`).
    Header,
    /// Goes into the JSON body (`key=value`).
    Param,
}

/// A single `key:value` (header) or `key=value` (body param) argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvPair {
    pub k: String,
    pub v: String,
    pub t: KvPairType,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // `key:value` is a header, anything without ':' is a body param.
        let (pair_type, split_char) = if s.contains(':') {
            (KvPairType::Header, ':')
        } else {
            (KvPairType::Param, '=')
        };

        // splitn(2, ...) keeps separators inside the value working,
        // e.g. `token: a:b` or `json={"a":1}`.
        let mut split = s.splitn(2, split_char);
        let k = split.next().unwrap_or_default();
        let v = split.next().ok_or_else(|| {
            anyhow!("failed to parse pair `{s}`: expected `<key>{split_char}<value>`")
        })?;
        if k.is_empty() {
            return Err(anyhow!("failed to parse pair `{s}`: empty key"));
        }
        Ok(Self {
            k: k.to_string(),
            v: v.to_string(),
            t: pair_type,
        })
    }
}

/// Parse and validate a URL argument.
pub fn parse_url(s: &str) -> anyhow::Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

/// Parse a single key-value pair argument (used as a clap value parser).
pub fn parse_kv_pair(s: &str) -> anyhow::Result<KvPair> {
    s.parse()
}

/// Split parsed pairs into a JSON-body map and a header map.
///
/// Invalid header names/values are rejected instead of silently dropped.
pub fn build_request(body: &[KvPair]) -> anyhow::Result<(HashMap<String, String>, HeaderMap)> {
    let mut params = HashMap::new();
    let mut headers = HeaderMap::new();
    for pair in body {
        match pair.t {
            KvPairType::Param => {
                params.insert(pair.k.clone(), pair.v.clone());
            }
            KvPairType::Header => {
                let name = HeaderName::from_str(&pair.k)
                    .map_err(|e| anyhow!("invalid header name `{}`: {e}", pair.k))?;
                let value = HeaderValue::from_str(&pair.v)
                    .map_err(|e| anyhow!("invalid header value for `{}`: {e}", pair.k))?;
                headers.insert(name, value);
            }
        }
    }
    Ok((params, headers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_header_pair() {
        let p: KvPair = "Content-Type:application/json".parse().unwrap();
        assert_eq!(p.k, "Content-Type");
        assert_eq!(p.v, "application/json");
        assert_eq!(p.t, KvPairType::Header);
    }

    #[test]
    fn parse_param_pair() {
        let p: KvPair = "user=zhang".parse().unwrap();
        assert_eq!(p.k, "user");
        assert_eq!(p.v, "zhang");
        assert_eq!(p.t, KvPairType::Param);
    }

    #[test]
    fn value_may_contain_separators() {
        // splitting stops at the first separator, so values keep theirs
        let p: KvPair = "token:abc:def".parse().unwrap();
        assert_eq!(p.t, KvPairType::Header);
        assert_eq!(p.v, "abc:def");

        let p: KvPair = "query=a=b".parse().unwrap();
        assert_eq!(p.t, KvPairType::Param);
        assert_eq!(p.v, "a=b");
    }

    #[test]
    fn empty_value_is_allowed() {
        let p: KvPair = "X-Empty:".parse().unwrap();
        assert_eq!(p.v, "");

        let p: KvPair = "empty=".parse().unwrap();
        assert_eq!(p.v, "");
    }

    #[test]
    fn missing_value_is_rejected() {
        assert!("no-separator".parse::<KvPair>().is_err());
        assert!(":no-key".parse::<KvPair>().is_err());
    }

    #[test]
    fn parse_url_validates() {
        assert!(parse_url("https://httpbin.org/get").is_ok());
        assert!(parse_url("not a url").is_err());
    }

    #[test]
    fn build_request_splits_params_and_headers() {
        let pairs: Vec<KvPair> = vec![
            "user=admin".parse().unwrap(),
            "X-Token:secret".parse().unwrap(),
        ];
        let (params, headers) = build_request(&pairs).unwrap();
        assert_eq!(params.get("user").map(String::as_str), Some("admin"));
        assert_eq!(headers.get("X-Token").unwrap(), "secret");
    }

    #[test]
    fn invalid_header_is_rejected() {
        let pairs: Vec<KvPair> = vec!["bad header\n:value".parse().unwrap()];
        assert!(build_request(&pairs).is_err());
    }
}
