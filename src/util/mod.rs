pub fn fill_vec(v: &mut Vec<u8>, size: usize) {
    for _ in 0..size {
        &v.push(0u8);
    }
}
