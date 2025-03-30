use user_functions::UserInputOfMoney;

mod display;
mod user_functions;

fn main() {
    let machine_resources = display::MachineResources::new(100, 100, 100, 0.0, 2.5, 3.0, 1.5);

    let user_choice = machine_resources
        .start_menue()
        .expect("Please enter a valid option");

    let user_input = user_functions::UserInputOfMoney::ask_the_user_to_enter_some_money()
        .expect("Please enter a number");

    let total_money = user_input.calculate_total_money();

    if user_choice == 'l' {
        UserInputOfMoney::buy_late(&machine_resources, total_money);
    } else if user_choice == 'c' {
        UserInputOfMoney::buy_cappuccino(&machine_resources, total_money);
    } else if user_choice == 'e' {
        UserInputOfMoney::buy_espresso(&machine_resources, total_money);
    } else {
        println!("Please enter a valid option");
    }
}
