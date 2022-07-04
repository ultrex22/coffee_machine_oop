mod coffee_maker;
mod menu;
mod money_machine;

#[allow(unused_variables)]
#[allow(illegal_floating_point_literal_pattern)]
#[allow(dead_code)]
fn main() {
    let mut coffee_maker = coffee_maker::CoffeeMaker::new();
    let coffee_menu = menu::Menu::new();
    let mut coffee_money = money_machine::MoneyMachine::new();

    let mut machine_on = true;
    while machine_on {
        let mut buffer = String::new();
        println!("What drink do you want? {}", coffee_menu.get_items());
        println!("Or select 'off' or 'report' :");
        let _br = std::io::stdin().read_line(&mut buffer).unwrap();
        let order = buffer.trim().to_lowercase();

        if order == "off" {
            println!("Machine turning off...");
            machine_on = false;
            continue;
        } else if order == "report" {
            coffee_maker.report();
            coffee_money.report();
            continue;
        }

        let drink_choice = coffee_menu.find_drink(order);
        let drink_choice = match drink_choice {
            Some(x) => x,
            None => continue,
        };
        let mut drink_choice = drink_choice.clone();

        if coffee_maker.is_resource_sufficient(drink_choice.clone())
            && coffee_money.make_payment(drink_choice.cost as f32)
        {
            coffee_maker.make_coffee(&mut drink_choice);
        }

        machine_on = false;
    }
}
