use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use console::Style;
use failure::Error;
use serde_json;

fn find_log_file(cwd: PathBuf) -> Result<PathBuf, Error> {
    // TODO: read `config/config.php` to support custom data dirs
    let server_path = cwd.join("data/nextcloud.log");
    let app_dir_path = cwd.join("../data/nextcloud.log");
    let app_path = cwd.join("../../data/nextcloud.log");

    if server_path.exists() {
        Ok(server_path)
    } else if app_dir_path.exists() {
        Ok(app_dir_path)
    } else if app_path.exists() {
        Ok(app_path)
    } else {
        bail!("could not find nextcloud.log");
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct LoggedException {
    Code: usize,
    Message: String,
    Trace: Vec<StackFrame>,
}

#[derive(Deserialize)]
struct StackFrame {
    class: Option<String>,
    function: Option<String>,
    file: Option<String>,
    line: Option<usize>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct LogLine {
    reqId: String,
    level: usize,
    // time: String,
    remoteAddr: String,
    user: String,
    app: String,
    method: String,
    url: String,
    message: serde_json::Value,
}

fn start_tail_process(path: PathBuf, follow: bool) -> Result<Child, Error> {
    if follow {
        Command::new("tail")
            .arg("-n")
            .arg("100")
            .arg("-f")
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| e.into())
    } else {
        Command::new("tail")
            .arg("-n")
            .arg("100")
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| e.into())
    }
}

fn parse_message(msg: serde_json::Value, style: &Style) -> Result<String, Error> {
    if let Ok(exception) = serde_json::from_value::<LoggedException>(msg.clone()) {
        let mut message = String::new();

        message.push_str(&format!("Exception: {} (Code {})\n", exception.Message, exception.Code));
        for frame in exception.Trace {
            let func = match (frame.class, frame.function) {
                (Some(class), Some(function)) => format!("{}::{}", style.apply_to(class), function),
                (None, Some(function)) => format!("{}", function),
                _ => format!(""),
            };
            message.push_str(&format!("         at {}\n", func));

            match (frame.file, frame.line) {
                (Some(file), Some(line)) => message.push_str(&format!("            {}:{}\n", style.apply_to(file), line)),
                (Some(file), None) => message.push_str(&format!("            {}\n", file)),
                _ => (),
            };
        }

        Ok(message)
    } else if let serde_json::Value::String(string) = msg {
        Ok(string)
    } else {
        bail!("could not parse log message");
    }
}

fn parse_line(original: String) -> Result<String, Error> {
    let parsed = serde_json::from_str::<LogLine>(&original)?;
    let (level_str, style) = match parsed.level {
        0 => ("DEBUG", Style::new().white()),
        1 => (" INFO", Style::new().cyan()),
        2 => (" WARN", Style::new().yellow()),
        3 => ("ERROR", Style::new().magenta()),
        4 => ("FATAL", Style::new().red()),
        _ => ("     ", Style::new().white()),
    };

    let message = parse_message(parsed.message, &style)?;

    Ok(format!(
        "{}: {}\n       {} {}\n       App: {}\n       Client: {}\n       Request: {}\n       User: {}\n",
        style.apply_to(level_str),
        style.apply_to(message),
        parsed.method,
        parsed.url,
        parsed.app,
        parsed.remoteAddr,
        parsed.reqId,
        parsed.user
    ))
}

pub fn logs<P>(cwd: P, follow: bool) -> Result<(), Error>
where
    P: Into<PathBuf>,
{
    let cwd = cwd.into();
    let file_path = find_log_file(cwd)?;

    let mut tail_p = start_tail_process(file_path, follow)?;
    for line in BufReader::new(tail_p.stdout.take().unwrap()).lines() {
        let line = line?;
        let parsed = parse_line(line)?;
        println!("{}", parsed);
    }

    tail_p.wait()?;

    Ok(())
}
