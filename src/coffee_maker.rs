use std::collections::HashMap;

use crate::menu::*;

pub(crate) struct CoffeeMaker {
    resource: HashMap<String, f64>,
}

impl CoffeeMaker {
    pub fn new() -> Self {
        Self {
            resource: HashMap::from([
                ("water".to_owned(), 300.0),
                ("milk".to_owned(), 200.0),
                ("coffee".to_owned(), 100.0),
            ]),
        }
    }

    pub fn report(&self) {
        println!("Water Level: {}", &self.resource["water"]);
        println!("Milk Level: {}", &self.resource["milk"]);
        println!("Coffee Level: {}", &self.resource["coffee"]);
    }

    pub fn is_resource_sufficient(&self, drink: MenuItem) -> bool {
        let mut ret = true;
        for item in drink.ingredients.clone() {
            if drink.ingredients[&item.0] > self.resource[&item.0] {
                println!("Sorry there is not enough {}.", item.0);
                ret = false;
            }
        }
        ret
    }

    pub fn make_coffee(&mut self, order_item: &mut MenuItem) {
        for (order_item_name, order_item_value) in &order_item.ingredients {
            let resource_value = self.resource.get(order_item_name).unwrap();
            let new_value = resource_value - order_item_value;
            self.resource
                .insert(order_item_name.clone(), new_value)
                .unwrap();
        }
        println!("Here is your {}. Enjoy!", order_item.name);
    }
}
