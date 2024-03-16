mod file_utils;
mod templator;
use templator::create_trpc_route;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    entity: String,
}

fn main() {
    let args = Args::parse();
    let entity_name = &args.entity; 

    create_trpc_route(entity_name)
}
