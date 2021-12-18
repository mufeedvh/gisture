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
    let prefix = match log_type {
        Type::Warning => format!("{}{}{}", "[".bold(), "WARN".bold().yellow(), "]".bold()),
        Type::Error => format!("{}{}{}", "[".bold(), "ERROR".bold().red(), "]".bold()),
        Type::Info => format!("{}{}{}", "[".bold(), "INFO".bold().cyan(), "]".bold()),
        Type::Success => format!("{}{}{}", "[".bold(), "SUCCESS".bold().green(), "]".bold())
    };

    eprintln!("{}", format!("{} {}", prefix, message))
}
