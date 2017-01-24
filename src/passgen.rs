use rand::{self, Rng};

const CHARACTERS: &'static [u8] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()_+=";

const CHARACTERS_SIZE: usize = 75;

pub fn generate(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(CHARACTERS[rng.gen::<usize>() % CHARACTERS_SIZE]);
    }
    vec
}