use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use sitemap::structs::UrlEntry;
use sitemap::writer::SiteMapWriter;

use chrono::prelude::*;

use crate::{
    config::Config,
    gist::GistPage,
    messages::{push_message, Type},
    parsers::ParserUtils,
};

pub(crate) struct Utils;

impl Utils {
    pub fn generate_site_metadata(page_map: &HashMap<String, GistPage>, config: &Config) {
        Self::generate_robots_txt(config);
        Self::generate_sitemap(page_map, config);
    }

    /// Generate a robots.txt file for search engine crawlers (SEO)
    fn generate_robots_txt(config: &Config) {
        let robots_txt = format!(
            "User-agent: Mediapartners-Google\nDisallow:\n\nUser-agent: *\nAllow: /\n\nSitemap: {}",
            ParserUtils::join_url_path(&config.blog_url, "sitemap.xml")
        );

        let mut robots_file = match File::create("public/robots.txt") {
            Ok(file) => file,
            Err(error) => {
                let message = format!("Failed to create `robots.txt` file due to: \n\t{}", error);
                push_message(Type::Error, &message);
                exit(1)
            }
        };

        match robots_file.write_all(robots_txt.as_bytes()) {
            Ok(()) => (),
            Err(error) => {
                let message = format!("Failed to write `robots.txt` file due to: \n\t{}", error);
                push_message(Type::Error, &message);
                exit(1)
            }
        }
    }

    /// Generate a XML sitemap of the blog structure (SEO)
    fn generate_sitemap(page_map: &HashMap<String, GistPage>, config: &Config) {
        let mut sitemap_xml_buffer = match File::create("public/sitemap.xml") {
            Ok(buffer) => buffer,
            Err(error) => {
                let message = format!("Failed to create `sitemap.xml` file due to: \n\t{}", error);
                push_message(Type::Error, &message);
                exit(1)
            }
        };

        let sitemap_writer = SiteMapWriter::new(&mut sitemap_xml_buffer);
        let mut urlwriter = match sitemap_writer.start_urlset() {
            Ok(urlwriter) => urlwriter,
            Err(error) => {
                let message = format!(
                    "Failed to write `urlset` to `sitemap.xml` file due to: \n\t{}.",
                    error
                );
                push_message(Type::Error, &message);
                exit(1)
            }
        };

        let blog_url = &config.blog_url;

        // write blog index url
        match urlwriter.url(blog_url.clone()) {
            Ok(()) => (),
            Err(_) => {
                push_message(
                    Type::Error,
                    "Failed to write `blog_url` from config to `sitemap.xml`.",
                );
                exit(1)
            }
        }

        for (page, page_data) in page_map {
            let lastmod_datetime = match page_data.updated_at.parse::<DateTime<FixedOffset>>() {
                Ok(parsed) => parsed,
                Err(error) => {
                    let message = format!(
                        "Failed to parse `{}` to `DateTime` format due to: \n\t{}.",
                        page_data.updated_at, error
                    );
                    push_message(Type::Error, &message);
                    exit(1)
                }
            };

            let url_builder = match UrlEntry::builder()
                .loc(ParserUtils::join_url_path(&blog_url, page))
                .lastmod(lastmod_datetime)
                .build()
            {
                Ok(url_builder) => url_builder,
                Err(error) => {
                    let message = format!("Failed to build `sitemap.xml` due to: \n\t{}", error);
                    push_message(Type::Error, &message);
                    exit(1)
                }
            };

            match urlwriter.url(url_builder) {
                Ok(()) => (),
                Err(error) => {
                    let message = format!(
                        "Failed to write `{}` to URL set of `sitemap.xml` due to: \n\t{}.",
                        page, error
                    );
                    push_message(Type::Error, &message);
                    exit(1)
                }
            }
        }

        match urlwriter.end() {
            Ok(_) => (),
            Err(error) => {
                let message = format!(
                    "Failed to close tags for `sitemap.xml` due to: \n\t{}.",
                    error
                );
                push_message(Type::Error, &message);
                exit(1)
            }
        }
    }
}
