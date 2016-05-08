//t = t1 * 10 + t0 and same for b
fn can(t:u32,b:u32) -> bool {
	let t0 = t % 10;
	let t1 = (t - t0)/10;
	let b0 = b % 10;
	let b1 = (b - b0)/10;	
	
	if (t0 == 0 && b0 == 0) || t == b {
		return false
	}
	
	let f : f32 = (t as f32)/(b as f32);
	let f00 : f32 = (t0 as f32) / (b0 as f32);
	let f01 : f32 = (t0 as f32) / (b1 as f32);
	let f10 : f32 = (t1 as f32) / (b0 as f32);
	let f11 : f32 = (t1 as f32) / (b1 as f32);
	
	(t0 == b0 && f11 == f)
	|| (t0 == b1 && f10 == f)
	|| (t1 == b0 && f01 == f)
	|| (t1 == b1 && f00 == f)
}

fn gcd(mut a:u32,mut b:u32)-> u32 {
	while b != 0 {
		let t = b;
		b = a % b;
		a = t;
	}
	a
}

//1 1 2 1
//4 5 5 2
fn go() -> String {
	let mut top = 1;
	let mut bot = 1;
	for t in 10..100 {
		for b in t..100 {
			if can(t,b) {
				bot *= b / gcd(t,b);
				top *= t / gcd(t,b);
			}
		}
	}
	bot /= gcd(top,bot);
	bot.to_string()
}

problem!(go, 100);
