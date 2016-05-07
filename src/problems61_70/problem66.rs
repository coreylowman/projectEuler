//assumes it doesn't contain it
fn sorted_insert(x : u64, v : &mut Vec<u64>){
	if v.len() == 0 { v.push(x); return }
	if x < v[0] { v.insert(0,x); return }
	if v[v.len() - 1] < x { v.push(x); return }
	let mut l = 0;
	let mut h = v.len() - 1;
	while l <= h {
		let m = (h + l) / 2;
		if v[m-1] < x && x < v[m] {
			v.insert(m,x);
			return
		}else if v[m] < x && x < v[m+1] {
			v.insert(m+1,x);
			return
		}else if v[m] < x {
			l = m + 1;
		}else if x < v[m] {
			h = m - 1;
		}
	}
}

fn is_square(n : u64) -> bool {
	let mut seen = Vec::new();
	let mut x : u64 = n / 2;
	while x * x != n {
		x = (x + (n / x)) / 2;
		if seen.binary_search(&x).is_ok() {
			return false
		}
		sorted_insert(x.clone(),&mut seen);
	}
	true
}

pub fn go() -> u64 {	
	let mut max_d = 2u64;
	let mut max_x = 3u64;
	for d in 3..1001 {
		println!("{}",d);
		let q = (d as f64).sqrt();
		if q.floor() == q {
			continue;
		}
		let d = d as u64;
		let mut x = 2u64;
		loop {
			println!("{}\t{}",d,x);
			//x^2 - d * y^2 = 1
			//x^2 = 1 + d * y^2
			//y^2 = (x^2 - 1) / d
			//d = (x^2 - 1) / y^2
			//d = (x/y)^1 - 1/(y^2)
			if (x * x - 1) % d != 0 {
				x += 1;
				continue;
			}
			let y2 : u64 = (x * x - 1) / d;
			if y2 > 1 && !is_square(y2) {
				x += 1;
				continue;
			}
			assert!(x*x - d * y2 == 1u64);
			if x > max_x {
				max_d = d;
				max_x = x;
			}
			break;
		}
	}
	println!("{} {}",max_d,max_x);
	max_d
}