// pub fn convert10to16(mut decimal: u32) -> String {
//     let mut result: u32 = 16;
//     let mut remainder: u32 = 0;
//     let mut array: Vec<u32> = vec![];
//     while (result >= 16) {
//         result = decimal / 16;
//         remainder = decimal % 16;
//         decimal = result;
//         array.push(remainder);
//         if (result < 16) {
//             array.push(result);
//         }
//     }
//     let mut str: String = "".to_string();
//     while (array.len() > 0) {
//         result = array.pop().unwrap();
//         if (result > 10)
//         {
//             match result {
//                 10 => str += "A",
//                 11 => str += "B",
//                 12 => str += "C",
//                 13 => str += "D",
//                 14 => str += "E",
//                 15 => str += "F",
//                 _ => println!("Not HEX"),
//             }
//         } else {
//             str += &result.to_string();
//         }
//     }
//     return str;
// }

pub fn convert10to16(mut decimal: u32) -> u8 {
    let mut result: u32 = 16;
    let mut remainder: u32 = 0;
    let mut decArray: Vec<u32> = vec![];
    while (result >= 16) {
        result = decimal / 16;
        remainder = decimal % 16;
        decimal = result;
        decArray.push(remainder);
        if (result < 16) {
            decArray.push(result);
        }
    }
    let mut hexDigit: u16 = 0;
    while (decArray.len() > 0) {
        result = decArray.pop().unwrap();
        if (result > 10)
        {
            match result {
                10 => hexDigit += 0xA,
                11 => hexDigit += 0xA,
                12 => hexDigit += 0xA,
                13 => hexDigit += 0xA,
                14 => hexDigit += 0xA,
                15 => hexDigit += 0xA,
                _ => println!("Not HEX"),
            }
        } else {
            hexDigit += result as u16;
        }
        hexDigit *= 10;
    }
    hexDigit /= 10;
    return hexDigit as u8;
}