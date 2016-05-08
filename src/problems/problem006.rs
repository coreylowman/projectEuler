fn go() -> String {
    let mut sum = 0;
    let mut sumsq = 0;
    for i in 1..101 {
        sum += i;
        sumsq += i * i;
    }
    (sum * sum - sumsq).to_string()
}

problem!(go, 25164150);
