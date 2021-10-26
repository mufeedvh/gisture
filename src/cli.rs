use clap::{App, Arg, ArgMatches};

pub fn interface() -> ArgMatches<'static> {
    App::new("gisture")
        .version("0.1.0")
        .author("Mufeed VH <contact@mufeedvh.com>")
        .about("A minimal and light-weight blog generator based on GitHub gists.")
        .arg(Arg::with_name("COMMAND")
            .help("`build` compiles all your gists to HTML pages.\n`serve` launches a web server to preview the generated pages.")
            .value_name("build | serve")
            .required(false)
            .index(1))
        .arg(Arg::with_name("PORT")
            .help("PORT to run the preview server on (comes after the `serve` command).")
            .value_name("PORT")
            .required(false)
            .index(2))
        .get_matches()
}
