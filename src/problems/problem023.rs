
fn ds(n:u64) -> u64 {
	let mut sum = 1;
	let lim = (n as f64).sqrt() as u64 + 1;
	for i in 2..lim {
		if n % i == 0 {
			sum += i;
			if i != n / i {
				sum += n / i;
			}
		}
	}
	sum
}

pub fn go() -> String {
	let mut abunds : Vec<bool> = Vec::new();	
	abunds.push(false); //0
	abunds.push(false); //1
	for i in 2..28123 {
		abunds.push(ds(i) > i); //i is abundant
	}
	let mut sum : u64 = 0;
	for x in 1..28123 {
		let mut canbe = false;
		for i in 1..x/2+1 {
			if abunds[i] && abunds[x - i] {
				canbe = true;
				break;
			}
		}
		if !canbe {
			sum = sum + (x as u64);
		}
	}
	sum.to_string()
}

problem!(go);
