use std::path::Path;
use std::process::exit;

use crate::messages::{push_message, Type};

/// Start a web server to preview the generated blog
pub async fn serve(port: u16) {
    if !Path::new("public").exists() {
        push_message(
            Type::Error,
            "Make sure you're running `serve` in your blog's directory \
            and the `public` directory has been generated.",
        );
        exit(1)
    }

    push_message(
        Type::Success,
        &format!("Gisture server is running on http://127.0.0.1:{}/ 🚀", port),
    );

    if portpicker::is_free(port) {
        warp::serve(warp::fs::dir("public"))
            .run(([127, 0, 0, 1], port))
            .await;
    } else {
        let message = format!("PORT {} is not free.", port);
        push_message(Type::Error, &message);
        exit(1)
    }
}
