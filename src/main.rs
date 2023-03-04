use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, value_name="Defender")]
   engine: Option<String>,

   #[arg(short, long)]
   file: Option<String>,

   #[arg(short, long)]
   url: Option<String>,
}

fn main() {
   let args = Args::parse();

   if let Some(engine) = args.engine.as_deref() {
    println!("Value for engine: {engine}");
}
}