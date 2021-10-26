mod cache;
mod cli;
mod config;
mod engine;
mod gist;
mod messages;
mod metadata;
mod parsers;
mod serve;
mod template;

#[tokio::main]
async fn main() {
    // a banner to look cool
    eprintln!(include_str!(".extras/banner"));
    // start the engines
    engine::init(cli::interface()).await
}
