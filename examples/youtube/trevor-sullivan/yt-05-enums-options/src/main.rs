use std::u8;

enum CharacterType{
    Archer,
    Warrior,
    Mage,
}

fn test_option_char_type() -> Option<CharacterType>{
    let mut char_type: Option<CharacterType>=None;
    // char_type = Some(CharacterType::Mage);
    return char_type;
}
fn test_option_type() -> Option<u8> {
    let mut opt1: Option<u8> = None;
    opt1 = Some(10);
    return opt1;
}

fn test_option_string() -> Option<String>{
    let mut opt1: Option<String> = None;
    opt1 = Some("Trevor Sullivan".to_string());
    return opt1;
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer",
            CharacterType::Warrior => "Warrior",
            CharacterType::Mage => "Mage",
        }.to_string()
    }
}

fn main() {
    println!("Hello, world!");
    let result: Option<u8> = test_option_type();
    println!("result: {}", result.unwrap());

    let str_result: Option<String> = test_option_string();
    println!("Name is {}",str_result.unwrap());

    let char_result = test_option_char_type();
    if char_result.is_some(){
        println!("User has selected a character type: {}",char_result.unwrap().to_string());
    }else {
        println!("Character type is None");
    }
}
