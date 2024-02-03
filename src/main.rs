mod libs;
use libs::converter::*;
use libs::bit::*;

fn main() {
    // w1w_readbyte();
    base64encode("pri");
    // Ввод
    let mut line = "".to_string();
    std::io::stdin().read_line(&mut line).unwrap();
    // Hex
    let mut hexArray: Vec<u8> = vec![];
    for ch in line.chars() {
        let mut hexDigit = convert10to16(ch as u32);
        if (hexDigit.eq(&('\r' as u8)) || hexDigit.eq(&('\n' as u8))) {
            continue;
        }
        hexArray.push(hexDigit);
    }
    println!("{:?}_h", hexArray);
    // Dec
    let mut decArray: Vec<u32> = vec![];
    for ch in line.chars() {
        if (ch.eq(&'\r') || ch.eq(&'\n')) {
            continue;
        }
        decArray.push(ch as u32);
    }
    println!("{:?}_d", decArray);


    //todo:: записать весь массив байт hexArray и потом попытаться декодировать в base64
    //todo:: записать весь массив байт hexArray и потом попытаться декодировать в url кодировку
    //todo:: написать decode для методов выше
    let mut bits: Vec<u8> = vec![0x2F; 10]; // Создаем список из 10 битов, все установлены в false
    println!("{:?}_d", bits);

    //bits.push();
    //bits[3] = true; // Устанавливаем 4-й бит в true
    // println!("{}", mem::size_of::<bool>());
    // println!("{:?}", bits); // Выводим список из битов
    // for bit in &bits {
    //     println!("{}", bit); // Печатаем каждый бит
    // }
}
