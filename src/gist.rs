use std::collections::HashMap;
use std::process::exit;
use std::time::Duration;

use serde_json::{Map, Value};
use ureq::{Agent, AgentBuilder};

use crate::{
    config::Config,
    messages::{push_message, Type},
    parsers::ParserUtils,
};

#[derive(Debug, Clone)]
pub(crate) struct GistPage {
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub content: String,
    pub html_url: String,
}

pub(crate) struct GistApi;

impl GistApi {
    /// Fetch user's gist entries
    fn get_user_gists(conn_pool: &Agent) -> Vec<Value> {
        let username = Config::get_config().github_username;
        let endpoint = format!("https://api.github.com/users/{}/gists", username);
        let user_gists: String = match conn_pool
            .get(&endpoint)
            .set("Accept", "application/vnd.github.v3+json")
            .call()
        {
            Ok(response) => match response.into_string() {
                Ok(json_response) => json_response,
                Err(error) => {
                    let message = format!(
                        "Failed while encoding response from Gist API: \n\t{}",
                        error
                    );
                    push_message(Type::Error, &message);
                    exit(1);
                }
            },
            Err(error) => {
                let message = format!("Couldn't send HTTP request to Gist API: \n\t{}", error);
                push_message(Type::Error, &message);
                exit(1);
            }
        };

        match serde_json::from_str(&user_gists) {
            Ok(gist_vec) => gist_vec,
            Err(error) => {
                let message = format!(
                    "Failed to serialize JSON response from Gist API: \n\t{}",
                    error
                );
                push_message(Type::Error, &message);
                exit(1);
            }
        }
    }

    /// Fetch raw markdown for a particular gist entry
    fn get_gist_markdown(conn_pool: &Agent, url: &str) -> String {
        match conn_pool.get(url).call() {
            Ok(response) => match response.into_string() {
                Ok(markdown) => markdown,
                Err(error) => {
                    let message = format!(
                        "Failed while encoding response from Gist API: \n\t{}",
                        error
                    );
                    push_message(Type::Error, &message);
                    exit(1);
                }
            },
            Err(error) => {
                let message = format!("Couldn't send HTTP request to Gist API: \n\t{}", error);
                push_message(Type::Error, &message);
                exit(1);
            }
        }
    }

    /// Organize a blog map of all gist entries
    pub fn get_all_blogs() -> HashMap<String, GistPage> {
        push_message(Type::Info, "Fetching gist schema.");

        // register a ureq agent (connection pool)
        let conn_pool: Agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();

        let user_gists = Self::get_user_gists(&conn_pool);

        // to store all blog's permalink and their contents
        let mut blogs: HashMap<String, GistPage> = HashMap::new();

        for gist in user_gists {
            let gist_files: Map<String, Value> =
                match serde_json::from_str(&gist["files"].to_string()) {
                    Ok(gist_files) => gist_files,
                    Err(error) => {
                        let message =
                            format!("Failed while scraping files inside Gist: \n\t{}", error);
                        push_message(Type::Error, &message);
                        exit(1);
                    }
                };

            for (file, metadata) in gist_files {
                // gisture files should end with `blog.md`, this is to distinguish from other gists
                if file.ends_with("blog.md") {
                    // url to get the the raw markdown
                    let raw_url = metadata["raw_url"].to_string().replace("\"", "");

                    // get raw markdown of gist
                    let markdown_content = Self::get_gist_markdown(&conn_pool, &raw_url);

                    // parse blog title from markdown (`# Title`)
                    let page_title = match markdown_content.split("\n").nth(0) {
                        Some(title) => {
                            if title.contains("#") {
                                title.replace("#", "").trim().to_string()
                            } else {
                                let message = format!("Gist `{}` doesn't contain a title (`# Title`) on top. (SKIPPED)", file);
                                push_message(Type::Warning, &message);
                                continue;
                            }
                        }
                        None => {
                            let message = format!("Encountered empty Gist: `{}` (SKIPPED)", file);
                            push_message(Type::Warning, &message);
                            continue;
                        }
                    };

                    // permalink is the filename without the gisture markdown extension
                    let permalink = file.replace(".blog.md", "");

                    // convert gist Markdown to HTML
                    let html_content = ParserUtils::parse_markdown_to_html(markdown_content);

                    let page_data: GistPage = GistPage {
                        title: page_title,
                        description: gist["description"].to_string().replace("\"", ""),
                        created_at: gist["created_at"].to_string().replace("\"", ""),
                        updated_at: gist["updated_at"].to_string().replace("\"", ""),
                        html_url: gist["html_url"].to_string().replace("\"", ""),
                        content: html_content,
                    };

                    // save blog with it's raw markdown
                    blogs.insert(permalink, page_data);

                    let message = format!("Fetched blog \"{}\".", file);
                    push_message(Type::Info, &message);
                }
            }
        }

        // if there are no gisture blogs, why should I live any longer?
        if blogs.len() == 0 {
            let username = Config::get_config().github_username;
            let message = format!("0 gisture blogs (*.blog.md) found for user '{}'", username);
            push_message(Type::Warning, &message);
            exit(0)
        }

        blogs
    }
}
