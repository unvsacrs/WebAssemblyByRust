use wasm_bindgen::prelude::*;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn next_prime(mut n: u64) -> u64 {
    loop {
        n += 1;
        if is_prime(n) {
            return n;
        }
    }
}

#[wasm_bindgen]
pub fn get_prime(begin: u64) -> u64 {
    next_prime(begin)
}
