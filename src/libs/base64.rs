pub fn base64encode(input: &str) -> String
{
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut encoded_string: String = "".to_owned();
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

        // Запись в encoded переменную по 4 символа(басе64 может записать только 4 символа по 6 бит, остальные записываются как =, = - значит пустота)
        for j in 0..4 {
            if (i + j <= input_length) {
                encoded_string.push(char::from(base64_chars.as_bytes()[char_array_4[j] as usize]));
            } else {
                encoded_string.push('=');
            }
        }
    }
    return encoded_string;
}