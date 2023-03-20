use itertools::Itertools;

fn encode(text: &str) -> String {
    text.chars().group_by(|&c| c).into_iter().fold(String::new(), |mut encoded, (char, group)| {
        encoded.push_str(&group.count().to_string());
        encoded.push(char);
        encoded
    })
}

fn encode_straightforward(text: &str) -> String {
    let mut encoded = String::new();

    if text.is_empty() {
        return encoded;
    }

    let mut text_it = text.chars();
    let mut counter = 1;
    let mut previous_char = text_it.next().unwrap();

    for char in text_it {
        if char == previous_char {
            counter += 1;
        } else {
            encoded += &counter.to_string();
            encoded.push(previous_char);
            previous_char = char;
            counter = 1;
        }
    }

    encoded += &counter.to_string();
    encoded.push(previous_char);

    return encoded;
}

fn decode(text: &str) -> String {
    let counters_chars: Vec<String> = text.chars().group_by(|&c| c.is_numeric()).into_iter().map(|(_, group)| group.collect()).collect();
    let counters = counters_chars.iter().step_by(2);
    let chars = counters_chars.iter().skip(1).step_by(2);
    counters.zip(chars).map(|(counter, letter)| letter.repeat(counter.parse().unwrap())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test() {
        assert_eq!(encode("AAAAAaAA"), String::from("5A1a2A"));
        assert_eq!(encode("ccc"), String::from("3c"));
        assert_eq!(encode("LinkedIn"), String::from("1L1i1n1k1e1d1I1n"));
        assert_eq!(encode("CCCCCCCCCCCCCCCCaHHHHHHHHHH"), String::from("16C1a10H"));
    }

    #[test]
    fn decode_test() {
        assert_eq!(decode("5A1a2A"), String::from("AAAAAaAA"));
        assert_eq!(decode("3c"), String::from("ccc"));
        assert_eq!(decode("1L1i1n1k1e1d1I1n"), String::from("LinkedIn"));
        assert_eq!(decode("16C1a10H"), String::from("CCCCCCCCCCCCCCCCaHHHHHHHHHH"));
    }
}
