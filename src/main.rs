extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("GitHub Releases (GHR)")
        .version("0.1.0")
        .author("iAmBenzo")
        .about("Perform installations and updates from GitHub releases.")
        .subcommand(
            SubCommand::with_name("install")
                .about("Downloads latest release and starts tracking repository.")
                .version("0.1.0")
                .author("Ben Burbage <ben@iambenzo.com>")
                .arg(
                    Arg::with_name("repo")
                        .help("GitHub Repository in quotes e.g. \"iambenzo\\vscode-theme-nyx\"")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("pre-release")
                        .help("Check this repo for pre-release content too")
                        .short("p")
                        .long("pre-release")
                        .multiple(false)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Checks and downloads updates for each tracked repository.")
                .version("0.1.0")
                .author("Ben Burbage <ben@iambenzo.com>"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists tracked repositories.")
                .version("0.1.0")
                .author("Ben Burbage <ben@iambenzo.com>"),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        let pre_release = {
            if matches.occurrences_of("pre-release") > 0 {
                true
            } else {
                false
            }
        };
        // println!("Pre-release used: {}", &pre_release);

        let repo = matches.value_of("repo").unwrap();
        // println!("Downloading {}", &repo);
        if let Err(e) = ghr::install(&repo, pre_release) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    if let Some(_matches) = matches.subcommand_matches("update") {
        println!("Updating sources!");
        if let Err(e) = ghr::update() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    if let Some(_matches) = matches.subcommand_matches("list") {
        println!("Tracked repositories:");
        if let Err(e) = ghr::list() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
