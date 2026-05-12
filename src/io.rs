use std::fmt::Display;
use serde_json::json;

pub struct IOFlags {
    pub stdout: bool,
    pub json: bool,
    pub only_result: bool
}

pub fn print_result<T, E>(result: Result<T, E>, flags: &IOFlags)
where
    T: Display,
    E: Display,
{
    let output = if flags.json {
        let (status, message) = match &result {
            Ok(t) => ("Ok", format!("{t}")),
            Err(e) => ("Err", format!("{e}")),
        };
        json!({ "status": status, "message": message }).to_string()
    } else {
        match &result {
            Ok(t) => format!("Ok: {t}"),
            Err(e) => format!("Err: {e}"),
        }
    };

    if result.is_ok() || flags.stdout {
        println!("{output}")
    } else {
        eprintln!("{output}")
    }
}