#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::utils::{is_english, is_russian};

    #[test]
    fn language_checker() {
        let russian_string = String::from("Текст");
        let english_string = String::from("Text");

        assert!(is_russian(&russian_string));
        assert!(is_english(&english_string));
    }
}
