use anyhow::{anyhow, Context, Result};
use std::process::Command;

pub const RS: char = '\u{1E}';
pub const FS: char = '\u{1F}';

pub fn run_applescript(script: &str, args: &[String]) -> Result<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .arg("--")
        .args(args)
        .output()
        .context("failed to execute osascript")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let msg = if stderr.is_empty() { stdout } else { stderr };
        return Err(anyhow!(msg));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim_end().to_string())
}

pub fn parse_records(output: &str) -> Vec<Vec<String>> {
    if output.trim().is_empty() {
        return vec![];
    }
    output
        .split(RS)
        .filter(|s| !s.is_empty())
        .map(|rec| rec.split(FS).map(|f| f.to_string()).collect())
        .collect()
}

pub fn normalize_service_type(input: &str) -> String {
    if input.eq_ignore_ascii_case("imessage") || input.eq_ignore_ascii_case("iMessage") {
        "iMessage".to_string()
    } else if input.eq_ignore_ascii_case("sms") {
        "SMS".to_string()
    } else if input.eq_ignore_ascii_case("rcs") {
        "RCS".to_string()
    } else {
        "".to_string()
    }
}
