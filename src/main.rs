#![allow(unused)]
extern crate eval;

use eval::eval;
use std::env::args;
use std::io;
use std::process::exit;

fn main() {
    let version: &str = "1.0.4";
    // My Structs
    struct Conversion {
        index: String,
        from: String,
        to: String,
        formula: &'static str,
    }

    let mut category_choice = String::new();
    category_choice = "0".to_string();

    let area_conversions: [Conversion; 16] = [
        Conversion {
            index: String::from("1"),
            from: String::from("Square Meter"),
            to: String::from("Square Mile"),
            formula: "input / 2590000",
        },
        Conversion {
            index: String::from("2"),
            from: String::from("Square Meter"),
            to: String::from("Square Foot"),
            formula: "input * 10.764",
        },
        Conversion {
            index: String::from("3"),
            from: String::from("Square Meter"),
            to: String::from("Square Yard"),
            formula: "input * 1.196",
        },
        Conversion {
            index: String::from("4"),
            from: String::from("Square Meter"),
            to: String::from("Square Inch"),
            formula: "input * 1550.0",
        },
        Conversion {
            index: String::from("5"),
            from: String::from("Square Mile"),
            to: String::from("Square Meter"),
            formula: "input * 2590000",
        },
        Conversion {
            index: String::from("6"),
            from: String::from("Square Mile"),
            to: String::from("Square Foot"),
            formula: "input * 27880000",
        },
        Conversion {
            index: String::from("7"),
            from: String::from("Square Mile"),
            to: String::from("Square Yard"),
            formula: "input * 3098000",
        },
        Conversion {
            index: String::from("8"),
            from: String::from("Square Mile"),
            to: String::from("Square Inch"),
            formula: "input * 4014000000",
        },
        Conversion {
            index: String::from("9"),
            from: String::from("Square Foot"),
            to: String::from("Square Meter"),
            formula: "input / 10.764",
        },
        Conversion {
            index: String::from("10"),
            from: String::from("Square Foot"),
            to: String::from("Square Mile"),
            formula: "input / 27880000",
        },
        Conversion {
            index: String::from("11"),
            from: String::from("Square Foot"),
            to: String::from("Square Yard"),
            formula: "input / 9.0",
        },
        Conversion {
            index: String::from("12"),
            from: String::from("Square Foot"),
            to: String::from("Square Inch"),
            formula: "input * 144.0",
        },
        Conversion {
            index: String::from("13"),
            from: String::from("Square Inch"),
            to: String::from("Square Meter"),
            formula: "input / 1550.0",
        },
        Conversion {
            index: String::from("14"),
            from: String::from("Square Inch"),
            to: String::from("Square Mile"),
            formula: "input / 4014000000",
        },
        Conversion {
            index: String::from("15"),
            from: String::from("Square Inch"),
            to: String::from("Square Foot"),
            formula: "input / 144.0",
        },
        Conversion {
            index: String::from("16"),
            from: String::from("Square Inch"),
            to: String::from("Square Yard"),
            formula: "input / 1296.0",
        },
    ];

    let length_conversions: [Conversion; 8] = [
        Conversion {
            index: String::from("1"),
            from: String::from("Km"),
            to: String::from("Mile"),
            formula: "input / 1.609",
        },
        Conversion {
            index: String::from("2"),
            from: String::from("Km"),
            to: String::from("Meter"),
            formula: "input * 1000",
        },
        Conversion {
            index: String::from("3"),
            from: String::from("Mile"),
            to: String::from("Km"),
            formula: "input * 1.609",
        },
        Conversion {
            index: String::from("4"),
            from: String::from("Mile"),
            to: String::from("Meter"),
            formula: "input * 1609",
        },
        Conversion {
            index: String::from("5"),
            from: String::from("Inch"),
            to: String::from("Meter"),
            formula: "input / 39.37",
        },
        Conversion {
            index: String::from("6"),
            from: String::from("Inch"),
            to: String::from("Foot"),
            formula: "input / 12",
        },
        Conversion {
            index: String::from("7"),
            from: String::from("Foot"),
            to: String::from("Inch"),
            formula: "input * 12",
        },
        Conversion {
            index: String::from("8"),
            from: String::from("Foot"),
            to: String::from("Meter"),
            formula: "input * 3.281",
        },
    ];

    let temp_conversions: [Conversion; 6] = [
        Conversion {
            index: String::from("1"),
            from: String::from("Celsius"),
            to: String::from("Fahrenheit"),
            formula: "(input * 9.0/5.0) + 32.0",
        },
        Conversion {
            index: String::from("2"),
            from: String::from("Celsius"),
            to: String::from("Kelvin"),
            formula: "input + 273.0",
        },
        Conversion {
            index: String::from("3"),
            from: String::from("Fahrenheit"),
            to: String::from("Celsius"),
            formula: "(input - 32) * 5.0/9.0",
        },
        Conversion {
            index: String::from("4"),
            from: String::from("Fahrenheit"),
            to: String::from("Kelvin"),
            formula: "((input - 32) * 5.0/9.0) + 273",
        },
        Conversion {
            index: String::from("5"),
            from: String::from("Kelvin"),
            to: String::from("Celsius"),
            formula: "input - 273",
        },
        Conversion {
            index: String::from("6"),
            from: String::from("Kelvin"),
            to: String::from("Fahrenheit"),
            formula: "((input -273) * 9.0/5.0) + 32.0",
        },
    ];

    let pressure_conversions: [Conversion; 12] = [
        Conversion {
            index: String::from("1"),
            from: String::from("Atm"),
            to: String::from("Torr"),
            formula: "input * 760.0",
        },
        Conversion {
            index: String::from("2"),
            from: String::from("Atm"),
            to: String::from("Pascal"),
            formula: "input * 101325.0",
        },
        Conversion {
            index: String::from("3"),
            from: String::from("Atm"),
            to: String::from("mmHg"),
            formula: "input * 760.0",
        },
        Conversion {
            index: String::from("4"),
            from: String::from("Torr"),
            to: String::from("Atm"),
            formula: "input / 760.0",
        },
        Conversion {
            index: String::from("5"),
            from: String::from("Torr"),
            to: String::from("Pascal"),
            formula: "(input / 760.0) * 101325.0",
        },
        Conversion {
            index: String::from("6"),
            from: String::from("Torr"),
            to: String::from("mmHg"),
            formula: "input",
        },
        Conversion {
            index: String::from("7"),
            from: String::from("Pascal"),
            to: String::from("Atm"),
            formula: "input / 101325.0",
        },
        Conversion {
            index: String::from("8"),
            from: String::from("Pascal"),
            to: String::from("Torr"),
            formula: "(input * 760.0) / 101325.0",
        },
        Conversion {
            index: String::from("9"),
            from: String::from("Pascal"),
            to: String::from("mmHg"),
            formula: "(input * 760.0) / 101325.0",
        },
        Conversion {
            index: String::from("10"),
            from: String::from("mmHg"),
            to: String::from("Atm"),
            formula: "input / 760.0",
        },
        Conversion {
            index: String::from("11"),
            from: String::from("mmHg"),
            to: String::from("Torr"),
            formula: "input",
        },
        Conversion {
            index: String::from("12"),
            from: String::from("mmHg"),
            to: String::from("Pascal"),
            formula: "(input / 760.0) * 101325.0",
        },
    ];

    let conversion_categories: [(&str, &str, &[Conversion]); 4] = [
        ("1", "Area Conversions", &area_conversions),
        ("2", "Length Conversions", &length_conversions),
        ("3", "Pressure Conversions", &pressure_conversions),
        ("4", "Temperature Conversions", &temp_conversions),
    ];
    // Functions
    // Arguments
    fn phelp(version: &str) {
        println!("RConv {}, GNU LICENSE v3", version);
        println!("A Command Line Unit Converter written in Rust.");
        println!();
        println!("Usage:  rconv [options]");
        println!();
        println!("Options:");
        println!("\t-h, --help           Prints the help.");
        println!("\t-l, --list           Prints the available conversion categories.");
        println!("\t-c, --category [num] Choose the conversion category.");
    }
    let mut args: Vec<String> = args().collect();

    for arg in 0..args.len() {
        if args[arg] == "-h" || args[arg] == "--help" {
            phelp(version);
            exit(0);
        } else if args[arg] == "-c1" {
            category_choice = "1".to_string();
        } else if args[arg] == "-c2" {
            category_choice = "2".to_string();
        } else if args[arg] == "-c3" {
            category_choice = "3".to_string();
        } else if args[arg] == "-c4" {
            category_choice = "4".to_string();
        } else if args[arg] == "-l" || args[arg] == "--list" {
            print!("\x1b[92m\x1b[1m");
            for category in conversion_categories.iter() {
                println!("{}: {}", category.0, category.1);
            }
            exit(0);
        } else if args[arg] == "-c" || args[arg] == "--category" {
            if arg < args.len() - 1 {
                if args[arg + 1] == "-h" || args[arg + 1] == "--help" {
                    phelp(version);
                    exit(0);
                } else {
                    let mut choice: &mut String;
                    choice = &mut args[arg + 1];
                    category_choice = (choice.parse::<usize>().unwrap()).to_string();
                }
            } else {
                eprintln!("No value provided!!");
                exit(1);
            }
        } else if args[arg] == "-ch" || args[arg] == "-hc" {
            phelp(version);
            exit(0);
        }
    }
    fn choose_category(conversions: &[Conversion]) {
        let mut local_input_string = String::new();
        let mut local_choice: &Conversion;
        let mut input_choice: usize;
        println!("Number of Choices: {}", conversions.len());
        for conversion in conversions.iter() {
            println!(
                "{} : {} to {}",
                conversion.index, conversion.from, conversion.to
            )
        }

        println!("Select an option:");
        io::stdin()
            .read_line(&mut local_input_string)
            .expect("Error!!");
        local_input_string.pop();
        if local_input_string == "" {
            eprintln!("No Input!!!");
            exit(1)
        }
        input_choice = local_input_string.parse::<usize>().unwrap() - 1;

        if (input_choice + 1) > conversions.len() {
            eprintln!("Too Big!!!");
            exit(1);
        } else {
        }

        local_choice = (&conversions[local_input_string.parse::<usize>().unwrap() - 1]);
        let local_types: Vec<&str>;
        let input_type: &String;
        let output_type: &String;

        input_type = &local_choice.from;
        output_type = &local_choice.to;

        // Calculation Variables
        let mut input_value = String::new();
        let input_int: f64;
        let mut output_value: f64 = 0.0;

        println!("Enter the value ({}):", input_type);
        io::stdin().read_line(&mut input_value).expect("Error!!");
        input_value.pop();
        if input_value == "" {
            eprintln!("No Input!!!");
            exit(1)
        }
        input_int = input_value.parse::<f64>().unwrap();

        for i in 0..conversions.len() {
            let conv = &conversions[i];
            if input_type == &conv.from && output_type == &conv.to {
                output_value = eval(&(conv.formula)[..].replace("input", &input_value[..]))
                    .unwrap()
                    .to_string()
                    .parse::<f64>()
                    .unwrap()
            }
        }
        println!("{}: {}", input_type, input_value);
        println!("{}: {}", output_type, output_value);
    }
    print!("\x1b[92m\x1b[1m");
    println!("Welcome to RConv!");

    if category_choice == "0" {
        for category in conversion_categories.iter() {
            println!("{}: {}", category.0, category.1);
        }
        println!("Select the category:");
        io::stdin().read_line(&mut category_choice).expect("Error!");
        category_choice.remove(0);
        category_choice.pop();
    } else {
    }
    if category_choice.parse::<usize>().unwrap() - 1 < conversion_categories.len() {
        for category in conversion_categories.iter() {
            if category_choice == category.0 {
                choose_category(&category.2);
            }
        }
    } else {
        println!("Wrong Choice!!");
    }
}
