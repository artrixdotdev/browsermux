use std::{env, fs};

include!("src/settings.rs");

fn main() {
   let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

   // Builds to target/release/schema.json when built with --release
   let profile = env::var("PROFILE").unwrap(); // "debug" or "release"
   let out_dir = manifest_dir.join("target").join(&profile);

   let schema = schemars::schema_for!(Settings);
   let schema = serde_json::to_string_pretty(&schema).unwrap();

   let path = out_dir.join("schema.json");
   fs::write(&path, schema).unwrap();

   println!("cargo:warning=schema file is generated at {path:?}");
}
