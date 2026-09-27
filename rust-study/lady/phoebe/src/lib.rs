//! phoebe - CSV processor & package installer library.
//!
//! Holds the CSV→JSON conversion so it can be unit tested with in-memory
//! readers instead of the real filesystem.

use std::io::Read;

/// Errors reported by the conversion.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reading the CSV source failed.
    #[error("failed to read CSV input: {0}")]
    Csv(#[from] csv::Error),

    /// Serializing the records to JSON failed.
    #[error("failed to serialize JSON output: {0}")]
    Json(#[from] serde_json::Error),

    /// The delimiter must be a single ASCII character.
    #[error("invalid delimiter `{0}`: expected a single ASCII character")]
    Delimiter(char),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Result of a CSV→JSON conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conversion {
    /// Number of records written.
    pub count: usize,
    /// JSON document (array of objects).
    pub json: String,
    /// Non-fatal problems, one per skipped malformed row.
    pub warnings: Vec<String>,
}

/// Convert CSV input to a JSON array of objects.
///
/// With `has_headers` the first row supplies the object keys; otherwise
/// generated `column_N` keys are used. Malformed rows are skipped and
/// reported through [`Conversion::warnings`].
pub fn csv_to_json<R: Read>(
    reader: R,
    delimiter: char,
    has_headers: bool,
    pretty: bool,
) -> Result<Conversion> {
    if !delimiter.is_ascii() {
        return Err(Error::Delimiter(delimiter));
    }

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(has_headers)
        .from_reader(reader);

    let headers: Vec<String> = if has_headers {
        csv_reader
            .headers()?
            .iter()
            .map(|h| h.to_string())
            .collect()
    } else {
        Vec::new()
    };

    let mut records: Vec<serde_json::Value> = Vec::new();
    let mut warnings = Vec::new();
    for result in csv_reader.records() {
        match result {
            Ok(record) => {
                let mut obj = serde_json::Map::new();
                for (i, field) in record.iter().enumerate() {
                    let key = headers
                        .get(i)
                        .cloned()
                        .unwrap_or_else(|| format!("column_{i}"));
                    obj.insert(key, serde_json::Value::String(field.to_string()));
                }
                records.push(serde_json::Value::Object(obj));
            }
            Err(e) => warnings.push(format!("skipping row: {e}")),
        }
    }

    let json = if pretty {
        serde_json::to_string_pretty(&records)?
    } else {
        serde_json::to_string(&records)?
    };

    Ok(Conversion {
        count: records.len(),
        json,
        warnings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn converts_with_header_row() {
        let input = "name,age\nalice,30\nbob,25\n";
        let conv = csv_to_json(Cursor::new(input), ',', true, false).unwrap();
        assert_eq!(conv.count, 2);
        assert!(conv.warnings.is_empty());
        assert!(conv.json.contains(r#""name":"alice""#));
        assert!(conv.json.contains(r#""age":"25""#));
    }

    #[test]
    fn converts_without_header_row() {
        let input = "alice,30\nbob,25\n";
        let conv = csv_to_json(Cursor::new(input), ',', false, false).unwrap();
        assert_eq!(conv.count, 2);
        assert!(conv.json.contains(r#""column_0":"alice""#));
        assert!(conv.json.contains(r#""column_1":"30""#));
    }

    #[test]
    fn custom_delimiter() {
        let input = "name;age\nalice;30\n";
        let conv = csv_to_json(Cursor::new(input), ';', true, false).unwrap();
        assert!(conv.json.contains(r#""name":"alice""#));
    }

    #[test]
    fn pretty_output_has_newlines() {
        let input = "name\nalice\n";
        let conv = csv_to_json(Cursor::new(input), ',', true, true).unwrap();
        assert!(conv.json.contains('\n'));
    }

    #[test]
    fn malformed_row_is_skipped_with_warning() {
        let input = "name,age\nalice,30\n\"unterminated,40\n";
        let conv = csv_to_json(Cursor::new(input), ',', true, false).unwrap();
        assert_eq!(conv.count, 1);
        assert_eq!(conv.warnings.len(), 1);
    }

    #[test]
    fn non_ascii_delimiter_is_rejected() {
        let err = csv_to_json(Cursor::new(""), '，', true, false).unwrap_err();
        assert!(err.to_string().contains("invalid delimiter"));
    }
}
