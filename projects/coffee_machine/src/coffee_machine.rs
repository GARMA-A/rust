#![allow(unused)]
use std::io::Read;
pub struct MachineResources {
    pub water: i8,
    pub milk: i8,
    pub coffee: i8,
    pub money: f32,
    pub late_price: f32,
    pub cappuccino_price: f32,
    pub espresso_price: f32,
}

impl MachineResources {
    pub fn new(
        water: i8,
        milk: i8,
        coffee: i8,
        money: f32,
        late_price: f32,
        cappuccino_price: f32,
        espresso_price: f32,
    ) -> MachineResources {
        MachineResources {
            water,
            milk,
            coffee,
            money,
            late_price,
            cappuccino_price,
            espresso_price,
        }
    }

    pub fn start_menue(&self) -> Result<char, std::io::Error> {
        let coffee_machine = r#"
     _________
    |         |
    |  _____  |
    | |     | |
    | |     | |
    | |_____| |
    |_________|
    |    |    |
    |    |    |
    |____|____|
    "#;

        let mut input: String = String::new();
        println!("{}", coffee_machine);
        println!("Welcome to the coffee machine!");
        println!("----------------------------------------------------");
        println!("to buy a late press 'l' (price : {}$) ", self.late_price);
        println!(
            "to buy a cappuccino press 'c' (price : {}$) ",
            self.cappuccino_price
        );
        println!(
            "to buy a espresso press 'e' (price : {}$) ",
            self.espresso_price
        );
        println!("----------------------------------------------------");
        println!("Please select an option:");
        std::io::stdin().read_line(&mut input)?;

        let input = input.as_bytes();
        if input.len() > 2 {
            println!("Please enter only one character");
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid selection",
            ));
        }
        match input[0].to_ascii_lowercase() as char {
            'l' => {
                crate::clear_screen();
                println!("You have selected a late");
                return Ok('l');
            }
            'c' => {
                crate::clear_screen();
                println!("You have selected a cappuccino");
                return Ok('c');
            }
            'e' => {
                crate::clear_screen();
                println!("You have selected a espresso");
                return Ok('e');
            }
            _ => {
                crate::clear_screen();
                println!("your enter is not 'l' , 'c' , 'e' please enter these characters");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid selection",
                ));
            }
        }
    }
}
