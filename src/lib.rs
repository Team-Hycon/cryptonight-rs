extern crate libc;

use libc::{c_void};

#[link(name = "cryptonight")]
extern "C" {
    fn cn_slow_hash() -> c_void;
}

unsafe fn hash() {
    cn_slow_hash();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hash() {
        
    }
}