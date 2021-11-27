#![allow(unused)]
use std::io;
use std::process;

fn main() {
    let mut category_choice = String::new();

    let area_conversions: [[&str; 2]; 16] = [
      ["1","Square Meter to Square Mile"],
      ["2","Square Meter to Square Foot"],
      ["3","Square Meter to Square Yard"],
      ["4","Square Meter to Square Inch"],
      ["5","Square Mile to Square Meter"],
      ["6","Square Mile to Square Foot"],
      ["7","Square Mile to Square Yard"],
      ["8","Square Mile to Square Inch"],
      ["9","Square Foot to Square Meter"],
      ["10","Square Foot to Square Mile"],
      ["11","Square Foot to Square Yard"],
      ["12","Square Foot to Square Inch"],
      ["13","Square Inch to Square Meter"],
      ["14","Square Inch to Square Mile"],
      ["15","Square Inch to Square Foot"],
      ["16","Square Inch to Square Yard"]
    ];



    let length_conversions:[[&str;2];8] = [
        ["1","Km to Mile"],
        ["2","Km to Meter"],
        ["3","Mile to Km"],
        ["4","Mile to Meter"],
        ["5","Inch to Meter"],
        ["6","Inch to Foot"],
        ["6","Foot to Inch"],
        ["8","Foot to Meter"]
    ];

    let temp_conversions:[[&str;2];6] = [
        ["1","Celsius to Fahrenheit"],
        ["2","Celsius to Kelvin"],
        ["3","Fahrenheit to Celsius"],
        ["4","Fahrenheit to Kelvin"],
        ["5","Kelvin to Celsius"],
        ["6","Kelvin to Fahrenheit"]
    ];


    let pressure_conversions:[[&str;2];12] = [
        ["1","Atm to Torr"],
        ["2","Atm to Pascal"],
        ["3","Atm to mmHg"],
        ["4","Torr to Atm"],
        ["5","Torr to Pascal"],
        ["6","Torr to mmHg"],
        ["7","Pascal to Atm"],
        ["8","Pascal to Torr"],
        ["9","Pascal to mmHg"],
        ["10","mmHg to Atm"],
        ["11","mmHg to Torr"],
        ["12","mmHg to Pascal"]
    ];






    let conversion_categories:((&str,&str,[[&str; 2]; 16]),(&str, &str, [[&str; 2]; 8]),(&str, &str, [[&str; 2];6]),(&str,&str, [[&str;2];12])) = (
        (
            "1",
            "Area Conversions",
            area_conversions
            ),
        (
            "2",
            "Length Conversions",
            length_conversions
        ),
        (
            "3",
            "Temperature Conversions",
            temp_conversions
        ),
        (
            "4",
            "Pressure Conversions",
            pressure_conversions
            )
    );




    // Functions
    fn choose_category(conversions: &[[&str; 2]]){
        let mut local_input_string  = String::new();
        let mut local_choice = String::new();
        let mut input_choice: usize;
        println!("Number of Choice: {}",conversions.len());
        for conversion in conversions.iter(){
            println!("{} : {}", conversion[0],conversion[1])
        }
        println!("Select an option:");
        io::stdin().read_line(&mut local_input_string).expect("Error!!");
        local_input_string.pop();
        input_choice = local_input_string.parse::<usize>().unwrap() - 1;

        if (input_choice+1)>conversions.len(){
            println!("Wrong Choice!!");
            process::exit(1);
        }else {}

        local_choice = (&conversions[local_input_string.parse::<usize>().unwrap() - 1][1]).to_string();
        let local_types: Vec<&str>;
        let input_type :&str;
        let output_type :&str;


        local_types = local_choice.split(" to ").collect();
        input_type = local_types[0];
        output_type = local_types[1];



        // Calculation Variables
        let mut input_value = String::new();
        let input_int: f64;
        let mut output_value: f64 =0.0;






        println!("Enter the value ({}):",input_type);
        io::stdin().read_line(&mut input_value).expect("Error!!");
        // println!("The length is {}",input_value.len());
        input_value.pop();
        // println!("The length is {}",input_value.len());
        // println!("Output Type : {}",output_type);
        input_int = input_value.parse::<f64>().unwrap();

        if input_type=="Celsius"{
            if output_type == "Fahrenheit"{
                output_value = (input_int * 9.0/5.0)+32.0;
            }
            else if output_type == "Kelvin"{
                output_value = input_int + 273.0;
            }
        }else if input_type=="Fahrenheit"{
            if output_type == "Celsius"{
                output_value = 5.0/9.0 *(input_int-32.0);
            }
            else if output_type == "Kelvin"{
                output_value = (5.0/9.0 *(input_int-32.0)) + 273.0;
            }
        }else if input_type=="Kelvin"{
            if output_type == "Celsius"{
                output_value = input_int - 273.0;
            }
            else if output_type == "Fahrenheit"{
                output_value = ((input_int - 273.0)* 9.0/5.0)+32.0;
            }
        }else if input_type =="Km"{
            if output_type == "Mile" {
                output_value = input_int / 1.609344;
            }
        }else if input_type == "Mile"{
            if output_type == "Km" {
                output_value = input_int * 1.609344;
            }
        }else if input_type == "Inch"{
            if output_type == "Meter" {
                output_value = input_int / 39.37;
            }
            else if output_type == "Foot"{
                output_value = input_int / 12.0;
            }
        }else if input_type == "Foot" {
            if output_type == "Meter"{
                output_value =  input_int / 3.281;
            }
            else if output_type == "Inch"{
                output_value = input_int / 12.0;
            }
        }else if input_type == "Atm"{
            if output_type == "Pascal"{
                output_value = input_int * 101325.0;
            }
            else if (output_type == "Torr") ||(output_type == "mmHg") {
                output_value = input_int * 760.0;
            }
        }else if input_type == "Pascal" {
            if output_type == "Torr" ||(output_type == "mmHg")  {
                output_value = (input_int / 101325.0) * 760.0 ;
            }else if output_type == "Atm" {
                output_value = input_int / 101325.0;
            }
        }else if input_type == "Torr" {
            if output_type == "Atm"{
                output_value = input_int / 760.0;
            }else if output_type == "Pascal" {
                output_value = (input_int / 760.0) * 101325.0;
            }else if output_type == "mmHg"{
                output_value = input_int;
            }
        }else if input_type == "mmHg"{
            if output_type == "Atm"{
                output_value = input_int / 760.0;
            }
            else if output_type == "Torr"{
                output_value = input_int;
            }
            else if output_type == "Pascal"{
                output_value = (input_int / 760.0) * 101325.0;
            }
        }else if input_type == "Square Meter"{
            if output_type == "Square Mile"{
                output_value = input_int / 2.59e+6;
            }else if output_type == "Square Yard"{
                output_value = input_int * 1.196;
            }else if output_type == "Square Foot"{
                output_value = input_int * 10.764;
            }else if output_type == "Square Inch"{
                output_value = input_int * 1550.0;
            }
        }



        else if input_type == "Square Mile"{
            if output_type == "Square Meter"{
                output_value = input_int * 2.59e+6;
            }
            else if output_type == "Square Yard"{
                output_value = input_int * 3.098e+6;
            }
            else if output_type == "Square Foot"{
                output_value = input_int * 2.788e+7;
            }
            else if output_type == "Square Inch"{
                output_value = input_int * 4.014e+9;
            }
        }else if input_type == "Square Yard"{
            if output_type == "Square Meter"{
                output_value = input_int / 1.196;
            }
            else if output_type == "Square Mile"{
                output_value = input_int / 3.098e+6;
            }
            else if output_type == "Square Foot"{
                output_value = input_int * 9.0;
            }
            else if output_type == "Square Inch"{
                output_value = input_int * 1296.0;
            }
        }else if input_type == "Square Foot"{
            if output_type == "Square Meter"{
                output_value = input_int / 10.764;
            }
            else if output_type == "Square Mile"{
                output_value = input_int / 2.788e+7;
            }
            else if output_type == "Square Yard"{
                output_value = input_int / 9.0;
            }
            else if output_type == "Square Inch"{
                output_value = input_int * 144.0;
            }
        }else if input_type == "Square Inch"{
            if output_type == "Square Meter"{
                output_value = input_int / 1550.0;
            }
            else if output_type == "Square Mile"{
                output_value = input_int / 4.014e+9;
            }
            else if output_type == "Square Yard"{
                output_value = input_int / 1296.0;
            }
            else if output_type == "Square Foot"{
                output_value = input_int  / 144.0;
            }
        }



        else {
            output_value = 0.0;
        }


        println!("{}: {}",input_type,input_value);
        println!("{}: {}",output_type,output_value);
    }






    print!("\x1b[92m\x1b[1m");
    println!("Welcome to UConv!");
    // println! ("Conversion Category 1: {:?}",conversion_categories);
    // for conversion_category in conversion_categories.iter(){
    //     println!("{}: {}",conversion_category.0,conversion_category.1);
    // }

    println!("{}: {}", conversion_categories.0.0, conversion_categories.0.1);
    println!("{}: {}", conversion_categories.1.0, conversion_categories.1.1);
    println!("{}: {}", conversion_categories.2.0, conversion_categories.2.1);
    println!("{}: {}", conversion_categories.3.0, conversion_categories.3.1);


    println!("Select the category:");
    io::stdin().read_line(&mut category_choice).expect("Error!");
    category_choice.pop();

    if category_choice == "1"{
        choose_category(&area_conversions);
    }else if category_choice == "2"{
        choose_category(&length_conversions);
    }else if category_choice == "3" {
        choose_category(&temp_conversions);
    }else if category_choice == "4" {
        choose_category(&pressure_conversions);
    }
    else {
        println!("Wrong Choice!");
        process::exit(1);
    }
    // println! ("Choice: {}",choice);
        //
    // if category_choice == "1"{
    //     println!("Number of Choice: {}",area_conversions.len());
    //     for conversion in area_conversions.iter(){
    //         println!("{} : {}", conversion[0],conversion[1])
    //     }
    //     println!("Select an option:");
    //     io::stdin().read_line(&mut input_str).expect("Error!!");
    //     input_str.pop();
    //     choice = area_conversions[input_str.parse::<usize>().unwrap() - 1][1];
    // }
    // else if category_choice == "3" {
    //     println!("Number of Choices: {}", temp_conversions.len());
    //     for conversion in temp_conversions.iter() {
    //         println!("{} : {}  ", conversion[0], conversion[1]);
    //     }
    //     println!("Select an option:");
    //     io::stdin().read_line(&mut input_str).expect("Error!!");
    //     input_str.pop();
    //     choice = temp_conversions[input_str.parse::<usize>().unwrap() - 1][1];
    // }
    //
    //
    //
    //
    // else if category_choice == "2" {
    //     println!("Number of Choices : {}", length_conversions.len());
    //     for conversion in length_conversions.iter(){
    //         println!("{} : {}  ",conversion[0], conversion[1]);
    //     }
    //     println!("Select an option:");
    //     io::stdin().read_line(&mut input_str).expect("Error!");
    //     input_str.pop();
    //     choice = length_conversions[input_str.parse::<usize>().unwrap() -1][1];
    // }
    //
    //
    // else if category_choice == "4" {
    //     println!("Number of Choices : {}", length_conversions.len());
    //     for conversion in pressure_conversions.iter(){
    //         println!("{} : {}  ",conversion[0], conversion[1]);
    //     }
    //     println!("Select an option:");
    //     io::stdin().read_line(&mut input_str).expect("Error!");
    //     input_str.pop();
    //     choice = pressure_conversions[input_str.parse::<usize>().unwrap() -1][1];
    // }




    // types = choice.split_whitespace().collect();
    // input_type = types[0];
    // output_type = types[2];



}