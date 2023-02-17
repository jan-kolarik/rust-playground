#[cfg_attr(test, mockall::automock)]
pub trait Bill {
    fn add_beer(&mut self);
    fn get_unpaid_count(&self) -> u8;
    fn pay(&mut self, count: u8);
}

pub struct SimpleBill {
    beers: u8
}

impl SimpleBill {
    pub fn new() -> Self {
        SimpleBill { beers: 0 }
    }
}

impl Bill for SimpleBill {
    fn add_beer(&mut self) {
        self.beers += 1;
    }

    fn get_unpaid_count(&self) -> u8 {
        self.beers
    }

    fn pay(&mut self, count: u8) {
        self.beers -= count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_after_creation() {
        let bill = SimpleBill::new();
        assert_eq!(0, bill.get_unpaid_count());
    }

    #[test]
    fn unpaid_count_is_incrementing_when_adding_beers() {
        let mut bill = SimpleBill::new();
        
        bill.add_beer();
        assert_eq!(1, bill.get_unpaid_count());

        bill.add_beer();
        assert_eq!(2, bill.get_unpaid_count());
    }

    #[test]
    fn pay_decreases_beers() {
        let mut bill = SimpleBill::new();

        bill.add_beer();
        bill.add_beer();
        bill.add_beer();
        bill.pay(2);

        assert_eq!(1, bill.get_unpaid_count());
    }

    #[test]
    #[should_panic]
    fn exception_when_paying_more_beers() {
        let mut bill = SimpleBill::new();
        bill.pay(10);
    }
}
