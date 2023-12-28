use hex::decode;
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Gee my argumente"); // "give me arguments" in afrikaans
    } else if args[1] == "/?" {
        println!("--vlag wys wys vlag"); // "--vlag shows flag" in afrikaans
    } else if args[1] == "--vlag"{
        let hex1 = "000000446974206973206e696520276e20766c6167206e6965";
        let hex2 = "00000000000000000000000000000000000000000000000000";
        
        let bytes1 = decode(hex1).expect("ongeldige heks1");
        let bytes2 = decode(hex2).expect("ongeldige heks2");
      
        let magic: Vec<u8> = bytes1
            .iter()
            .zip(bytes2.iter())
            .map(|(&b1, &b2)| b1 ^ b2)
            .collect();
       
        let text = String::from_utf8(magic).expect("ongeldige UTF-8");
       
        println!("{}", text);
    }else if args[1] == "--bronkode"{
        println!("bronkode kan gevind word op my github, gebruikersnaam kaszovsky");
    
    } else {
            println!("ongeldige argumene: {}", args[1]);
    }
}