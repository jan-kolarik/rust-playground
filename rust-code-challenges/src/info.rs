use std::fmt::Display;

fn info<T>(text: &T) 
where T: Display {
    print!("{}", text)
}

fn info_asref<T>(text: &T) 
where T: AsRef<str> {
    print!("{}", text.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_string_slice() {
        let text = "Test text";
        info(&text);
    }

    #[test]
    fn print_string() {
        let text = "Testing string".to_string();
        info(&text);
    }
}