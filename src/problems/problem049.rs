use util::prime_gen;

fn is_perm(mut n1: u64, mut n2: u64) -> bool {
    let mut d1 = [0; 10];
    let mut d2 = [0; 10];
    while n1 > 0 {
        d1[(n1 % 10) as usize] += 1;
        n1 /= 10;
    }
    while n2 > 0 {
        d2[(n2 % 10) as usize] += 1;
        n2 /= 10;
    }
    for i in 0..10 {
        if d1[i] != d2[i] {
            return false;
        }
    }
    true
}

fn go() -> String {
    let primes = prime_gen::gen(10_000);
    for &p in primes.iter() {
        let p1 = p + 3330;
        let p2 = p + 6660;
        if p != 1487 && primes.binary_search(&p1).is_ok() &&
           primes.binary_search(&p2).is_ok() && is_perm(p, p1) && is_perm(p, p2) {
            return (10_000 * (10_000 * p + p1) + p2).to_string();
        }
    }
    0.to_string()
}

problem!(go, 296962999629);
