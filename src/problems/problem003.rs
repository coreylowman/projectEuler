
use util::prime_gen;

fn go() -> String {
    let num : u64 = 600851475143;
    let primes = prime_gen::gen(775146);
    for &p in primes.iter().rev() {
        if num % p == 0 {
            return p.to_string()
        }
    }
    0.to_string()
}

problem!(go);
