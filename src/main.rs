use chrono::NaiveDateTime;
use std::io;

fn main() {
    let mut input_date: NaiveDateTime = Default::default();
    let mut input = String::new();
    println!("{}", input);
    while input_date == Default::default(){
        println!("Enter a date: ");
        input = read_input();
        match NaiveDateTime::parse_from_str(&input, "%d/%m/%Y %H:%M" ){
            Ok(date) => {
                input_date = date;
            },
            Err(_) => {
                println!("Invalid date format. Please use DD/MM/YYY HH:MM");
            }
        };
    };
    println!("input: {}", input_date);
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}