//! Error handling without exceptions or null: `Result`, custom error types, `?` propagation, and `From` conversions.

use std::fmt;
use std::num::ParseIntError;

/// Everything that can go wrong while parsing a `key = value` config.
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    /// A line had no `=`.
    MissingEquals {
        /// 1-based line number.
        line: usize,
    },
    /// A number failed to parse.
    BadNumber {
        /// 1-based line number.
        line: usize,
        /// Underlying parse error.
        source: ParseIntError,
    },
    /// A value was outside the allowed range.
    OutOfRange {
        /// Config key.
        key: String,
        /// Value given.
        value: i64,
        /// Minimum allowed.
        min: i64,
        /// Maximum allowed.
        max: i64,
    },
    /// A required key was absent.
    Missing(&'static str),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingEquals { line } => write!(f, "line {line}: expected `key = value`"),
            ConfigError::BadNumber { line, source } => write!(f, "line {line}: {source}"),
            ConfigError::OutOfRange {
                key,
                value,
                min,
                max,
            } => write!(f, "{key} = {value} is outside {min}..={max}"),
            ConfigError::Missing(k) => write!(f, "missing required key `{k}`"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::BadNumber { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Validated server settings.
#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    /// TCP port.
    pub port: u16,
    /// Worker threads.
    pub workers: u8,
}

fn ranged(key: &str, v: i64, min: i64, max: i64) -> Result<i64, ConfigError> {
    if (min..=max).contains(&v) {
        Ok(v)
    } else {
        Err(ConfigError::OutOfRange {
            key: key.into(),
            value: v,
            min,
            max,
        })
    }
}

/// Parses text like `port = 8080` / `workers = 4`. Every failure is a typed error, never a crash.
pub fn parse_config(text: &str) -> Result<ServerConfig, ConfigError> {
    let (mut port, mut workers) = (None, None);
    for (i, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let (k, v) = line
            .split_once('=')
            .ok_or(ConfigError::MissingEquals { line: i + 1 })?;
        let n: i64 = v.trim().parse().map_err(|source| ConfigError::BadNumber {
            line: i + 1,
            source,
        })?;
        match k.trim() {
            "port" => port = Some(ranged("port", n, 1, 65535)? as u16),
            "workers" => workers = Some(ranged("workers", n, 1, 64)? as u8),
            _ => {} // unknown keys are ignored
        }
    }
    Ok(ServerConfig {
        port: port.ok_or(ConfigError::Missing("port"))?,
        workers: workers.unwrap_or(4),
    })
}

/// Checked arithmetic: overflow becomes `None` instead of wrapping silently (a common C bug in size calculations).
pub fn buffer_size(count: usize, item_size: usize) -> Option<usize> {
    count.checked_mul(item_size)?.checked_add(16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_and_invalid_configs() {
        assert_eq!(
            parse_config("port = 8080\n# comment\nworkers=8").unwrap(),
            ServerConfig {
                port: 8080,
                workers: 8
            }
        );
        assert_eq!(
            parse_config("workers = 2").unwrap_err(),
            ConfigError::Missing("port")
        );
        assert!(matches!(
            parse_config("port 80"),
            Err(ConfigError::MissingEquals { line: 1 })
        ));
        let e = parse_config("port = eighty").unwrap_err();
        assert_eq!(e.to_string(), "line 1: invalid digit found in string");
        assert!(std::error::Error::source(&e).is_some());
        assert_eq!(
            parse_config("port = 70000").unwrap_err().to_string(),
            "port = 70000 is outside 1..=65535"
        );
    }

    #[test]
    fn overflow_is_detected() {
        assert_eq!(buffer_size(10, 4), Some(56));
        assert_eq!(buffer_size(usize::MAX / 2, 4), None);
    }
}
