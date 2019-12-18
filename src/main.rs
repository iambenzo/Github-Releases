extern crate clap;
use clap::{App, Arg, SubCommand};

mod download;

fn main() {
    let matches = App::new("GitHub Releases (GHR)")
        .version("0.1.0")
        .author("iAmBenzo")
        .about("Perform installations and updates from GitHub releases")
        .subcommand(
            SubCommand::with_name("download")
                .about("Download a file")
                .version("0.1.0")
                .author("iAmBenzo")
                .arg(
                    Arg::with_name("repo")
                        .help("location of file")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("download") {
        let repo = matches.value_of("repo").unwrap();
        println!("Downloading {}", &repo);
        if let Err(e) = download::download_file(&repo) {
            eprintln!("Application error: {}", e);
            std::process::exit(1);
        }
    }
}
