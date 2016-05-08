extern crate num;

use self::num::traits::PrimInt;

fn factorial_arr(n: usize) -> Vec<u64> {
    let mut res: Vec<u64> = vec![0;n];
    res[0] = 1;
    for i in 1..n {
        let x: u64 = i as u64 + 1;
        res[i] = x * res[(i - 1)];
    }
    res
}

fn go() -> String {
    let facts: Vec<u64> = factorial_arr(9);
    let mut coeff = [0; 10];
    let mut num: u64 = 0;
    for i in 0..9 {
        let mut c = 0;
        while num + c * facts[8 - i] < 1_000_000 {
            c = c + 1;
        }
        coeff[i] = c - 1;
        num = num + coeff[i] * facts[9 - i - 1];
    }
    let mut choices: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut sum = 0;
    let mut i = 0;
    while !choices.is_empty() {
        coeff[i] = choices.remove(coeff[i] as usize) as u64;
        i += 1;
    }
    i = 0;
    for x in coeff.iter() {
        sum = sum + x * 10.pow((9 - i) as u32);
        i += 1;
    }
    sum.to_string()
}

problem!(go, 2783915460);
