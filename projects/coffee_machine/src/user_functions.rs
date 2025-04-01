use std::process::exit;

use crate::coffee_machine;

pub struct UserInputOfMoney {
    pub quarter_number: i8,
    pub dime_number: i8,
    pub nickel_number: i8,
    pub penny_number: i8,
}

impl Default for UserInputOfMoney {
    fn default() -> Self {
        UserInputOfMoney {
            quarter_number: 0,
            dime_number: 0,
            nickel_number: 0,
            penny_number: 0,
        }
    }
}

impl UserInputOfMoney {
    pub fn new(
        quarter_number: i8,
        dime_number: i8,
        nickel_number: i8,
        penny_number: i8,
    ) -> UserInputOfMoney {
        UserInputOfMoney {
            quarter_number,
            dime_number,
            nickel_number,
            penny_number,
        }
    }

    pub fn ask_the_user_to_enter_some_money() -> Result<Self, Box<dyn std::error::Error>> {
        let mut input = String::new();
        println!("Enter the number of quarters you have: ");
        std::io::stdin().read_line(&mut input)?;
        let quarter_number: i8 = input.trim().parse()?;
        input.clear();
        crate::clear_screen();

        println!("Enter the number of dimes you have: ");
        std::io::stdin().read_line(&mut input)?;
        let dime_number: i8 = input.trim().parse()?;
        input.clear();
        crate::clear_screen();

        println!("Enter the number of nickels you have");
        std::io::stdin().read_line(&mut input)?;
        let nickel_number: i8 = input.trim().parse()?;
        input.clear();
        crate::clear_screen();

        println!("enter the number of quartes you have");
        std::io::stdin().read_line(&mut input)?;
        let penny_number: i8 = input.trim().parse().expect("Please enter a number");
        crate::clear_screen();

        Ok(UserInputOfMoney::new(
            quarter_number,
            dime_number,
            nickel_number,
            penny_number,
        ))
    }

    pub fn calculate_total_money(&self) -> f32 {
        let quarter_value = 0.25;
        let dime_value = 0.10;
        let nickel_value = 0.05;
        let penny_value = 0.01;

        let total_money = (self.quarter_number as f32 * quarter_value)
            + (self.dime_number as f32 * dime_value)
            + (self.nickel_number as f32 * nickel_value)
            + (self.penny_number as f32 * penny_value);

        total_money
    }

    pub fn buy_late(machine_resources: &coffee_machine::MachineResources, total_money: f32) {
        crate::clear_screen();
        if total_money >= machine_resources.late_price {
            println!("the late will be ready soon.....");
            let remaining_money = total_money - machine_resources.late_price;
            println!("Your remaining money : {}", remaining_money);
        } else {
            println!("You don't have enough money to buy a late");
        }
    }

    pub fn buy_cappuccino(machine_resources: &coffee_machine::MachineResources, total_money: f32) {
        crate::clear_screen();
        if total_money >= machine_resources.cappuccino_price {
            println!("the capuccino will be ready soon.....");
            let remaining_money = total_money - machine_resources.cappuccino_price;
            println!("Your remaining money : {}", remaining_money);
        } else {
            println!("You don't have enough money to buy a cappuccino");
        }
    }

    pub fn buy_espresso(machine_resources: &coffee_machine::MachineResources, total_money: f32) {
        crate::clear_screen();
        if total_money >= machine_resources.espresso_price {
            println!("the espresso will be ready soon.....");
            let remaining_money = total_money - machine_resources.espresso_price;
            println!("Your remaining money : {}", remaining_money);
        } else {
            println!("You don't have enough money to buy a espresso");
        }
    }
}
