use std::collections::HashMap;

use std::sync::Arc;
use std::sync::Mutex;

use crate::bill::Bill;

#[cfg_attr(test, mockall::automock)]
pub trait BillShelf {
    fn put_bill(&mut self, customer: &str, bill: Box<dyn Bill>);
    fn get_bill(&self, customer: &str) -> Arc<Mutex<Box<dyn Bill>>>;
}

struct HugeBillShelf {
    customer_bills: HashMap<String, Arc<Mutex<Box<dyn Bill>>>>
}

impl HugeBillShelf {
    pub fn new() -> Self {
        HugeBillShelf { customer_bills: HashMap::new() }
    }
}

impl BillShelf for HugeBillShelf {
    fn put_bill(&mut self, customer: &str, bill:Box<dyn Bill>) {
        self.customer_bills.insert(customer.to_string(), Arc::new(Mutex::new(bill)));
    }

    fn get_bill(&self, customer: &str) -> Arc<Mutex<Box<dyn Bill>>> {
        self.customer_bills.get(customer).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::bill::MockBill;

    use super::*;

    #[test]
    #[should_panic]
    fn get_bill_panics_when_unknown_customer() {
        let shelf = HugeBillShelf::new();
        shelf.get_bill("unknown");
    }

    #[test]
    fn get_bill_returns_customer_bill() {
        let mut bill = Box::new(MockBill::new());
        bill.expect_get_unpaid_count().return_const(10);
        
        let customer = "some_customer";

        let mut shelf = HugeBillShelf::new();
        shelf.put_bill(customer, bill);

        let bill_from_shelf = shelf.get_bill(customer);
        assert_eq!(10, bill_from_shelf.lock().unwrap().get_unpaid_count());
    }
}
