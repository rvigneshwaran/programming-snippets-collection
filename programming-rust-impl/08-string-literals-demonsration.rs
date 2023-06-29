

fn main(){
    
    // Definition of the String literal.
    let temple_name:&str = "Thanjai Periya Kovil";
    let temple_location:&str = "South bank of Cauvery river in Thanjavur, Tamil Nadu";
    println!("The {} Temple is location at {}.",temple_name,temple_location);

    // String literal are static by default.
    let architecture:&'static str = "Tamil Architecture";
    let emperor:&'static str = "Raja Raja Cholan";
    println!("Architecture : {} Build By Emperor : {}. ",architecture,emperor);

    // Definition of the String Object creation - 2 Methods 
    // 1. Using New - String::new()
    // 2. One from String Literal - String::from()
    let new_string = String::new();
    println!("The value of new_string is :: {}",new_string);
    let string_from_literal = String::from("Rajarajeswaram");
    println!("The value of string_from_literal is :: {}",string_from_literal);

    // Find the length of the string using len()
    println!("The length of new_string is :: {}",new_string.len());
    println!("The length of string_from_literal is :: {}",string_from_literal.len());

    // How to add strings to the string object
    let mut sample_string_instance = String::new();
    sample_string_instance.push_str("11th-century temple");
    println!("The value of sample_string_instance is :: {}",sample_string_instance);
    sample_string_instance.push_str(", Walls after 16th century");
    println!("The updated value of sample_string_instance is :: {}",sample_string_instance);

    // To access all the methods of the String Object we have to convert literal to String using to_string()
    let arch_tech = "axial and symmetrical geometry rules";
    println!("The length of arch_tech is :: {}",arch_tech.len());
    let replaced = arch_tech.replace("and",", circumambulation, ");
    println!("Updated 1 :: The value of arch_tech is :: {}",replaced);
    let first_one = "Keralantakan tiruvasal".to_string();
    println!("The value of first_one is :: {}",first_one);

    // trim the input String contends
    let mut inner_courtyard = "                Rajarajan tiruvasal            ";
    println!("The length of inner_courtyard BEFORE TRIM is :: {}",inner_courtyard.len());
    inner_courtyard = inner_courtyard.trim();
    println!("The length of inner_courtyard AFTER TRIM is :: {}",inner_courtyard.len());

    // Split by white Spaces using .split_whitespace()
    let _preservation = "Archaeological Survey of India (ASI)";
    let mut no_of_words = 1;
    for indv_word in _preservation.split_whitespace(){
        println!("{} -> {}",no_of_words,indv_word);
        no_of_words=no_of_words+1;
    }
    println!("The Total No of Words Present in no_of_words is {} ",no_of_words-1);

    // Split by any string using split()
    let north_wall= "Ardhanarishvara (half Shiva and half Parvati), Gangadhara without Parvati, Pashupata-murti, Shiva-alingana-murti, Two dvarapalas";
    println!("Sculptures in the Northern Wall :: ");
    for indv_word in north_wall.split(","){
        println!("{}",indv_word.trim());
    }

    // Iterate by individual chacaters in the string using chars()
    let frescoes = "circumambulatory pathway";
    let mut char_position = 0;
    for indv_char in frescoes.chars(){
        println!("Character at position {} is {}",char_position,indv_char);
        char_position=char_position+1;
    }

    // Concatenation of the Individual String using two methods 
    // 1. Using an append operator "+"
    // 2. using a format Macro "format!"
    let indv_1 = "Commemoration".to_string();
    let indv_2 = "Millennium ".to_string() + &indv_1 ;
    println!("The value of the concatenated String -> using Concat Operator is :: {}",indv_2);
    let concat_str = format!("{} {}",indv_2,indv_1);
    println!("The value of the concatenated String -> using format macro is :: {}",concat_str);

    // Typecasting Numbers in to String using to_string()
    let _first_element = 45;
    let mut first_element_string = _first_element.to_string();
    first_element_string = first_element_string.replace("4","5");
    println!("The value of the first_element_string String is :: {}",first_element_string);


}