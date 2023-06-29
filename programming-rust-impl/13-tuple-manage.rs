// Tuple is a Compound data type where we can store all types of variable
// Compound Data Type - Store Multiple values of different Type.

fn assign_values(input_tup:(i32,f64,f64)){
    // assigning the values of tuple to different values
    let (value_1,value_2,value_3)=input_tup;
    println!("First Value from Tuple :: {}",value_1);
    println!("Second Value from Tuple :: {}",value_2);
    println!("Third Value from Tuple :: {}",value_3);
}

fn main(){
    // Tuple definition with the data types.
    let closed_tup:(&str,&str,&str)=("Rust","Python","Java");
    println!("{:?}",closed_tup);
  
    // Tuple definition without data types.
    let open_tup=("Rust","Python","Java","Ruby");
    println!("{:?}",open_tup);
  
    // Tuple definition of different data types.
    let diff_tup:(i32,f64,u8)=(12,12.26,22);
    println!("{:?}",diff_tup);

    // Retrive tuple through index - Index Starts from 0
    println!("Printing the first element in Tuple diff_tup :: {:?} ",diff_tup.0);
    println!("Printing the second element in Tuple diff_tup :: {:?} ",diff_tup.1);
    println!("Printing the third element in Tuple diff_tup :: {:?} ",diff_tup.2);

    // Passing the tuple as parameter to the function and getting values
    let function_pass_tup:(i32,f64,f64)=(21,21.22,43.65);
    assign_values(function_pass_tup);
  }