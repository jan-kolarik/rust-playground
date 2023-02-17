use std::sync::Arc;
use std::sync::Mutex;

use crate::beer::Beer;
use crate::waiter::Waiter;

#[cfg_attr(test, mockall::automock)]
pub trait Customer {
    fn say_hello(&mut self);
    fn get_name(&self) -> String;
    fn order_beer(&mut self);
    fn receive_beer(&self, beer: Box<dyn Beer>);
    fn pay(&mut self);
}

struct HappyCustomer {
    name: String,
    my_waiter: Arc<Mutex<dyn Waiter>>
}

impl HappyCustomer {
    fn new(customer_name: &str, waiter: Arc<Mutex<dyn Waiter>>) -> Self {
        HappyCustomer { 
            name: customer_name.to_string(),
            my_waiter: waiter 
        }
    }
}

impl Customer for HappyCustomer {
    fn say_hello(&mut self) {
        self.my_waiter.clone().lock().unwrap().register_customer(self);
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn order_beer(&mut self) {
        self.my_waiter.clone().lock().unwrap().process_order(self);
    }

    fn receive_beer(&self, mut beer: Box<dyn Beer>) {
        beer.drink();
    }

    fn pay(&mut self) {
        self.my_waiter.clone().lock().unwrap().pay_bill(self);
    }
}

#[cfg(test)]
mod tests {
    use crate::beer::MockBeer;
    use crate::waiter::MockWaiter;

    use super::*;

    #[test]
    fn get_name_returns_name() {
        let name = "Some Name";
        let customer = HappyCustomer::new(name, Arc::new(Mutex::new(MockWaiter::new())));

        assert_eq!(name, customer.get_name());
    }

    #[test]
    fn say_hello_registers_customer_at_waiter() {
        let mut waiter = MockWaiter::new();
        waiter.expect_register_customer().times(1).return_const(());

        let mut customer = HappyCustomer::new("name", Arc::new(Mutex::new(waiter)));
        customer.say_hello();
    }

    #[test]
    fn order_beer_calls_process_order_at_waiter() {
        let mut waiter = MockWaiter::new();
        waiter.expect_process_order().times(1).return_const(());

        let mut customer = HappyCustomer::new("another_name", Arc::new(Mutex::new(waiter)));
        customer.order_beer();
    }

    #[test]
    fn receiving_beer_results_in_immediate_drink() {
        let waiter = MockWaiter::new();
        let customer = HappyCustomer::new("another_name", Arc::new(Mutex::new(waiter)));

        let mut beer = Box::new(MockBeer::new());
        beer.expect_drink().times(1).return_const(());

        customer.receive_beer(beer);
    }

    #[test]
    fn pay_pays_bill_at_waiter() {
        let mut waiter = MockWaiter::new();
        waiter.expect_pay_bill().times(1).return_const(());

        let mut customer = HappyCustomer::new("another_name", Arc::new(Mutex::new(waiter)));
        customer.pay();
    }
}
