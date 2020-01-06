extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("GitHub Releases (GHR)")
        .version("0.1.0")
        .author("Ben Burbage <ben@iambenzo.com>")
        .about("Perform installations and updates from GitHub releases.")
        .subcommand(
            SubCommand::with_name("install")
                .about("Downloads latest release and starts tracking repository.")
                .version("0.1.0")
                .author("Ben Burbage <ben@iambenzo.com>")
                .arg(
                    Arg::with_name("repo")
                        .help("GitHub Repository in quotes e.g. \"iambenzo/vscode-theme-nyx\"")
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
                )
                .arg(
                    Arg::with_name("install-script")
                        .help("Script/Command to use after source has been downloaded. e.g. \"sh install.sh\"")
                        .short("i")
                        .long("install-script")
                        .takes_value(true)
                        .multiple(false)
                        .required(false),
                )
                .arg(
                    Arg::with_name("update-script")
                        .help("Script/Command to use after source has been updated. e.g. \"sh update.sh\"")
                        .short("u")
                        .long("update-script")
                        .takes_value(true)
                        .multiple(false)
                        .required(false),
                )
                .arg(
                    Arg::with_name("remove-script")
                        .help("Script/Command to remove installed source during removal. e.g. \"sh remove.sh\"")
                        .short("r")
                        .long("remove-script")
                        .takes_value(true)
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
            SubCommand::with_name("remove")
                .about("Uninstalls and stops tracking a repository.")
                .version("0.1.0")
                .author("Ben Burbage <ben@iambenzo.com>")
                .arg(
                    Arg::with_name("repo")
                        .help("GitHub Repository in quotes e.g. \"iambenzo/vscode-theme-nyx\"")
                        .index(1)
                        .required(true),
                )
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

        let install_script = match matches.value_of("install-script") {
            Some(x) => x,
            None => "",
        };

        let update_script = match matches.value_of("update-script") {
            Some(x) => x,
            None => "",
        };

        let remove_script = match matches.value_of("remove-script") {
            Some(x) => x,
            None => "",
        };

        // println!("install_script: {}", &install_script);

        let repo = matches.value_of("repo").unwrap();
        println!("Installing {}...", &repo);
        if let Err(e) = ghr::install(
            &repo,
            pre_release,
            install_script,
            update_script,
            remove_script,
        ) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    if let Some(_matches) = matches.subcommand_matches("update") {
        println!("Updating sources...");
        if let Err(e) = ghr::update() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let repo = matches.value_of("repo").unwrap();
        println!("Removing {}...", &repo);
        if let Err(e) = ghr::remove(&repo) {
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
