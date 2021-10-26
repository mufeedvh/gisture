use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;
use std::string::String;

use handlebars::Handlebars;
use html_minifier::HTMLMinifier;
use serde_json::json;

use futures::executor::block_on;

use crate::{
    cache::Cache,
    config::Config,
    gist::GistPage,
    messages::{push_message, Type},
    parsers::ParserUtils,
};

// directory to save rendered pages
static RENDERED_DIR: &'static str = "public";

pub(crate) struct TemplateWriter;

impl TemplateWriter {
    /// Read a template boilerplate file
    fn read_file(filename: &str) -> String {
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(error) => {
                let message = format!(
                    "Failed to open template file `{}` due to: \n\t{}",
                    filename, error
                );
                push_message(Type::Error, &message);
                exit(1)
            }
        };

        let mut buf_reader = BufReader::new(file);
        let mut file_content = String::new();
        match buf_reader.read_to_string(&mut file_content) {
            Ok(_) => (),
            Err(error) => {
                let message = format!(
                    "Failed while reading template file `{}` due to: \n\t{}",
                    filename, error
                );
                push_message(Type::Error, &message);
                exit(1)
            }
        }

        file_content
    }

    /// To validate directory structure
    fn prepare(path: &str) {
        if !Path::new(path).exists() {
            match fs::create_dir_all(path) {
                Ok(()) => (),
                Err(error) => {
                    let message = format!(
                        "Failed to create the directory to save template files: \n\t{}",
                        error
                    );
                    push_message(Type::Error, &message);
                    exit(1)
                }
            }
        }
    }

    /// Save a rendered template
    fn save_file(filename: &str, content: &[u8], page: bool) {
        let template_path = format!("{}/{}", RENDERED_DIR, filename);

        let file_path = match page {
            true => {
                Self::prepare(&template_path);
                format!("{}/index.html", template_path)
            }
            false => template_path,
        };

        let mut template = match File::create(file_path) {
            Ok(file) => file,
            Err(error) => {
                let message = format!("Failed to create template file: \n\t{}", error);
                push_message(Type::Error, &message);
                exit(1)
            }
        };

        match template.write_all(content) {
            Ok(()) => (),
            Err(error) => {
                let message = format!("Failed to write rendered template: \n\t{}", error);
                push_message(Type::Error, &message);
                exit(1)
            }
        }
    }

    /// Minify the HTML document
    fn minify_html(html: String) -> String {
        let mut html_minifier = HTMLMinifier::new();

        match html_minifier.digest(&html) {
            Ok(()) => match String::from_utf8(html_minifier.get_html().into()) {
                Ok(html) => html,
                Err(_) => {
                    push_message(
                        Type::Warning,
                        "Encountered an invalid UTF-8 file. (SKIPPED)",
                    );
                    html
                }
            },
            Err(error) => {
                let message = format!("Failed to minify HTML due to \n\t{}.", error);
                push_message(Type::Warning, &message);
                html
            }
        }
    }

    /// Render a list of all gist entries to inject as HTML elements
    fn render_blog_list(page_map: &HashMap<String, GistPage>) -> String {
        // read blog listing template
        let page_list_template = Self::read_file("templates/page_list.html");

        // html writer
        let mut page_listing_html = String::new();

        let handlebars_reg = Handlebars::new();

        for (page, page_data) in page_map {
            let template_variables = &json!(
                {
                    "page_title": page_data.title,
                    "page_description": page_data.description,
                    "page_url": format!("/{}", page),
                    "published_date": page_data.created_at
                }
            );

            let rendered_listing: String = match handlebars_reg
                .render_template(&page_list_template, template_variables)
            {
                Ok(html) => html,
                Err(error) => {
                    let message = format!("Failed to render Handlebars template: \n\t{}", error);
                    push_message(Type::Error, &message);
                    exit(1)
                }
            };

            page_listing_html = format!("{}\n{}", page_listing_html, rendered_listing);
        }

        page_listing_html
    }

    /// Generate default boilerplate templates
    pub fn generate_boilerplate() {
        if !Path::new("templates").exists() {
            Self::prepare("templates");

            let mut default_templates: HashMap<&str, &[u8]> = HashMap::with_capacity(5);
            default_templates.insert("404.html", include_bytes!("../templates/404.html"));
            default_templates.insert("comments.html", include_bytes!("../templates/comments.html"));
            default_templates.insert("index.html", include_bytes!("../templates/index.html"));
            default_templates.insert("page_list.html", include_bytes!("../templates/page_list.html"));
            default_templates.insert("page.html", include_bytes!("../templates/page.html"));
        
            for (file, content) in default_templates {
                let mut template = match File::create(format!("templates/{}", file)) {
                    Ok(file) => file,
                    Err(error) => {
                        let message = format!("Failed to create boilerplate template file: \n\t{}", error);
                        push_message(Type::Error, &message);
                        exit(1)
                    }
                };
        
                match template.write_all(content) {
                    Ok(()) => (),
                    Err(error) => {
                        let message = format!("Failed to generate boilerplate template: \n\t{}", error);
                        push_message(Type::Error, &message);
                        exit(1)
                    }
                }
            }
        }
    }

    /// Render and build all templates with the boilerplate HTML
    pub fn render_templates(page_map: &HashMap<String, GistPage>) {
        // prepare the directory to save rendered templates
        Self::prepare(RENDERED_DIR);

        let config = Config::get_config();

        let handlebars_reg = Handlebars::new();

        // render index page
        let blog_listing = Self::render_blog_list(&page_map); // generate blog listing

        let template_variables = &json!(
            {
                "blog_title": config.blog_title,
                "blog_description": config.blog_description,
                "blog_url": config.blog_url,
                "blog_list": blog_listing
            }
        );

        let index_template = Self::read_file("templates/index.html");

        let index_html: String =
            match handlebars_reg.render_template(&index_template, template_variables) {
                Ok(html) => {
                    if config.minify_html {
                        Self::minify_html(html)
                    } else {
                        html
                    }
                }
                Err(error) => {
                    let message = format!("Failed to render Handlebars template: \n\t{}", error);
                    push_message(Type::Error, &message);
                    exit(1)
                }
            };

        Self::save_file("index.html", index_html.as_bytes(), false);

        // render pages
        let page_template = Self::read_file("templates/page.html");

        for (page, page_data) in page_map {
            let blog_cache: Cache = Cache {
                permalink_key: page.to_string(),
                updated_at: page_data.updated_at.clone(),
            };

            match block_on(Cache::is_cached(&blog_cache)) {
                Ok(true) => {
                    let message = format!("Skipped entry \"{}\" (exists in disk cache).", page);
                    push_message(Type::Info, &message);
                    continue;
                }
                _ => {
                    // each pages title has to be rendered according to configured formatting
                    let page_title = match handlebars_reg.render_template(
                        &config.pages_title,
                        &json!({
                            "blog_title": page_data.title
                        }),
                    ) {
                        Ok(rendered_title) => rendered_title,
                        Err(error) => {
                            let message =
                                format!("Failed to render Handlebars template: \n\t{}", error);
                            push_message(Type::Error, &message);
                            exit(1)
                        }
                    };

                    let comment_section = format!(
                        "<br>\n\t<a href=\"{}\" style=\"color: blue;\">Read comments for this gist</a>",
                        page_data.html_url
                    );

                    let template_variables = &json!(
                        {
                            "blog_title": config.blog_title,
                            "blog_description": config.blog_description,
                            "blog_url": config.blog_url,
                            "page_title": page_title,
                            "page_description": page_data.description,
                            "page_url": ParserUtils::join_url_path(&config.blog_url, page),
                            "published_date": page_data.created_at,
                            "updated_at": page_data.updated_at,
                            "blog_contents": page_data.content,
                            "comment_section": comment_section
                        }
                    );

                    let rendered_page =
                        match handlebars_reg.render_template(&page_template, template_variables) {
                            Ok(rendered_page) => {
                                if config.minify_html {
                                    Self::minify_html(rendered_page)
                                } else {
                                    rendered_page
                                }
                            }
                            Err(error) => {
                                let message =
                                    format!("Failed to render Handlebars template: \n\t{}", error);
                                push_message(Type::Error, &message);
                                exit(1)
                            }
                        };

                    block_on(Cache::save_cache_entry(&Cache {
                        permalink_key: page.to_string(),
                        updated_at: page_data.updated_at.clone(),
                    }))
                    .ok();

                    Self::save_file(&page, rendered_page.as_bytes(), true)
                }
            }
        }
    }
}
