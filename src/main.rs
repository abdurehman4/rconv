use std::io;

fn main() {
    let mut input_str = String::new();
    let mut input_num:usize =0;
    let mut conversion_index:usize = 1;
    let mut choice:&str;
    let mut types: Vec<&str>;
    let mut input_type: &str;
    let mut output_type: &str;
    let mut input_value = String::new();
    let mut input_int: usize;
    let mut output_value: usize = 0;

    let conversions :[&str;6] = [
        "Celsius to Fahrenheit",
        "Fahrenheit to Celsius",
        "Celsius to Kelvin",
        "Kelvin to Celsius",
        "Fahrenheit to Kelvin",
        "Kelvin to Fahrenheit"
    ];
    println!("Welcome to UConv!");
    println!("Number of conversions: {}",conversions.len());
    for conversion in conversions.iter() {
        println!("{} : {}  ",conversion_index,conversion);
        conversion_index = conversion_index+1;
    }
    println!("Select an option:");
    io::stdin().read_line(&mut input_str).expect("Error!!");
    input_str.remove(1);
    choice = conversions[input_str.parse::<usize>().unwrap()-1];

    types = choice.split_whitespace().collect();
    input_type = types[0];
    output_type = types[2];

    println!("Enter the value ({}):",input_type);
    io::stdin().read_line(&mut input_value).expect("Error!!");
    // println!("The length is {}",input_value.len());
    input_value.pop();
    // println!("The length is {}",input_value.len());
    // println!("Output Type : {}",output_type);
    input_int = input_value.parse::<usize>().unwrap();

    if input_type=="Celsius"{
        if output_type == "Fahrenheit"{
            output_value = (input_int * 9/5)+32;
        }
        if output_type == "Kelvin"{
            output_value = input_int + 273;
        }
    }else if input_type=="Fahrenheit"{
        if output_type == "Celsius"{
            output_value = 5/9 *(input_int-32);
        }
        if output_type == "Kelvin"{
            output_value = (5/9 *(input_int-32)) + 273;
        }
    }else if input_type=="Kelvin"{
        if output_type == "Celsius"{
            output_value = input_int - 273;
        }
        if output_type == "Fahrenheit"{
            output_value = (    (input_int - 273)* 9/5)+32;
        }
    }
    else {
        output_value = 0;
    }


    println!("{}: {}",input_type,input_value);
    println!("{}: {}",output_type,output_value);
}