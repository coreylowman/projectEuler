fn go() -> String {
    let mut sum: u64 = 0;
    for x in 1..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }

    sum.to_string()
}

problem!(go, 233168);
