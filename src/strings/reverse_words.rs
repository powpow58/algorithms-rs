use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// Reverses the order of characters in a word
///
/// # Arguments
///
/// * `word` - The word to reverse
pub fn reverse_word(word: &str) -> String {
    word.chars().rev().collect()
}

/// Reverses the words in a sentence.
///
/// # Arguments
///
/// * `sentence` - The sentence to be reversed.
pub fn reverse_word_order(sentence: &str) -> String {
    sentence.split(' ').rev().collect()
}

/// Reverse all words that are longer than `n` characters in a sentence.
///
/// # Arguments
///
/// * `sentence` - The sentence to be reversed.
/// * `n` - The minimum length of a word before it can be reversed.
///
/// # Example
///
/// ```
/// use algorithms::strings::reverse_words_longer_than;
///
/// println!("{}", reverse_words_longer_than("Rust is great and Java is also great", 4));
///
/// //Output: Rust is taerg and Java is also taerg
///
/// println!("{}", reverse_words_longer_than("1 12 123 1234 54321 654321", 3));
///
/// //Output: 1 12 123 1234 12345 123456 
/// ```
pub fn reverse_words_longer_than(sentence: &str, n: usize) -> String {
    sentence
        .split_whitespace()
        .map(|word| match word.len() > n {
            true => reverse_word(word),
            false => word.to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(("Hey wollef sroirraw", 4))]
    fn test_remove_long_words(data: (&str, usize)) {
        let actual = reverse_words_longer_than(data.0, data.1);
        assert_eq!(actual, "Hey fellow warriors")
    }
}
