pub fn put_i32(input_value: i32, buffer: &mut Vec<u8>) {
	buffer.push((input_value >> 24) as u8);
	buffer.push((input_value >> 16) as u8);
	buffer.push((input_value >> 8) as u8);
	buffer.push((input_value) as u8);
}

pub fn put_i32_at_offset(input_value: i32, buffer: &mut Vec<u8>, offset: i32) {
	let offset = offset as usize;
	buffer[offset] = (input_value >> 24) as u8;
	buffer[offset + 1] = (input_value >> 16) as u8;
	buffer[offset + 2] = (input_value >> 8) as u8;
	buffer[offset + 3] = input_value as u8;
}

pub fn get_i32(buffer: &Vec<u8>, offset: i32) -> i32 {
	let offset = offset as usize;
	let i0 = (buffer[offset] as i32) << 24;
	let i1 = (buffer[offset + 1] as i32) << 16;
	let i2 = (buffer[offset + 2] as i32) << 8;
	let i3 = (buffer[offset + 3] as i32);
	
	i0 | i1 | i2 | i3
}

pub fn put_i64(input_value: i64, buffer: &mut Vec<u8>) {
	buffer.push((input_value >> 56) as u8);
	buffer.push((input_value >> 48) as u8);
	buffer.push((input_value >> 40) as u8);
	buffer.push((input_value >> 32) as u8);
	buffer.push((input_value >> 24) as u8);
	buffer.push((input_value >> 16) as u8);
	buffer.push((input_value >> 8) as u8);
	buffer.push((input_value) as u8);
}

pub fn get_i64(buffer: &Vec<u8>, offset: i32) -> i64 {
	let offset = offset as usize;
	let i0 = (buffer[offset] as i64) << 56;
	let i1 = (buffer[offset + 1] as i64) << 48;
	let i2 = (buffer[offset + 2] as i64) << 40;
	let i3 = (buffer[offset + 3] as i64) << 32;
	let i4 = (buffer[offset + 4] as i64) << 24;
	let i5 = (buffer[offset + 5] as i64) << 16;
	let i6 = (buffer[offset + 6] as i64) << 8;
	let i7 = (buffer[offset + 7] as i64);
	
	(i0 | i1 | i2 | i3 | i4 | i5 | i6 | i7) as i64
}

fn zero_test(input_value: i64, shift: isize) -> bool {
	let result = sign_preserving_shift(input_value, shift) == 0;
	result
}

fn sign_preserving_shift(input_value: i64, shift: isize) -> i64 {
	((input_value as u64) >> shift) as i64
}

pub fn encode(input_value: i64, buffer: &mut Vec<u8>) {
    let value = ((input_value << 1) ^ (input_value >> 63));
    if zero_test(value, 7) {
        buffer.push(value as u8);
    } else {
        buffer.push(((value & 0x7F) | 0x80) as u8);
        if zero_test(value, 14) {
            buffer.push((value >> 7) as u8);
        } else {
            buffer.push((value >> 7 | 0x80) as u8);
            if zero_test(value, 21) {
                buffer.push((value >> 14) as u8);
            } else {
                buffer.push((value >> 14 | 0x80) as u8);
                if zero_test(value, 28) {
                    buffer.push((value >> 21) as u8);
                } else {
                    buffer.push((value >> 21 | 0x80) as u8);
                    if zero_test(value, 35) {
                        buffer.push((value >> 28) as u8);
                    } else {
                        buffer.push((value >> 28 | 0x80) as u8);
                        if zero_test(value, 42) {
                            buffer.push((value >> 35) as u8);
                        } else {
                            buffer.push((value >> 35 | 0x80) as u8);
                            if zero_test(value, 49) {
                                buffer.push((value >> 42) as u8);
                            } else {
                                buffer.push((value >> 42 | 0x80) as u8);
                                if zero_test(value, 56) {
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
    let mut v: i64 = buffer[offset] as i64;
    let mut value: i64 = (v & 0x7F) as i64;
    let mut consumed_bytes: i32 = 1;
    if (v & 0x80) != 0 {
        v = buffer[offset + 1] as i64;
        value |= ((v & 0x7F) << 7) as i64;
        consumed_bytes = 2;
        if (v & 0x80) != 0 {
            v = buffer[offset + 2] as i64;
            value |= ((v & 0x7F) << 14) as i64;
            consumed_bytes = 3;
            if (v & 0x80) != 0 {
                v = buffer[offset + 3] as i64;
                value |= ((v & 0x7F) << 21) as i64;
                consumed_bytes = 4;
                if (v & 0x80) != 0 {
                    v = buffer[offset + 4] as i64;
                    value |= ((v & 0x7F) << 28) as i64;
                    consumed_bytes = 5;
                    if (v & 0x80) != 0 {
                        v = buffer[offset + 5] as i64;
                        value |= ((v & 0x7F) << 35) as i64;
                        consumed_bytes = 6;
                        if (v & 0x80) != 0 {
                            v = buffer[offset + 6] as i64;
                            value |= ((v & 0x7F) << 42) as i64;
                            consumed_bytes = 7;
                            if (v & 0x80) != 0 {
                                v = buffer[offset + 7] as i64;
                                value |= ((v & 0x7F) << 49) as i64;
                                consumed_bytes = 8;
                                if (v & 0x80) != 0 {
                                    v = buffer[offset + 8] as i64;
                                    value |= (v << 56) as i64;
                                    consumed_bytes = 9;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    value = sign_preserving_shift(value, 1) ^ (-(value & 1));
    (value, consumed_bytes)
}

