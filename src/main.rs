extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("GitHub Releases (GHR)")
        .version("0.1.0")
        .author("iAmBenzo")
        .about("Perform installations and updates from GitHub releases")
        .subcommand(
            SubCommand::with_name("install")
                .about("Downloads latest release and adds repository to sources list.")
                .version("0.1.0")
                .author("Ben Burbage <ben@iambenzo.com>")
                .arg(
                    Arg::with_name("repo")
                        .help("location of file")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        let repo = matches.value_of("repo").unwrap();
        // println!("Downloading {}", &repo);
        if let Err(e) = ghr::install(&repo) {
            eprintln!("Application error: {}", e);
            std::process::exit(1);
        }
    }
}
