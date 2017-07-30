pub fn offset_addr(base: u32, offset: i32) -> u32 {
    let result = (base as i64) + (offset as i64);
    let result_word = result as u32;
    assert!(result == result_word as i64, "OVERFLOW ERROR");
    result_word
}

pub fn u2i(input: u32) -> i32 {
    input as i32
}

pub fn i2u(input: i32) -> u32 {
    input as u32
}
