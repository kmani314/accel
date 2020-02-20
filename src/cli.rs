extern crate clap;
use clap::{App, ArgMatches, AppSettings, Arg};

pub fn run_cli() -> ArgMatches<'static> {
    App::new("accel")
        .version("0.1")
        .author("Krishna Mani <krishnamani314@gmail.com>")
        .about("Fast defintion resolver and flashcard generator")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        // Solves term lists
        .subcommand(
            App::new("solve")
            .about("Defines terms and creates flashcards")
            .arg(
                Arg::with_name("input")
                .index(1)
                .required(true)
            )
            .arg(
                Arg::with_name("output")
                .short("o")
                .long("output")
                .required(false)
                .default_value("deck.csv")
            )
            .arg(
                Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .takes_value(true)
                .required(true)
            )
            .arg(
                Arg::with_name("source")
                .short("s")
                .long("source")
                .takes_value(true)
                .required(true)
                .multiple(true)
            )
        )
        .get_matches()
}
