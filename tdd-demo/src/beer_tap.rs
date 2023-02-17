use crate::beer::Beer;
use crate::beer::OrdinaryBeer;

#[cfg_attr(test, mockall::automock)]
pub trait BeerTap {
    fn pour_beer(&self) -> Box<dyn Beer>;
}

pub struct OrdinaryBeerTap;

impl OrdinaryBeerTap {
    pub fn new() -> Self {
        OrdinaryBeerTap {}
    }
}

impl BeerTap for OrdinaryBeerTap {
    fn pour_beer(&self) -> Box<dyn Beer> {
        Box::new(OrdinaryBeer::new())
    }
}
