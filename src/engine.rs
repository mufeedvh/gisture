use clap::ArgMatches;

use crate::{
    config::Config,
    gist::GistApi,
    messages::{push_message, Type},
    metadata::Utils,
    serve,
    template::TemplateWriter,
};

use std::process::exit;

pub async fn init(args: ArgMatches<'_>) {
    // set up config
    Config::generate_default();
    TemplateWriter::generate_boilerplate();

    push_message(Type::Info, "Setting up configuration files and boilerplate templates.");

    // get user command
    let feature: Option<&str> = args.value_of("COMMAND");

    match feature {
        Some("build") => {
            let page_map = GistApi::get_all_blogs();
            let config = Config::get_config();

            TemplateWriter::render_templates(&page_map);
            Utils::generate_site_metadata(&page_map, &config);

            push_message(Type::Success, "Your gisture blog is ready to ship. 🚀")
        }
        Some("serve") => {
            let port: u16 = match args.value_of("PORT") {
                Some(port) => match port.parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => {
                        push_message(Type::Error, "PORT should be a number.");
                        exit(1)
                    }
                },
                None => {
                    let port = portpicker::pick_unused_port().expect("No ports free");
                    let message = format!(
                        "PORT was not provided (picked a random free port: {}).",
                        port
                    );
                    push_message(Type::Warning, &message);
                    port
                }
            };
            // `async` just for the serve...
            serve::serve(port).await
        }
        _ => {
            push_message(Type::Info, "Change the username and metadata in `gisture.json` to get started.");
            push_message(Type::Info, "The `templates` directory contains a basic starter template for your blog, customize it to your own needs.");
            push_message(Type::Warning, "Expected command `build` or `serve`.")
        }
    }
}
