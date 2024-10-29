mod text_processing {

    mod letters {
        pub fn count_letters(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_alphabetic()).count()
        }
    }

    mod numbers {
        pub fn count_numbers(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_numeric()).count()
        }
    }

    pub fn count_letters_and_numbers(text: &str) -> (usize, usize) {
        let number_of_letters = letters::count_letters(text);
        let number_of_numbers = numbers::count_numbers(text);
        (number_of_letters, number_of_numbers)
    }
}

fn main() {
    assert_eq!(
        text_processing::count_letters_and_numbers("221B Baker Street"),
        (12, 3)
    );
    assert_eq!(
        text_processing::count_letters_and_numbers("711 Maple Street"),
        (11, 3)
    );
    assert_eq!(
        text_processing::count_letters_and_numbers("4 Privet Drive"),
        (11, 1)
    );
}
