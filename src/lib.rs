extern crate libc;

use libc::{c_char, c_void};

#[link(name = "cryptonight")]
extern "C" {
    fn cn_slow_hash(data: *const c_void, length: usize, hash: *const c_char, variant: i32, pre_hashed: i32) -> c_void;
}

pub fn hash(data: &[u8], size: usize) -> Vec<i8> {
    let hash_data = [0i8; 32];
    let mut hash_vec = vec![0; 32];
    let data_ptr: *const c_void = &data as *const _ as *const c_void;
    unsafe {
        cn_slow_hash(data_ptr, size, hash_data.as_ptr(), 1 as i32, 0 as i32);
    }
    for i in 0..hash_data.len() {
        hash_vec.push(hash_data[i])
    }
    //let hash_vec: Vec<u8> = transmute(Slice { data: hash_data, len: 256 });
    hash_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let test_data = "This is a test which as at least 43 bytes ...";
        //let expected_hash_value = []
        let hash_value = hash(test_data.as_bytes(), test_data.len());
        //assert_eq!(hash_value., expected_hash_value);
    }
}