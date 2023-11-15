use crate::types;
use std::process::Command;

pub fn got_dev(args: types::Args) {
    if args.dev.is_none() {
        panic!("dev string not getting through")
    }
    struct Project {
        name: String,
        path: String,
    }
    let projects = vec![
        Project {
            name: String::from("supervisor"),
            path: String::from("/home/benjamin_work/Projects/retain/cb-retain-supervisor/"),
        },
        Project {
            name: String::from("participant"),
            path: String::from("/home/benjamin_work/Projects/retain/cb-retain-participant/"),
        },
        Project {
            name: String::from("participant_lib"),
            path: String::from("/home/benjamin_work/Projects/retain/libs/participant-ui-lib/"),
        },
    ];

    let mut selected = Project {
        name: String::from("placeholder"),
        path: String::from("placeholder"),
    };

    projects.into_iter().for_each(|project| {
        if &project.name == args.dev.as_ref().unwrap() {
            selected = project
        } else {
            panic!("name doesn't work")
        }
    });

    if args.action.as_ref().is_none() {
        Command::new("goto")
            .arg(&selected.path)
            .status()
            .expect("process failed");
        return;
    }
    if args.action.as_ref().unwrap() == "helix" {
        Command::new("hx")
            .current_dir(&selected.path)
            .status()
            .expect("process failed");
    }
    if args.action.as_ref().unwrap() == "run" {
        Command::new("npm run dev")
            .current_dir(&selected.path)
            .status()
            .expect("process failed");
    }
}
