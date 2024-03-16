use crate::file_utils;
use file_utils::{read_template_file, replace_placeholders, write_to_file};


fn create_from_template(entity_name: &str, template_path: &str, output_file_name: String) {
  let template_content = read_template_file(template_path)
      .expect("Failed to read template file");

  let processed_content = replace_placeholders(&template_content, entity_name);

  write_to_file(&output_file_name, &processed_content)
      .expect("Failed to write output file");

  print!("Generated file: {}", output_file_name);  
}

pub fn create_trpc_route(entity_name: &str) {
  let template_path = "templates/trpc-drizzle.ts";
  let output_file_name = format!("build/{}_trpc_router.ts", entity_name.to_lowercase());
  
  create_from_template(entity_name, template_path, output_file_name); 
}

