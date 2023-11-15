mod commands {
    pub mod dev;
}
mod setup {
    pub mod dotfiles;
}
use clap::Parser;
mod types;

fn main() {
    let args = types::Args::parse();
    if args.dev.as_ref().is_some() {
        commands::dev::got_dev(args)
    } else if args.setup.is_some() {
        if args.setup.as_ref().unwrap() == "dotfiles" {
            setup::dotfiles::setup_dotfiles(args)
        }
    }
}
