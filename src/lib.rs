use spinners::{Spinner, Spinners};

pub fn create_spinner(message: String) -> Spinner {
    return Spinner::new(Spinners::Dots, message);
}
