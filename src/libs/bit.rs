

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


// unsigned char w1w_readbyte(void)
// {
// unsigned char Byte=0x00;
// char count = 0;
//
// for(count = 0; count < 8; count ++)
// {
// Byte >>= 1;
//
// if(w1w_readbit())
// {
// Byte |= 0x80;
// }
// }
// return Byte;
// }