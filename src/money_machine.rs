#[derive(Debug, Copy, Clone)]
struct CoinValues {
    quarters: f32,
    dimes: f32,
    nickles: f32,
    pennies: f32,
}

impl CoinValues {
    fn new() -> Self {
        Self {
            quarters: 0.25,
            dimes: 0.10,
            nickles: 0.05,
            pennies: 0.01,
        }
    }
}

impl IntoIterator for CoinValues {
    type Item = f32;
    type IntoIter = CoinValuesIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        CoinValuesIntoIterator {
            coin_values: self,
            index: 0,
        }
    }
}

struct CoinValuesIntoIterator {
    coin_values: CoinValues,
    index: i8,
}

impl Iterator for CoinValuesIntoIterator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.coin_values.quarters,
            1 => self.coin_values.dimes,
            2 => self.coin_values.nickles,
            3 => self.coin_values.pennies,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

pub struct MoneyMachine {
    currency: char,
    coin_values: CoinValues,
    profit: f32,
    money_received: f32,
}

impl MoneyMachine {
    pub(crate) fn new() -> Self {
        Self {
            currency: '$',
            coin_values: CoinValues::new(),
            profit: 0.0,
            money_received: 0.0,
        }
    }

    pub(crate) fn report(&self) {
        println!("Profit: {}{}", self.currency, self.profit);
    }

    fn process_coins(&mut self) -> f32 {
        println!("Please insert coins:");
        for coin in self.coin_values {
            let mut buffer = String::new();
            let coin_name = match coin {
                0.25 => ("Quarters", 0.25),
                0.10 => ("Dimes", 0.10),
                0.05 => ("Nickles", 0.05),
                0.01 => ("Pennies", 0.01),
                _ => ("", 0.0),
            };
            println!("How many {}? ", coin_name.0);
            std::io::stdin().read_line(&mut buffer).unwrap();
            let buffer: f32 = buffer.trim().parse().unwrap();
            self.money_received += buffer * coin_name.1;
        }
        self.money_received
    }

    pub fn make_payment(&mut self, cost: f32) -> bool {
        self.process_coins();
        return if self.money_received >= cost {
            let change = self.money_received - cost;
            println!("Here is {}{:.2} in change.", self.currency, change);
            self.profit += cost;
            self.money_received = 0.00;
            true
        } else {
            println!("Sorry that's not enough money. Money refunded.");
            self.money_received = 0.00;
            false
        };
    }
}
