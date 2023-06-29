// Demonstation of the function capablities.

// Function declaration without the parameters.
fn display_string(){
    println!("This is a display output from the function without parameters");
}

// Function Declaration with Return Type Defined.
fn get_number() -> i16 {
    return 45;
}

// Function Declaration with Pass By Value
fn calculate(input_num1:i64,input_num2:i64,operation:String) -> i64 {
    let mut operation_result:i64 = 0;
    if operation == "+" {
        operation_result = input_num1 + input_num2;
    }else if operation == "-" {
        operation_result = input_num1 - input_num2;
    }else if operation == "*" {
        operation_result =  input_num1 * input_num2;
    }else if operation == "/" {
        operation_result = input_num1 / input_num2;
    }else if operation == "%" {
        operation_result = input_num1 % input_num2;
    }
    return operation_result;
}

// Function Declaration with Pass By Reference
fn pbr_calculate(input_num1:&mut i64,input_num2:&mut i64,operation:String) -> i64 {
    let mut operation_result:i64 = 0;
    if operation == "+" {
        operation_result = *input_num1 + *input_num2;
    }else if operation == "-" {
        operation_result = *input_num1 - *input_num2;
    }else if operation == "*" {
        operation_result =  *input_num1 * *input_num2;
    }else if operation == "/" {
        operation_result = *input_num1 / *input_num2;
    }else if operation == "%" {
        operation_result = *input_num1 % *input_num2;
    }
    // Derefencing the variables in Pass By reference
    *input_num1 = 0;
    *input_num2 = 0;
    return operation_result;
}

fn main(){

    // Invocation of function without input parameters
    display_string();

    // Invocation of the function which has a return type defined.
    let random_number = get_number();
    println!("The output of the random_number generated is :: {}",random_number);

    // Invoke of function with input parameters - Pass By Value
    let input_number1 = 100000;
    let input_number2 = 60000;
    let sum_operation = "+";
    let diff_operation:String = String::from("-");
    let prod_operation:String = String::from("*");
    let div_operation:String = String::from("/");
    let mod_operation:String = String::from("%");
    let mut output_operation = calculate(input_number1,input_number2,sum_operation.to_string());
    println!("The Value of output after sum_operation is {} ",output_operation);
    output_operation = calculate(input_number1,input_number2,diff_operation);
    println!("The Value of output after diff_operation is {} ",output_operation);
    output_operation = calculate(input_number1,input_number2,prod_operation);
    println!("The Value of output after prod_operation is {} ",output_operation);
    output_operation = calculate(input_number1,input_number2,div_operation);
    println!("The Value of output after div_operation is {} ",output_operation);
    output_operation = calculate(input_number1,input_number2,mod_operation);
    println!("The Value of output after mod_operation is {} ",output_operation);
    println!("");
    let mut input_number3 = 200000;
    let mut input_number4 = 120000;
    output_operation = pbr_calculate(&mut input_number3,&mut input_number4,sum_operation.to_string());
    println!("The Value of output after sum_operation::Pass By Reference is {} ",output_operation);
    println!("The value of the input variables has been de-referenced in pbf_calculate function -> ");
    println!("The Value of input_number3 is {} ",input_number3);
    println!("The Value of input_number4 is {} ",input_number4);
}