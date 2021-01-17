use clap::{App, ArgMatches, SubCommand};

pub fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("Borz")
        .version("0.1")
        .about("A command line interface for the Borz social networking platform")
        .subcommand(
            SubCommand::with_name("clean").about("Deletes all configuration and cache files"),
        )
        .subcommand(SubCommand::with_name("login").about("Logs in to your Borz account"))
        .subcommand(SubCommand::with_name("logout").about("Logs out of your account"))
        .subcommand(SubCommand::with_name("signup"))
        .about("Creates a new Borz account")
        .subcommand(SubCommand::with_name("verify"))
        .about("Verifies the email of a new account")
        .get_matches()
}
