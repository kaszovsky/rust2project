use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Gee my argumente"); // "give me arguments" in afrikaans
    } else if args[1] == "/?" {
        println!("--vlag wys wys vlag"); // "--vlag shows flag" in afrikaans
    } else if args[1] == "--vlag"{
        let hex1 = "000000446974206973206e696520276e20766c6167206e6965";
        let text: String = String::from_utf8(hex::decode(hex1).unwrap()).unwrap();
        println!("{}", text);
    }else if args[1] == "--bronkode"{
        println!("bronkode kan gevind word op my github");
    
    } else {
            println!("ongeldige argumene: {}", args[1]);
    }
}