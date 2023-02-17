trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

type Message = Vec<Letter>;
type Letter = Vec<Pulse>;

#[derive(PartialEq)]
enum Pulse {
    Short,
    Long
}

fn convert_char_to_letter(c: char) -> Letter {
    use Pulse::*;

    match c {
        'a' => vec![Long, Short],
        'b' => vec![Long, Short, Short, Short],
        'c' => vec![Long, Short, Long, Short],
        _ => panic!("Unknown letter {}", c)
    }
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut message = Message::with_capacity(self.len());
        for letter in self.to_lowercase().chars() {
            message.push(convert_char_to_letter(letter))
        }
        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_a_to_morse_code() {
        let letter_a = "A".to_string();
        let expected_result = vec![vec![Pulse::Long, Pulse::Short]];
        assert!(letter_a.to_morse_code().iter().eq(expected_result.iter()));
    }

    #[test]
    fn convert_b_to_morse_code() {
        let letter_a = "B".to_string();
        let expected_result = vec![vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short]];
        assert!(letter_a.to_morse_code().iter().eq(expected_result.iter()));
    }

    #[test]
    fn convert_string_to_morse_code() {
        let word = "ABC".to_string();
        let expected_result = vec![
            vec![Pulse::Long, Pulse::Short],
            vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
            vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short]
        ];
        assert!(word.to_morse_code().iter().eq(expected_result.iter()));
    }
}