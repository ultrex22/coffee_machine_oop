use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MenuItem {
    pub name: String,
    pub cost: f64,
    pub ingredients: HashMap<String, f64>,
}

pub struct Menu {
    menu: Vec<MenuItem>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            menu: vec![
                MenuItem {
                    name: "latte".to_owned(),
                    cost: 2.5,
                    ingredients: HashMap::from([
                        ("water".to_owned(), 200.0),
                        ("milk".to_owned(), 150.0),
                        ("coffee".to_owned(), 24.0),
                    ]),
                },
                MenuItem {
                    name: "espresso".to_owned(),
                    cost: 1.5,
                    ingredients: HashMap::from([
                        ("water".to_owned(), 50.0),
                        ("milk".to_owned(), 0.0),
                        ("coffee".to_owned(), 18.0),
                    ]),
                },
                MenuItem {
                    name: "cappuccino".to_owned(),
                    cost: 3.0,
                    ingredients: HashMap::from([
                        ("water".to_owned(), 250.0),
                        ("milk".to_owned(), 50.0),
                        ("coffee".to_owned(), 24.0),
                    ]),
                },
            ],
        }
    }

    pub fn get_items(&self) -> String {
        let mut options = String::new();
        for item in &self.menu {
            options.push(',');
            options.push_str(item.name.as_str());
        }
        options.push(':');
        options
    }

    pub fn find_drink(&self, order_name: String) -> Option<&MenuItem> {
        let mut ret = None;
        for item in &self.menu {
            if item.name == order_name {
                ret = Some(item)
            }
        }
        if ret.is_none() {
            println!("Sorry, that item is not available, select another");
        }
        ret
    }
}
