use crate::beer_tap::BeerTap;
use crate::bill_pad::BillPad;
use crate::bill_shelf::BillShelf;
use crate::customer::Customer;

#[cfg_attr(test, mockall::automock)]
pub trait Waiter {
    fn register_customer(&mut self, customer: &mut dyn Customer);
    fn process_order(&self, customer: &mut dyn Customer);
    fn pay_bill(&self, customer: &mut dyn Customer);
}

struct KindWaiter {
    beer_tap: Box<dyn BeerTap>,
    bill_pad: Box<dyn BillPad>,
    bill_shelf: Box<dyn BillShelf>
}

impl KindWaiter {
    fn new(beer_factory: Box<dyn BeerTap>, bill_factory: Box<dyn BillPad>, bill_container: Box<dyn BillShelf>) -> Self {
        KindWaiter { 
            beer_tap: beer_factory,
            bill_pad: bill_factory,
            bill_shelf: bill_container
        }
    }
}

impl Waiter for KindWaiter {
    fn register_customer(&mut self, customer: &mut dyn Customer) {
        let new_bill = self.bill_pad.pluck_bill();
        self.bill_shelf.put_bill(&customer.get_name(), new_bill);
    }

    fn process_order(&self, customer: &mut dyn Customer) {
        let bill = self.bill_shelf.get_bill(&customer.get_name());
        bill.lock().unwrap().add_beer();

        let beer = self.beer_tap.pour_beer();
        customer.receive_beer(beer);
    }

    fn pay_bill(&self, customer: &mut dyn Customer) {
        let bill_mutex = self.bill_shelf.get_bill(&customer.get_name());
        let mut bill = bill_mutex.lock().unwrap();

        let beer_count = bill.get_unpaid_count();
        bill.pay(beer_count);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::Mutex;

    use mockall::predicate::eq;

    use crate::beer::MockBeer;
    use crate::beer_tap::MockBeerTap;
    use crate::bill::MockBill;
    use crate::bill_pad::MockBillPad;
    use crate::bill_shelf::MockBillShelf;
    use crate::customer::MockCustomer;

    use super::*;

    #[test]
    fn register_customer_creates_new_bill() {
        let mut bill = Box::new(MockBill::new());
        bill.expect_get_unpaid_count().return_const(0);
        
        let mut bill_pad = Box::new(MockBillPad::new());
        bill_pad.expect_pluck_bill().times(1).return_once(move || bill);

        let mut bill_shelf = Box::new(MockBillShelf::new());
        bill_shelf.expect_put_bill().times(1).withf(|name, bill| name == "Jack" && bill.get_unpaid_count() == 0).return_const(());
        
        let beer_tap = Box::new(MockBeerTap::new());

        let mut waiter = KindWaiter::new(beer_tap, bill_pad, bill_shelf);

        let mut customer = MockCustomer::new();
        customer.expect_get_name().times(1).return_const("Jack");

        waiter.register_customer(&mut customer);
    }

    #[test]
    fn process_order_sends_new_beer_to_customer() {
        let customer_name = "Jill";

        let mut bill = Box::new(MockBill::new());
        bill.expect_add_beer().times(1).return_const(());

        let mut bill_shelf = Box::new(MockBillShelf::new());
        bill_shelf.expect_get_bill().times(1).with(eq(customer_name)).return_once(move |_| Arc::new(Mutex::new(bill)));

        let mut beer_tap = Box::new(MockBeerTap::new());
        beer_tap.expect_pour_beer().times(1).return_once(move || Box::new(MockBeer::new()));

        let bill_pad = Box::new(MockBillPad::new());

        let waiter = KindWaiter::new(beer_tap, bill_pad, bill_shelf);

        let mut customer = MockCustomer::new();
        customer.expect_get_name().times(1).return_const(customer_name);
        customer.expect_receive_beer().times(1).return_const(());

        waiter.process_order(&mut customer);
    }

    #[test]
    fn pay_bill_pays_all_beers() {
        let beer_tap = Box::new(MockBeerTap::new());
        let bill_pad = Box::new(MockBillPad::new());

        let mut bill = Box::new(MockBill::new());
        bill.expect_get_unpaid_count().times(1).return_const(3);
        bill.expect_pay().times(1).with(eq(3)).return_const(());

        let customer_name = "Customer";

        let mut bill_shelf = Box::new(MockBillShelf::new());
        bill_shelf.expect_get_bill().times(1).with(eq(customer_name)).return_once(move |_| Arc::new(Mutex::new(bill)));

        let waiter = KindWaiter::new(beer_tap, bill_pad, bill_shelf);

        let mut customer = MockCustomer::new();
        customer.expect_get_name().times(1).return_const(customer_name);

        waiter.pay_bill(&mut customer)
    }
}
