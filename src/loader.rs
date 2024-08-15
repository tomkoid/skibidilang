use spinners::{Spinner, Spinners};

const SPINNER_TYPE: Spinners = Spinners::Line;

pub struct Loader {
    spinner: Spinner,
}

impl Loader {
    pub fn new(message: String) -> Loader {
        let sp = Spinner::new(SPINNER_TYPE, message);
        Loader { spinner: sp }
    }

    pub fn stop_success(&mut self) {
        self.spinner.stop_with_symbol("✓");
    }

    pub fn stop_success_msg(&mut self, message: String) {
        self.spinner.stop_and_persist("✓", message);
    }

    pub fn stop_error(&mut self) {
        self.spinner.stop_with_symbol("✗");
    }

    pub fn stop_error_msg(&mut self, message: String) {
        self.spinner.stop_and_persist("✗", message);
    }
}
