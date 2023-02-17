use crate::bill::Bill;
use crate::bill::SimpleBill;

#[cfg_attr(test, mockall::automock)]
pub trait BillPad {
    fn pluck_bill(&self) -> Box<dyn Bill>;
}

pub struct SimpleBillPad;

impl SimpleBillPad {
    pub fn new() -> Self {
        SimpleBillPad {}
    }
}

impl BillPad for SimpleBillPad {
    fn pluck_bill(&self) -> Box<dyn Bill> {
        Box::new(SimpleBill::new())
    }
}
