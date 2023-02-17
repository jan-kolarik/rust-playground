#[cfg_attr(test, mockall::automock)]
pub trait Beer {
    fn is_full(&self) -> bool;
    fn drink(&mut self);
}

pub struct OrdinaryBeer {
    full: bool
}

impl OrdinaryBeer {
    pub fn new() -> Self {
        OrdinaryBeer { full: true }
    }
}

impl Beer for OrdinaryBeer {
    fn is_full(&self) -> bool {
        self.full
    }

    fn drink(&mut self) {
        self.full = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_full_after_creation() {
        let beer = OrdinaryBeer::new();
        assert!(beer.is_full());
    }

    #[test]
    fn is_empty_after_drinking() {
        let mut beer = OrdinaryBeer::new();
        beer.drink();
        assert!(!beer.is_full());
    }
}
