use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

pub struct Progress {
    progress_bar: MultiProgress,
    steps: u16,
}

impl Progress {
    pub fn new() -> Self {
        Progress {
            progress_bar: MultiProgress::new(),
            steps: 0,
        }
    }

    pub fn add(&mut self) {
        let spinner_style = ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{prefix:.bold.dim} {spinner} {wide_msg}");
        let pb = self.progress_bar.add(ProgressBar::new(1));
        pb.set_style(spinner_style);
        pb.set_prefix(&format!("[{}/?]", self.steps + 1));
    }
}
