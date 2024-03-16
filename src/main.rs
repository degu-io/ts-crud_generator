use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use clap::Parser;

fn read_template_file(file_path: &str) -> std::io::Result<String> {
    fs::read_to_string(file_path)
}

fn replace_placeholders(template: &str, entity_name: &str) -> String {
    let replace_string = entity_name.to_lowercase();
    template.replace("{{ENTITY_NAME}}", &replace_string)
}

fn write_to_file(file_name: &str, content: &str) -> std::io::Result<()> {
    let path = Path::new(file_name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    entity: String,
}

fn main() {
    let args = Args::parse();
    let template_path = "templates/trpc-drizzle.ts";
    let entity_name = &args.entity; 
    let output_file_name = format!("build/{}_trpc_router.ts", entity_name.to_lowercase());

    println!("Generating CRUD for entity: {}", entity_name);

    let template_content = read_template_file(template_path)
        .expect("Failed to read template file");

    let processed_content = replace_placeholders(&template_content, entity_name);

    write_to_file(&output_file_name, &processed_content)
        .expect("Failed to write output file");

    print!("Generated file: {}", output_file_name);

}
