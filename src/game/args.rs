//! CLI arguments for the Irony GameHandler helper (`-i` / `--id`, `-a` / `--alternate`).

use std::num::ParseIntError;

/// Parsed GameHandler-style arguments (Irony `CommandLineArgs`).
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct GameHandlerArgs {
    pub steam_app_id: Option<u64>,
    pub use_alternate_launch: bool,
}

/// Failure while parsing argv (missing value or bad integer).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseArgsError {
    MissingValueForId,
    InvalidId(String),
}

impl std::fmt::Display for ParseArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingValueForId => write!(f, "missing value for -i/--id"),
            Self::InvalidId(s) => write!(f, "invalid steam app id: {s}"),
        }
    }
}

impl std::error::Error for ParseArgsError {}

impl From<ParseIntError> for ParseArgsError {
    fn from(e: ParseIntError) -> Self {
        Self::InvalidId(e.to_string())
    }
}

/// Parse GameHandler arguments from an argv slice (excluding the program name).
///
/// Parity: Irony `CommandLine` options `-i`/`--id`, `-a`/`--alternate`.
pub fn parse_args(args: &[String]) -> Result<GameHandlerArgs, ParseArgsError> {
    let mut out = GameHandlerArgs::default();
    let mut i = 0usize;
    while i < args.len() {
        let a = args[i].as_str();
        match a {
            "-a" | "--alternate" => {
                out.use_alternate_launch = true;
                i += 1;
            }
            "-i" | "--id" => {
                let v = args.get(i + 1).ok_or(ParseArgsError::MissingValueForId)?;
                out.steam_app_id = Some(v.parse::<u64>()?);
                i += 2;
            }
            s => {
                if let Some(rest) = s.strip_prefix("--id=") {
                    out.steam_app_id = Some(rest.parse::<u64>()?);
                }
                i += 1;
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(args: &[&str]) -> Vec<String> {
        args.iter().map(|x| (*x).to_string()).collect()
    }

    #[test]
    fn parse_short_flags() {
        let a = parse_args(&s(&["-i", "1158310", "-a"])).expect("parse");
        assert_eq!(a.steam_app_id, Some(1158310));
        assert!(a.use_alternate_launch);
    }

    #[test]
    fn parse_long_id_equals() {
        let a = parse_args(&s(&["--id=123"])).expect("parse");
        assert_eq!(a.steam_app_id, Some(123));
        assert!(!a.use_alternate_launch);
    }

    #[test]
    fn parse_missing_id_value_errors() {
        assert!(matches!(
            parse_args(&s(&["-i"])),
            Err(ParseArgsError::MissingValueForId)
        ));
    }
}
