use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use std::{collections::HashMap, fmt, path::PathBuf};
/// The root configuration structure for the browser router.
///
/// This struct corresponds directly to the top-level entries
/// in the `config.toml` file.
///
/// Example TOML:
/// ```toml
/// default = "firefox"
///
/// [browsers]
/// firefox = "/usr/bin/firefox"
///
/// [[rules]]
/// regex = ".*\\.youtube\\.com.*"
/// browser = "brave"
/// ```
#[derive(Debug, Deserialize, JsonSchema)]
pub struct Settings {
   /// The name of the default browser to be used when no rule matches.
   pub default: String,

   /// A mapping of browser names to either a command string or a structured
   /// configuration (supports both simple and advanced forms using an
   /// untagged enum).
   pub browsers: HashMap<String, BrowserConfig>,

   /// A list of routing rules. The first rule whose regex matches
   /// a given URL is used to select the target browser.
   pub rules: Vec<Rule>,
}

impl fmt::Display for Settings {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Settings")
         .field("default", &self.default)
         .field(
            "browsers",
            &self
               .browsers
               .values()
               .map(|v| v.to_string())
               .collect::<Vec<_>>(),
         )
         .field(
            "rules",
            &self.rules.iter().map(|r| r.to_string()).collect::<Vec<_>>(),
         )
         .finish()
   }
}

/// Defines the configuration for a single browser.
///
/// This supports two styles of TOML configuration:
///
/// ### Simple form:
/// ```toml
/// [browsers]
/// firefox = "/usr/bin/firefox"
/// ```
///
/// ### Advanced form (with args):
/// ```toml
/// [browsers.firefox]
/// path = "/usr/bin/firefox"
/// args = ["--new-window"]
/// ```
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum BrowserConfig {
   /// Simple form: just the command path as a string.
   Simple(PathBuf),

   /// Advanced form: structured path + optional arguments.
   Detailed {
      /// Path to the browser executable.
      path: PathBuf,
      /// Optional list of arguments to be passed to the browser
      /// when executed.
      #[serde(default)]
      args: Vec<String>,
   },
}

impl fmt::Display for BrowserConfig {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
         BrowserConfig::Simple(path) => write!(f, "{}", path.display()),
         BrowserConfig::Detailed { path, args } => {
            write!(f, "{} {}", path.display(), args.join(" "))
         }
      }
   }
}

/// Defines a single rule for routing URLs to browsers.
///
/// The rules are evaluated **in order**; the first matching
/// regex determines the browser used.
///
/// Example:
/// ```toml
/// [[rules]]
/// regex = ".*\\.work\\.com.*"
/// browser = "chrome"
/// ```
#[derive(Debug, Deserialize, JsonSchema)]
pub struct Rule {
   /// The regular expression pattern used to match the URL.
   #[serde(
      deserialize_with = "serde_regex::deserialize",
      serialize_with = "serde_regex::serialize"
   )]
   pub regex: Regex,

   /// The name of the browser to use if the regex matches.
   /// Must be one of the keys defined in `browsers`.
   pub browser: String,
}

impl fmt::Display for Rule {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{} => {}", self.regex, self.browser)
   }
}

mod serde_regex {
   use regex::Regex;
   use serde::Deserialize;

   pub fn deserialize<'de, D>(deserializer: D) -> Result<Regex, D::Error>
   where
      D: serde::Deserializer<'de>,
   {
      let s = String::deserialize(deserializer)?;
      Regex::new(&s).map_err(serde::de::Error::custom)
   }
   #[allow(dead_code)]
   pub fn serialize<S>(regex: &Regex, serializer: S) -> Result<S::Ok, S::Error>
   where
      S: serde::Serializer,
   {
      serializer.serialize_str(regex.as_str())
   }
}
