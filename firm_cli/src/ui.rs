use std::time::Duration;

use console::Style;
use indicatif::{ProgressBar, ProgressStyle};

use super::logging;

pub struct UiStyle;

impl UiStyle {
    pub fn normal() -> Style {
        Style::new()
    }

    pub fn highlight() -> Style {
        Style::new().bold()
    }

    pub fn dim() -> Style {
        Style::new().dim()
    }

    pub fn warning() -> Style {
        Style::new().yellow().bold()
    }

    pub fn success() -> Style {
        Style::new().green().bold()
    }

    pub fn error() -> Style {
        Style::new().red().bold()
    }
}

pub fn header(msg: &str) {
    eprintln!("{}", UiStyle::highlight().apply_to(msg));
}

pub fn debug(msg: &str) {
    eprintln!("{}", UiStyle::dim().apply_to(msg));
}

pub fn info(msg: &str) {
    eprintln!("{}", UiStyle::normal().apply_to(msg));
}

pub fn warning(msg: &str) {
    eprintln!("{}", UiStyle::warning().apply_to(msg));
}

pub fn success(msg: &str) {
    eprintln!("{}", UiStyle::success().apply_to(msg));
}

pub fn error(msg: &str) {
    eprintln!("{}", UiStyle::error().apply_to(msg));
}

pub fn error_with_details(main_msg: &str, details: &str) {
    eprintln!("{}", UiStyle::error().apply_to(main_msg));
    eprintln!("   {}", UiStyle::dim().apply_to(details));
}

pub fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}")
            .expect("Invalid template"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(msg.to_string());

    let tracker = logging::get_multi_progress();
    tracker.add(pb.clone());

    pb
}

pub fn progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} {msg} [{bar}] {pos}/{len}")
            .expect("Invalid template")
            .progress_chars("# "),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    let tracker = logging::get_multi_progress();
    tracker.add(pb.clone());

    pb
}

pub fn json_output<T: serde::Serialize>(data: &T) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        println!("{}", json);
    }
}
