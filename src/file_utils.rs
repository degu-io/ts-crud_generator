use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

pub fn read_template_file(file_path: &str) -> std::io::Result<String> {
  fs::read_to_string(file_path)
}

pub fn replace_placeholders(template: &str, entity_name: &str) -> String {
  let replace_string = entity_name.to_lowercase();
  template.replace("{{ENTITY_NAME}}", &replace_string)
}

pub fn write_to_file(file_name: &str, content: &str) -> std::io::Result<()> {
  let path = Path::new(file_name);
  if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
  }

  let mut file = File::create(file_name)?;
  file.write_all(content.as_bytes())?;
  Ok(())
}
