fn main(){

    let input_number:i64 = 23;
    // Demonstation decision statement "if" "else"
    if input_number > 0 {
        println!("The Input Number {} is a Positive Number",input_number);
    }else{
        println!("The input Number {} is not a Positive number",input_number);
    }

    // Demonstation decision statement "if" "else if"
    if input_number > 0 {
        println!("The Input number {} is a +ve Number",input_number);
    }else if input_number < 0 {
        println!("The Input Number {} is a -ve Number",input_number);
    }else{
        println!("The Input Number {} is a Neither +ve or -ve Number",input_number);
    }

    let match_number = match input_number > 0 {
        true => println!("The Input Number is a +ve Number"),
        _ => println!("The Input Number is a -ve Number")
    };
    //println!("{} -> {}", input_number, match_number.to_string());

    let binary_input = 1;
    let binary = match binary_input {
        1 => println!("Switch turned ON"),
        _ => println!("Switch turned OFF")
    };
    //println!("{} -> {}",binary_input,binary.to_string())
}