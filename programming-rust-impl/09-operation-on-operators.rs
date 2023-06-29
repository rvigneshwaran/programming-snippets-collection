fn main(){
    let mut first_input = 25;
    let mut second_input = 5;

    // Arthimatic Operators
    println!("\n Arthimatic Operators");
    println!("The Operation first_input + second_input :: {}",first_input+second_input);
    println!("The Operation first_input - second_input :: {}",first_input-second_input);
    println!("The Operation first_input * second_input :: {}",first_input*second_input);
    println!("The Operation first_input / second_input :: {}",first_input/second_input);
    println!("The Operation first_input % second_input :: {}",first_input%second_input);

    // Relational Operators
    println!("\n Relational Operators");
    println!("The Operation first_input > second_input :: {}",first_input>second_input);
    println!("The Operation first_input < second_input :: {}",first_input<second_input);
    println!("The Operation first_input >= second_input :: {}",first_input>=second_input);
    println!("The Operation first_input <= second_input :: {}",first_input<=second_input);
    println!("The Operation first_input == second_input :: {}",first_input==second_input);
    println!("The Operation first_input != second_input :: {}",first_input != second_input);

    // Logical Operators
    println!("\n Logical Operators");
    println!("The Operation first_input > 0 && second_input > 0 :: {}",first_input > 0 && second_input > 0);
    println!("The Operation first_input > 0 || second_input > 0 :: {}",first_input > 0 || second_input > 0);
    println!("The Operation !(first_input < 0) :: {}",!(first_input < 0));

    // Bitwise Operators
    first_input = 5;
    second_input = 3;
    println!("\n Bitwise Operators");
    println!("The Operation first_input & second_input :: {}",first_input & second_input);
    println!("The Operation first_input | second_input :: {}",first_input | second_input);
    println!("The Operation first_input ^ second_input :: {}",first_input ^ second_input);

    // Bitwise Operators
    first_input = 5;
    second_input = 1;
    println!("The Operation first_input << second_input :: {}",first_input << second_input);
    println!("The Operation first_input >> second_input :: {}",first_input >> second_input);

    // Bitwise Operators
    first_input = 40;
    second_input = 3;
    println!("The Operation first_input << second_input :: {}",first_input << second_input);
    println!("The Operation first_input >> second_input :: {}",first_input >> second_input);

}