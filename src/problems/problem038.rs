extern crate num;

use self::num::traits::PrimInt;

fn num_concat(x: u64, y: u64) -> u64 {
    let l = num_length(y);
    x * 10.pow(l as u32) + y
}

fn has_all(mut a: u64) -> bool {
    let mut digs = [0; 10];

    while a > 0 {
        digs[(a % 10) as usize] += 1;
        a /= 10;
    }

    if digs[0] > 0 {
        return false;
    }

    for i in 1..digs.len() {
        if digs[i] != 1 {
            return false;
        }
    }
    true
}

fn num_length(mut n: u64) -> u64 {
    let mut l = 0;
    while n > 0 {
        l += 1;
        n /= 10;
    }
    l
}

fn go() -> String {
    // number can't have more than 5 digits... as i.concat(2*i) would have 10 digits
    // whereas the number we are looking for has 9
    let mut i = 10_000;
    loop {
        let mut n = 1;
        let mut np = i;
        let x = i;
        loop {
            n += 1;
            np = num_concat(np, n * x);
            if has_all(np) {
                return np.to_string();
            }
            if num_length(np) > 9 {
                break;
            }
        }
        i -= 1;
    }
}

problem!(go, 932718654);
