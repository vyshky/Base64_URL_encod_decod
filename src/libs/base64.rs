pub fn base64encode(input: &str) -> String
{
    let base64_chars_table = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut encoded_string: String = "".to_owned();
    let input_length = input.len();
    let input_chars = input.as_bytes();


    for i in (0..input_length).step_by(3)
    {
        let mut char_array_3: [u8; 3] = [0; 3];
        let mut base64_chars_array: [u8; 4] = [0; 4];
        // В кодировке base64 минимальные текс состоит из 3 символов ASCI. Из 3 символов(ASCI) мы получаем 4 символа из таблицы base64
        for j in (0..3)
        {
            if (i + j < input_length) {
                char_array_3[j] = input_chars[i + j];
            }
        }
        // (char_array_3[0] & 0xFC) >> 2 - берем 6 левых битов и смещаем вправо на 2 бита
        // получаем из 1111_1100 вот такой бит 0011_1111. Получаем 6 битов
        base64_chars_array[0] = (char_array_3[0] & 0xFC) >> 2;
        // char_array_3[0] & 0x03) << 4 - берем 2 бита и смещаем влево на 4 бита
        // из этого 0000_0011 получаем 0011_0000
        // (char_array_3[1] & 0xF0) >> 4 - берем 4 левых битов и смещаем вправо на 4
        // из этого 1111_0000 получаем 0000_1111
        // конкатенируем
        // 0011_0000 + 0000_1111 = 0011_1111 .Получаем 6 битов
        base64_chars_array[1] = ((char_array_3[0] & 0x03) << 4) + ((char_array_3[1] & 0xF0) >> 4);
        // (char_array_3[1] & 0x0F) << 2 - берем 4 правых бита и смещаем влево на 2 бита
        // 0000_1111 << 2 = 0011_1100
        // char_array_3[2] & 0xC0) >> 6 - берем 2 левых бита и смещаем их вправо на 6 бит
        // 1100_0000 >> 6 = 0000_0011
        // конкатенируем
        // 0011_1100 + 0000_0011 = 0011_1111 .Получаем 6 бит
        base64_chars_array[2] = ((char_array_3[1] & 0x0F) << 2) + ((char_array_3[2] & 0xC0) >> 6);
        // Тут берем последние 6 битов и сразу же записываем
        base64_chars_array[3] = char_array_3[2] & 0x3F;

        // Запись в encoded переменную по 4 символа(из таблицы символов BASE64)
        // Если символ в base64_chars == пустоте, то вместо пустоты записываем '='
        for j in 0..4 {
            if (i + j <= input_length) {
                encoded_string.push(char::from(base64_chars_table.as_bytes()[base64_chars_array[j] as usize]));
            } else {
                encoded_string.push('=');
            }
        }
    }
    return encoded_string;
}

pub fn base64decode(input: &str) -> String
{
    // TODO::  :')
    return "sss".to_owned();
}