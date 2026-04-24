mod formatter;
mod os_colors;

use std::{
    fs::{self},
    time::SystemTime,
};

use humanize_duration::prelude::DurationExt;
use owo_colors::OwoColorize;

use crate::{
    formatter::CustomFormatter,
    os_colors::{BLUE, GREEN, ORANGE, PURPLE, RED, YELLOW},
};

fn main() {
    // Get OS on Windows
    #[cfg(target_os = "windows")]
    let os = "Windows".to_owned();

    #[cfg(target_os = "macos")]
    let os = "macOS".to_owned();

    // Get OS on anything that is Linux or FreeBSD based
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    let os = {
        use etc_os_release::OsRelease;

        let etc = OsRelease::open();
        match etc {
            Ok(res) => res.pretty_name().to_owned(),
            Err(_) => "Unknown".to_owned(),
        }
    };

    // Quit if not on Windows, Mac, or Linux
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        eprintln!("Unsupported OS");
        return;
    }

    // Get install time
    let install_time = get_install_time();
    // Get it out of the Result
    let Ok(install_time) = install_time else {
        eprintln!("Failed to get age");
        return;
    };

    // Get it out of the Option
    let Some(install_time) = install_time else {
        eprintln!("Failed to get age");
        return;
    };

    // Get difference between now and install_time
    let age = SystemTime::now().duration_since(install_time);
    // Get it out of the Result
    let Ok(age) = age else {
        eprintln!("Failed to get age");
        return;
    };

    // Humanize age with custom formatter
    let mut age_pretty = age
        .human_with_format(humanize_duration::Truncate::Second, CustomFormatter)
        .to_string();

    // Remove the ending comma
    _ = age_pretty.pop();

    // Push " old" to the end to humanize it more
    age_pretty.push_str(" old");

    println!(
        "Your {} install is {}",
        format_with_os_color(&os, &os),
        format_with_os_color(&os, &age_pretty)
    );
}

/// Gets when the root filesystem was created (which should match OS installation time [or at least close enough])
fn get_install_time() -> std::io::Result<Option<SystemTime>> {
    // Set path to system root
    #[cfg(windows)]
    let path = "C:\\";

    #[cfg(not(windows))]
    let path = "/";

    // Read dir metadata
    let metadata = fs::metadata(path)?;

    // Return created time, using modified time as fallback; Returns None if that fails too
    Ok(match metadata.created() {
        Ok(created) => Some(created),
        Err(_) => metadata.modified().ok(),
    })
}

/// Formats str_to_format with os colors
fn format_with_os_color(os: &str, str_to_format: &str) -> String {
    match os.to_lowercase() {
        s if BLUE.iter().any(|d| s.contains(d)) => str_to_format.blue().to_string(),
        s if PURPLE.iter().any(|d| s.contains(d)) => str_to_format.purple().to_string(),
        s if GREEN.iter().any(|d| s.contains(d)) => str_to_format.green().to_string(),
        s if ORANGE.iter().any(|d| s.contains(d)) => str_to_format.bright_red().to_string(), // I don't know if bright red is the best choice for orange, but I don't have access to orange
        s if YELLOW.iter().any(|d| s.contains(d)) => str_to_format.yellow().to_string(),
        s if RED.iter().any(|d| s.contains(d)) => str_to_format.red().to_string(),
        _ => str_to_format.blue().to_string(),
    }
}
