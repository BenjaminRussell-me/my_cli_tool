use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub dev: Option<String>,
    #[arg(short, long)]
    pub action: Option<String>,
    #[arg(short, long)]
    pub setup: Option<String>,
}
