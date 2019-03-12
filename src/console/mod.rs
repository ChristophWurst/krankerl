use indicatif::{ProgressBar, ProgressStyle};

pub fn default_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner()
                          .tick_chars("/|\\- ")
                          .template("{spinner:.dim.bold} {wide_msg}"));
    spinner
}
