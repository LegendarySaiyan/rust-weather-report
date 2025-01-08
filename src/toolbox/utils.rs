pub fn is_russian(text: &String) -> bool {
    text.chars()
        .any(|c| ('А'..='я').contains(&c) || c == 'ё' || c == 'Ё')
}

pub fn is_english(text: &String) -> bool {
    text.chars()
        .any(|c| ('A'..='Z').contains(&c) || ('a'..='z').contains(&c))
}
