use std::{ops::Deref, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};

pub struct Spinner(ProgressBar);

impl Spinner {
    pub fn new() -> Self {
        let spinner = ProgressBar::new_spinner().with_style(
            ProgressStyle::default_spinner()
                .tick_chars("⣾⣷⣯⣟⡿⢿⣻⣽⣾")
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        spinner.enable_steady_tick(Duration::from_millis(100));
        Self(spinner)
    }
}

impl Deref for Spinner {
    type Target = ProgressBar;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.finish_and_clear();
    }
}
