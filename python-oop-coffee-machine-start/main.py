from menu import Menu
from coffee_maker import CoffeeMaker
from money_machine import MoneyMachine

coffee_maker = CoffeeMaker()
coffee_menu = Menu()
coffee_money = MoneyMachine()


def coffee_machine():
    machine_on = True
    while machine_on:
        order = input(f"What would you like? {coffee_menu.get_items()} : ").lower()
        if order == "off":
            machine_on = False
            print("Machine turning off...")
            return
        if order == "report":
            coffee_maker.report()
            coffee_machine()
        drink_choice = coffee_menu.find_drink(order)
        if drink_choice is None:
            coffee_machine()
        if coffee_maker.is_resource_sufficient(drink_choice):
            coffee_money.make_payment(drink_choice.cost)
            coffee_maker.make_coffee(drink_choice)


coffee_machine()

# from repl.it:

# money_machine = MoneyMachine()
# coffee_maker = CoffeeMaker()
# menu = Menu()

# is_on = True

# while is_on:
#     options = menu.get_items()
#     choice = input(f"What would you like? ({options}): ")
#     if choice == "off":
#         is_on = False
#     elif choice == "report":
#         coffee_maker.report()
#         money_machine.report()
#     else:
#         drink = menu.find_drink(choice)
#         is_enough_ingredients = coffee_maker.is_resource_sufficient(drink)
#         is_payment_successful = money_machine.make_payment(drink.cost)
#         if is_enough_ingredients and is_payment_successful:
#             coffee_maker.make_coffee(drink)
