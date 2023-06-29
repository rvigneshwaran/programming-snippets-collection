fn main(){

    // Declaration of Input Variables.
    let input_variable = "Sample Input text 1";
    let inpt_variabe:i32 = 34;
    println!("Default assignemnt of the Input Varaible without datatype :: {}",input_variable);
    println!("Input Variable with Datatype :: {}",inpt_variabe);

    // Demonstation of the Immutable and Mutable variable
    // TRIGGER ERROR : IMMUTABLE VARIABLE ASSIGNED AGAIN
    // input_variable = "Sample Input text 2";

    // Definition of the Mutable variable
    let mut input_variable1 = "Sample Input Text 3";
    println!("Variable Initialization value :: {}",input_variable1);
    // Updating the Mutable variable
    input_variable1 = "Updated :: Sample Input Text 3";
    println!("Variable Updation value :: {}",input_variable1);


    // Shadowing of the variable
    // we can shadow the previous value of the Immutable variable by using the let keyword
    let input_variable = "Updated :: Sample Input text 1";
    println!("Shadowed Immutable Input Variable  :: {}",input_variable);
    // Shadowing the variable with different data type
    let input_variable = 43;
    println!("Shadowing Immutable Input Variable with Different DATA TYPE :: {}",input_variable)
}