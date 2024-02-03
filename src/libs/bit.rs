use std::io::stdout;
use std::str::Chars;

// todo :: понять как перебирать блоками, то есть смещать на 3 бита и записывать значение в массив
pub fn w1w_readbyte() -> u8
{
    let mut byte: u8 = 0xFF;
    let mut count = 0;
    for i in 0..8 {
        byte >>= 1;
        byte = byte | 0x80;
        count = i;
    }
    return byte;
}

pub fn base64encode(input: &str)
{
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut encoded_string = "";
    let input_length = input.len();
    let input_chars = input;

    for i in (0..input_length).step_by(3)
    {
        let mut char_array_3: [u8; 3] = [0; 3];
        let mut char_array_4: [u8; 4] = [0; 4];
        // В кодировке base64 минимальные текс состоит из 3 символов из 3 символов мы получаем 4 символа из таблицы base64
        for j in (0..3)
        {
            if (i + j < input_length) {
                char_array_3[j] = input_chars.as_bytes()[i + j];
            }
        }
        // сохраняем 6 левых битов в char_array_4[0] и смещаем в право на 2 бита(для чего? Чтобы выровнять по правому краю те шесть битов)
        // получаем из 1111_1100 вот такой бит 0011_1111, тут биты могут быть совмем другие, так заполнил для лучшего поним
        char_array_4[0] = (char_array_3[0] & 0xFC) >> 2;
        // сохраняем 2 правых бита из char_array_3[0] и смещаем на 4 влево
        // получаем 0000_0011 и смещаем на 4 в лево 0011_0000
        // + операция плюс, выполнится последнее
        // потом берем следующие 4 бита из следующего символа
        // 1111_0000 и смещаем на 4 в лево 0000_1111
        // 0011_0000 + 0000_1111 = 0011_1111 в итоге мы получили биты 2 бита(оставшиеся) из прошлого символа и совместили с взятыми битам из следующего символа char_array_3[1]
        char_array_4[1] = ((char_array_3[0] & 0x03) << 4) + ((char_array_3[1] & 0xF0) >> 4);
        // берем оставшиеся 4 правых бита из символа char_array_3[1] и совмещаем с 2 левыми битами следующего символа char_array_3[2]
        // 0011_1100 + 0000_0011 = 0011_1111
        char_array_4[2] = ((char_array_3[1] & 0x0F) << 2) + ((char_array_3[2] & 0xC0) >> 6);
        //  Тут берем последние 6 битов и сразу же записываем
        char_array_4[3] = char_array_3[2] & 0x3F;
        println!();
    }
}

// #include <iostream>
// #include <string>
//
// const std::string base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
//
// std::string base64_encode(const std::string& input) {
// std::string encoded_string;
// size_t input_length = input.length();
// const char* input_data = input.c_str();
//
// for (size_t i = 0; i < input_length; i += 3) {
// unsigned char char_array_3[3] = {0};
// unsigned char char_array_4[4] = {0};
//
// for (size_t j = 0; j < 3; j++) {
// if (i + j < input_length) {
// char_array_3[j] = input_data[i + j];
// }
// }
//
// char_array_4[0] = (char_array_3[0] & 0xFC) >> 2;
// char_array_4[1] = ((char_array_3[0] & 0x03) << 4) + ((char_array_3[1] & 0xF0) >> 4);
// char_array_4[2] = ((char_array_3[1] & 0x0F) << 2) + ((char_array_3[2] & 0xC0) >> 6);
// char_array_4[3] = char_array_3[2] & 0x3F;
//
// for (size_t j = 0; j < 4; j++) {
// if (i + j <= input_length) {
// encoded_string += base64_chars[char_array_4[j]];
// } else {
// encoded_string += '=';
// }
// }
// }
//
// return encoded_string;
// }
//
// int main() {
// std::string input = "Hello, World!";
// std::string encoded_string = base64_encode(input);
//
// std::cout << "Base64 Encoded String: " << encoded_string << std::endl;
//
// return 0;
// }