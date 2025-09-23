use crate::{BrowserConfig, Settings};
use anyhow::{Context, Result, ensure};
use std::process::Command;
use url::Url;

pub fn run(settings: Settings, url: Url) -> Result<()> {
   let url = url.to_string();

   let browsers = settings.browsers;

   let matching_browser = settings
      .rules
      .iter()
      .find(|rule| rule.regex.is_match(&url))
      .map(|rule| rule.browser.clone())
      .unwrap_or(settings.default);

   ensure!(
      browsers.contains_key(&matching_browser),
      format!("browser {} not found", matching_browser)
   );

   let browser = browsers.get(&matching_browser).unwrap(); // Safe unwrap because of above code

   let browser = match browser {
      BrowserConfig::Simple(path) => Command::new(path).arg(&url).spawn(),
      BrowserConfig::Detailed { path, args } => Command::new(path).args(args).arg(&url).spawn(),
   };

   browser.with_context(|| format!("failed to launch browser for {}", url))?;

   Ok(())
}
