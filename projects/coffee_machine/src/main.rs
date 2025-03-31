#![allow(unused)]
use std::{intrinsics::breakpoint, process::Command};

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

use user_functions::UserInputOfMoney;

mod coffee_machine;
mod user_functions;

fn main() {
    'system_start: loop {
        let machine_resources =
            coffee_machine::MachineResources::new(100, 100, 100, 0.0, 2.5, 3.0, 1.5);

        let user_choice = match machine_resources.start_menue() {
            Ok(ch) => ch,
            Err(error) => {
                println!("Error: {}", error);
                continue 'system_start;
            }
        };

        let user_input: UserInputOfMoney =
            match user_functions::UserInputOfMoney::ask_the_user_to_enter_some_money() {
                Ok(number) => number,
                Err(error) => {
                    println!("Error: {}", error);
                    continue 'system_start;
                }
            };

        let total_money = user_input.calculate_total_money();

        if user_choice == 'l' {
            UserInputOfMoney::buy_late(&machine_resources, total_money);
            break;
        } else if user_choice == 'c' {
            UserInputOfMoney::buy_cappuccino(&machine_resources, total_money);
            break;
        } else if user_choice == 'e' {
            UserInputOfMoney::buy_espresso(&machine_resources, total_money);
            break;
        } else {
            println!("Please enter a valid option");
        }
    }
    println!("Thank you for using the coffee machine!");
}
