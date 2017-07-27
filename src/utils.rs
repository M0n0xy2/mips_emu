pub fn offset_addr(base: u32, offset: i32) -> u32 {
    ((base as i64) + (offset as i64)) as u32
    /*if offset >= 0 {
        base + offset as u32
    } else {
        base - (-offset) as u32
    }*/
}

pub fn u2i(input: u32) -> i32 {
    /*unsafe {
        std::mem::transmute::<u32, i32>(input)
    }*/
    input as i32
}

pub fn i2u(input: i32) -> u32 {
    /* unsafe {
        std::mem::transmute::<i32, u32>(input)
    }*/
    input as u32
}
