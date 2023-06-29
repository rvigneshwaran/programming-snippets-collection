fn main(){

    // Demonstation of the definite loop Statements
    for indv_element in 10..20{
        if indv_element == 12 || indv_element == 14 {
            println!("Element is Skipped at index {}",indv_element);
            continue;
        }
        println!("FOR LOOP :: Index at the element {}",indv_element);
    }

    let mut index = 0;
    while index < 10 {
        println!("WHILE LOOP :: Element is at the index {} ",index);
        index=index+1;
    }

    let mut loop_index = 10;
    loop{
        println!("LOOP :: Element is at the index {} ",loop_index);
        loop_index=loop_index+1;
        if loop_index > 20 {
            break;
        }
    }
}