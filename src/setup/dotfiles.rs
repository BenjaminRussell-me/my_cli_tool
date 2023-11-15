use crate::types;
use std::process::Command;

pub fn setup_dotfiles(args: types::Args) {
    if args.setup.is_none() {
        panic!("setup string not getting through")
    }

    if args.setup.as_ref().unwrap() == "dotfiles" {
        Command::new("alias")
            .arg("dotfiles='/usr/bin/gin --git-dir=$HOME/.dotfiles/ --work-tree=$HOME'")
            .status()
            .expect("process failed");
        Command::new("git clone")
            .arg("--bare git@github.com:BenjaminRussell-me/dotfiles.git $HOME/.dotfiles")
            .status()
            .expect("process failed");
        Command::new("dotfiles config")
            .arg("--local status.showUntrackedFiles no")
            .status()
            .expect("process failed");
        Command::new("dotfiles checkout")
            .status()
            .expect("process failed");
    }
}
