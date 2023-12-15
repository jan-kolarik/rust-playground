fn decrypt(cipher: &str, key: &str) -> String {
    let first_letter = 'A' as i32;
    let alpha_length = 'Z' as i32 - first_letter + 1;
    let mut text = String::new();
    for (i, c) in cipher.chars().enumerate() {
        let c_pos = c as i32 - first_letter;
        let key_pos = key.chars().nth(i % key.len()).unwrap() as i32 - first_letter;
        let res_pos = ((c_pos - key_pos) % alpha_length + alpha_length) % alpha_length;
        text.push(char::from_u32((res_pos + first_letter) as u32).unwrap());
    }
    text
}

fn decrypt_2(cipher: &str, key: &str) -> String {
    let first_letter = b'A';
    let alpha_length = b'Z' - first_letter + 1;
    let mut key_iter = key.bytes().map(|b| b - first_letter).cycle();
    let text = cipher.bytes().map(|x| {
        ((x + alpha_length - first_letter) - key_iter.next().unwrap()) % alpha_length + first_letter
    }).collect();
    String::from_utf8(text).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrypt_test() {
        assert_eq!(decrypt("PVCDJGPAYCMYJRKUC", "WHYRUST"), String::from("TOEMPOWEREVERYONE"));
        assert_eq!(decrypt_2("PVCDJGPAYCMYJRKUC", "WHYRUST"), String::from("TOEMPOWEREVERYONE"));
    }
}
