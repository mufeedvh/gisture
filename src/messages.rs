use colored::*;

/// Logging message types
pub enum Type {
    Warning,
    Error,
    Info,
    Success,
}

/*
    should've just used `https://github.com/dtolnay/thiserror` or something, but it's "not bad" okay?
    I wanted this format on messages to provide a CLI experience that doesn't look like stack traces
    and it's just some `Ok`, `Err`, `exit(1)` over and over again... yeah.
*/

/// Outputs logging messages
pub fn push_message(log_type: Type, message: &str) {
    let out: String;
    
    match log_type {
        Type::Warning => {
            let prefix = format!("{}{}{}", "[".bold(), "WARN".bold().yellow(), "]".bold());
            out = format!("{} {}", prefix, message);
        }
        Type::Error => {
            let prefix = format!("{}{}{}", "[".bold(), "ERROR".bold().red(), "]".bold());
            out = format!("{} {}", prefix, message);
        }
        Type::Info => {
            let prefix = format!("{}{}{}", "[".bold(), "INFO".bold().cyan(), "]".bold());
            out = format!("{} {}", prefix, message);
        }
        Type::Success => {
            let prefix = format!("{}{}{}", "[".bold(), "SUCCESS".bold().green(), "]".bold());
            out = format!("{} {}", prefix, message);
        }
    }

    eprintln!("{}", out)
}
