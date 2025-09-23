use std::path::PathBuf;

pub fn expand_dir(path: PathBuf) -> PathBuf {
   let mut path = path;

   if path.starts_with("~") {
      path = std::env::home_dir()
         .unwrap()
         .join(path.strip_prefix("~").unwrap());
   }

   if path.is_relative() {
      path = std::env::current_dir().unwrap().join(path);
   }

   path
}
