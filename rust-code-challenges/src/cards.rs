#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ace
}

struct Hand {
    cards: Vec<Card>
}

impl Hand {
    fn value(&self) -> usize {
        use Card::*;

        let mut sorted_cards = self.cards.clone();
        sorted_cards.sort();

        let mut total = 0;

        for card in sorted_cards {
            total += match card {
                Jack | Queen | King => 10,
                Ace if total <= 10 => 11,
                Ace if total > 10 => 1,
                _ => card as usize,
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Card::*;

    #[test]
    fn number_cards() {
        let hand = Hand { cards: vec![Two, Three, Four] };
        assert_eq!(hand.value(), 9);
    }

    #[test]
    fn various_cards() {
        let hand = Hand { cards: vec![Five, King] };
        assert_eq!(hand.value(), 15);
    }

    #[test]
    fn over_twenty_one() {
        let hand = Hand { cards: vec![Five, Six, Ace] };
        assert_eq!(hand.value(), 12);
    }
}