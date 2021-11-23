use std::io;
use std::process;

fn main() {
    let mut input_str = String::new();
    // let mut input_num:f64 =0.0;
    let mut choice:&str = "";
    let types: Vec<&str>;
    let input_type: &str;
    let output_type: &str;
    let mut input_value = String::new();
    let input_int: f64;
    let mut output_value: f64 =0.0;
    let mut category_choice = String::new();
    // let mut current_category = String::new();


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


    let pressure_conversions:[[&str;2];6] = [
        ["1","Atm to Pascal"],
        ["2","Atm to Torr"],
        ["3","Torr to Pascal"],
        ["4","Torr to Atm"],
        ["5","Pascal to Torr"],
        ["6","Pascal to Atm"]
    ];


    let conversion_categories:((&'static str, &'static str, [[&str; 2]; 8]),(&'static str, &'static str, [[&str; 2];6]),(&str,&str, [[&str;2];6])) = (
        (
            "1",
            "Length Conversions",
            length_conversions
        ),
        (
            "2",
            "Temperature Conversions",
            temp_conversions
        ),
        (
            "3",
            "Pressure Conversions",
            pressure_conversions
            )
    );


    print!("\x1b[92m\x1b[1m");
    println!("Welcome to UConv!");
    // println! ("Conversion Category 1: {:?}",conversion_categories);
    // for conversion_category in conversion_categories.iter(){
    //     println!("{}: {}",conversion_category.0,conversion_category.1);
    // }

    println!("{}: {}", conversion_categories.0.0, conversion_categories.0.1);
    println!("{}: {}", conversion_categories.1.0, conversion_categories.1.1);
    println!("{}: {}", conversion_categories.2.0, conversion_categories.2.1);


    println!("Select the category:");
    io::stdin().read_line(&mut category_choice).expect("Error!");
    category_choice.pop();

    if category_choice == "2" {
        println!("Number of Choices: {}", temp_conversions.len());
        for conversion in temp_conversions.iter() {
            println!("{} : {}  ", conversion[0], conversion[1]);
        }
        println!("Select an option:");
        io::stdin().read_line(&mut input_str).expect("Error!!");
        input_str.pop();
        choice = temp_conversions[input_str.parse::<usize>().unwrap() - 1][1];
    }else if category_choice == "1" {
        println!("Number of Choices : {}", length_conversions.len());
        for conversion in length_conversions.iter(){
            println!("{} : {}  ",conversion[0], conversion[1]);
        }
        println!("Select an option:");
        io::stdin().read_line(&mut input_str).expect("Error!");
        input_str.pop();
        choice = length_conversions[input_str.parse::<usize>().unwrap() -1][1];
    }else if category_choice == "3" {
        println!("Number of Choices : {}", length_conversions.len());
        for conversion in pressure_conversions.iter(){
            println!("{} : {}  ",conversion[0], conversion[1]);
        }
        println!("Select an option:");
        io::stdin().read_line(&mut input_str).expect("Error!");
        input_str.pop();
        choice = pressure_conversions[input_str.parse::<usize>().unwrap() -1][1];
    }
    else {
        println!("Wrong Choice!");
        process::exit(1);
    }



    types = choice.split_whitespace().collect();
    input_type = types[0];
    output_type = types[2];

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
        else if output_type == "Torr"{
            output_value = input_int * 760.0;
        }
    }else if input_type == "Pascal" {
        if output_type == "Torr" {
            output_value = input_int / 133.0 ;
        }else if output_type == "Atm" {
            output_value = input_int / 101325.0;
        }
    }else if input_type == "Torr" {
        if output_type == "Atm"{
            output_value = input_int / 760.0;
        }else if output_type == "Pascal" {
            output_value = input_int * 133.0;
        }
    }



    else {
        output_value = 0.0;
    }


    println!("{}: {}",input_type,input_value);
    println!("{}: {}",output_type,output_value);
}