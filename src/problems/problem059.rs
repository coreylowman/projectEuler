use std::fs::File;
use std::io::prelude::*;

fn decrypt(key: [u8; 3], mut cipher: Vec<u8>) -> (Vec<u8>, String) {
    for i in 0..cipher.len() {
        cipher[i] = key[i % 3] ^ cipher[i];
    }
    (cipher.clone(), String::from_utf8(cipher).unwrap())
}

fn valid(ptxt: String) -> bool {
    if !ptxt.contains("the") || !ptxt.contains("as") || !ptxt.contains(" ") {
        return false;
    }
    if ptxt.contains("{") || ptxt.contains("}") {
        return false;
    }
    if ptxt.contains("&") {
        return false;
    }
    true
}

pub fn go() -> String {
    let mut f = File::open("data/p059_cipher.txt").ok().expect("file open fail");
    let mut s = String::new();
    f.read_to_string(&mut s).ok().expect("file read fail");
    let nums: Vec<&str> = s.split(',').collect();
    let nums: Vec<u8> = nums.iter().map(|s| s.parse::<u8>().unwrap()).collect();
    for i in 97..123 {
        for j in 97..123 {
            for k in 97..123 {
                let key: [u8; 3] = [i, j, k];
                let (q, t) = decrypt(key, nums.clone());
                if valid(t.clone()) {
                    let mut sum = 0u64;
                    for n in q {
                        sum = sum + n as u64;
                    }
                    return sum.to_string();
                }
            }
        }
    }
    0.to_string()
}

problem!(go, 107359);
