use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::process::exit;

use pulldown_cmark::{html, Options, Parser};

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use html_escape::{encode_text, decode_html_entities};
use lol_html::html_content::ContentType;
use lol_html::{rewrite_str, text, RewriteStrSettings};

use crate::messages::{push_message, Type};

pub(crate) struct ParserUtils;

impl ParserUtils {
    /// Highlight source code snippets in the parsed HTML
    fn highlight_source_code(html: &str) -> String {
        let mut lang_set: Vec<String> = Vec::new();

        // the lazy but quick html parsing
        for segment in html.split("\n") {
            if segment.contains("<code class=\"language-") {
                for item in segment.split("\"") {
                    if item.contains("language") {
                        lang_set.push(item.replace("language-", ""));
                    }
                }
            } else if segment.contains("<code>") {
                lang_set.push("txt".into())
            }
        }

        if lang_set.len() != 0 {
            static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| {
                let mut syntax_set = SyntaxSet::load_defaults_newlines().into_builder();
                syntax_set.add_from_folder("syntaxes", true).ok();
                syntax_set.build()
            });

            let theme_set = ThemeSet::load_defaults();
            let theme = &theme_set.themes["InspiredGitHub"];

            let mut code_elem_idx = 0;

            let rewrite = match rewrite_str(
                &html.replace("&quot;", "\""),
                RewriteStrSettings {
                    element_content_handlers: vec![text!("code", |chunk| {
                        if code_elem_idx > lang_set.len() - 1 {
                            let escape = encode_text(chunk.as_str()).to_string();
                            chunk.replace(&escape, ContentType::Text)
                        } else {
                            let lang_syntax =
                                match SyntaxSet::find_syntax_by_token(&SYNTAX_SET, &lang_set[code_elem_idx]) {
                                    Some(lang_syntax) => lang_syntax,
                                    None => {
                                        let message = format!(
                                            "No syntax highlighting spec found for `{}`.",
                                            &lang_set[code_elem_idx]
                                        );
                                        push_message(Type::Warning, &message);
                                        match SyntaxSet::find_syntax_by_token(&SYNTAX_SET, "txt") {
                                            Some(lang_syntax) => lang_syntax,
                                            _ => unreachable!()
                                        }
                                    }
                                };

                            chunk.replace(
                                &highlighted_html_for_string(
                                    &chunk.as_str(),
                                    &SYNTAX_SET,
                                    lang_syntax,
                                    theme,
                                ),
                                ContentType::Text,
                            );
                        };

                        code_elem_idx += 1;

                        Ok(())
                    })],
                    ..RewriteStrSettings::default()
                },
            ) {
                Ok(rewrite) => rewrite,
                Err(error) => {
                    let message = format!(
                        "Failed to highlight syntax inside HTML document due to: \n\t{}",
                        error
                    );
                    push_message(Type::Error, &message);
                    exit(1)
                }
            };

            // return a sanitized output
            // is there a better way to go about this? i suck at html.
            decode_html_entities(&rewrite)
                .replace("&amp;", "&") // encoding lives happily ever after
                .replace("<pre style=\"background-color:#ffffff;\">\n</pre>", "") // anomaly
                .replace("<code><pre style=\"background-color:#ffffff;\">", "<code>") // next anomaly
        } else {
            html.to_string()
        }
    }

    /// Converts gist's raw markdown to HTML
    pub fn parse_markdown_to_html(raw_markdown: String) -> String {
        static OPTIONS: Lazy<pulldown_cmark::Options> = Lazy::new(|| {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_SMART_PUNCTUATION);
            options.insert(Options::ENABLE_FOOTNOTES);
            options.insert(Options::ENABLE_TASKLISTS);
            options
        });

        let parser = Parser::new_ext(&raw_markdown, *OPTIONS);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Self::highlight_source_code(&html_output)
    }

    /// For handling trailing slashes on URLs
    pub fn join_url_path(url: &str, path: &str) -> String {
        if url.ends_with("/") {
            format!("{}{}", url, path)
        } else {
            format!("{}/{}", url, path)
        }
    }

    /// RFC 3339 date-time parsing and sorting
    pub fn _sort_by_date(_page_metadata: HashMap<String, String>) -> HashMap<i32, String> {
        /*
            This is not required atm because gist already outputs a sorted list.
            It's in descending order so we just traverse the list in reverse.
            If anyone asks for sorting configurations for their blog listing or
            the gist fetching goes multi-threaded this will be implemented.
        */
        unimplemented!();
    }
}
