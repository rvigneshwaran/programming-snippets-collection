fn main(){

    // Demontration of the constant definition using a Single Precision Float Value
    const PI:f32 = 3.14;
    println!("Constant Definition :: {} ",PI);

    // ERROR TRIGGER : creating constants without datatype :: missing type for `const` item
    //const DEFAULT_TEMP = 45;

    // ERROR TRIGGER : Shadowing Constant values :: error[E0428]: the name `PI` is defined multiple times
    //const PI:f32 = 3.14159;
}