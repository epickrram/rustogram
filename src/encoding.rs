pub fn put_i32(input_value: i32, buffer: &mut Vec<u8>) {
	buffer.push((input_value >> 24) as u8);
	buffer.push((input_value >> 16) as u8);
	buffer.push((input_value >> 8) as u8);
	buffer.push((input_value) as u8);
}

pub fn get_i32(buffer: &Vec<u8>, offset: i32) -> i32 {
	let offset = offset as usize;
	let i0 = (buffer[offset] as i32) << 24;
	let i1 = (buffer[offset + 1] as i32) << 16;
	let i2 = (buffer[offset + 2] as i32) << 8;
	let i3 = (buffer[offset + 3] as i32);
	
	i0 | i1 | i2 | i3
}

pub fn encode(input_value: i64, buffer: &mut Vec<u8>) {
//    let value = ((input_value << 1) ^ (input_value >> 63)) as u64;
    let value = input_value as u64;
    if value >> 7 == 0 {
        buffer.push(value as u8);
    } else {
        buffer.push(((value & 0x7F) | 0x80) as u8);
        if value >> 14 == 0 {
            buffer.push((value >> 7) as u8);
        } else {
            buffer.push((value >> 7 | 0x80) as u8);
            if value >> 21 == 0 {
                buffer.push((value >> 14) as u8);
            } else {
                buffer.push((value >> 14 | 0x80) as u8);
                if value >> 28 == 0 {
                    buffer.push((value >> 21) as u8);
                } else {
                    buffer.push((value >> 21 | 0x80) as u8);
                    if value >> 35 == 0 {
                        buffer.push((value >> 28) as u8);
                    } else {
                        buffer.push((value >> 28 | 0x80) as u8);
                        if value >> 42 == 0 {
                            buffer.push((value >> 35) as u8);
                        } else {
                            buffer.push((value >> 35 | 0x80) as u8);
                            if value >> 49 == 0 {
                                buffer.push((value >> 42) as u8);
                            } else {
                                buffer.push((value >> 42 | 0x80) as u8);
                                if value >> 56 == 0 {
                                    buffer.push((value >> 49) as u8);
                                } else {
                                    buffer.push((value >> 49 | 0x80) as u8);
                                    buffer.push((value >> 56) as u8);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn decode(buffer: &Vec<u8>, input_offset: i32) -> (i64, i32) {
    let offset = input_offset as usize;
    let mut v: u64 = buffer[offset] as u64;
    let mut value: u64 = (v & 0x7F) as u64;
    let mut consumed_bytes: i32 = 1;
    if (v & 0x80) != 0 {
        v = buffer[offset + 1] as u64;
        value |= ((v & 0x7F) << 7) as u64;
        consumed_bytes = 2;
        if (v & 0x80) != 0 {
            v = buffer[offset + 2] as u64;
            value |= ((v & 0x7F) << 14) as u64;
            consumed_bytes = 3;
            if (v & 0x80) != 0 {
                v = buffer[offset + 3] as u64;
                value |= ((v & 0x7F) << 21) as u64;
                consumed_bytes = 4;
                if (v & 0x80) != 0 {
                    v = buffer[offset + 4] as u64;
                    value |= ((v & 0x7F) << 28) as u64;
                    consumed_bytes = 5;
                    if (v & 0x80) != 0 {
                        v = buffer[offset + 5] as u64;
                        value |= ((v & 0x7F) << 35) as u64;
                        consumed_bytes = 6;
                        if (v & 0x80) != 0 {
                            v = buffer[offset + 6] as u64;
                            value |= ((v & 0x7F) << 42) as u64;
                            consumed_bytes = 7;
                            if (v & 0x80) != 0 {
                                v = buffer[offset + 7] as u64;
                                value |= ((v & 0x7F) << 49) as u64;
                                consumed_bytes = 8;
                                if (v & 0x80) != 0 {
                                    v = buffer[offset + 8] as u64;
                                    value |= (v << 56) as u64;
                                    consumed_bytes = 9;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // value = (value >> 1) ^ (-(value & 1));
    (value as i64, consumed_bytes)
}
