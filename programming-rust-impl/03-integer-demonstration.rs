fn main(){

    let input_integer = 45; // Default is Signed 32 Bit Intger
    let input_unsigned_int1:u32 = 32;
    let input_signed_int2:i32 = 45;
    let input_si_from_arch:isize = 34;
    let input_un_from_arch:usize = 89;

    println!("Input Integer :: {} (Default Definition :: Signed 32bit Integer)",input_integer);
    println!("Input Integer :: {} (Unsigned 32bit Integer)",input_unsigned_int1);
    println!("Input Integer :: {} (Signed 32bit Integer)",input_signed_int2);
    println!("Input Integer :: {} (Signed Size Derived from Architecture)",input_si_from_arch);
    println!("Input Integer :: {} (UnSigned Size Derived from Architecture)",input_un_from_arch);

    // DEMONSTATION OF TRIGGER_ERROR :: MISMATCH IN VARIABLE ASSIGNMENT
    // Lets try to assign the Integer with a flowting point value, we will land up in the below error
    // error[E0308]: mismatched types
    // let trigger_error:i32 = 45.023;

    // DEMONSTRATION OF TRIGGER_ERROR :: OVERFLOW OF INTEGERS
    //let overflow_int_assignment1:u8 = 256; // Range of 8bit Integer is from 0 to 255
    //let overflow_int_assignment2:u16 = 65535; // Range of 16 bit Integer is 0 to 65535
    //let overflow_int_assignment3:u32 = 4294967296; // Range of 16 bit Integer is 0 to 4294967296
    //println!("Input Integer Overflow Assignment::overflow_int_assignment1 {}",overflow_int_assignment1);
    //println!("Input Integer Overflow Assignment::overflow_int_assignment2 {}",overflow_int_assignment2);
    //println!("Input Integer Overflow Assignment::overflow_int_assignment3 {}",overflow_int_assignment3);
}