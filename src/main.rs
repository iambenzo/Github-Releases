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
                        .help("location of file")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Checks and downloads updates for each tracked repository.")
                .version("0.1.0")
                .author("Ben Burbage <ben@iambenzo.com>")
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists tracked repositories.")
                .version("0.1.0")
                .author("Ben Burbage <ben@iambenzo.com>")
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        let repo = matches.value_of("repo").unwrap();
        // println!("Downloading {}", &repo);
        if let Err(e) = ghr::install(&repo) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    if let Some(_matches) = matches.subcommand_matches("update") {
        println!("Updating sources!");
    }

    if let Some(_matches) = matches.subcommand_matches("list") {
        println!("Tracked repositories:");
        if let Err(e) = ghr::list() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
