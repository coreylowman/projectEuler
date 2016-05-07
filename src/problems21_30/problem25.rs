use num::bigint::{BigInt,ToBigInt};

/* 	a b
	c d */
struct Mat { a : BigInt, b : BigInt, c : BigInt, d : BigInt }

impl Mat {
	fn new() -> Mat {
		Mat { 	a : 1.to_bigint().unwrap(),
			b : 1.to_bigint().unwrap(), 
			c : 1.to_bigint().unwrap(), 
			d : 1.to_bigint().unwrap() 
		}
	}
	
	fn fib_mat() -> Mat {
		Mat { a : 1.to_bigint().unwrap(),
			b : 1.to_bigint().unwrap(), 
			c : 1.to_bigint().unwrap(), 
			d : 0.to_bigint().unwrap()}
	}
}

impl Clone for Mat {
	fn clone(&self) -> Mat {
		Mat { 	a : self.a.clone(),
				b : self.b.clone(),
				c : self.c.clone(),
				d : self.d.clone(),
			}
	}
}

fn mat_mul(f : &Mat,g : &Mat) -> Mat {
	Mat { 	a : &f.a * &g.a + &f.b * &g.c,
			b : &f.a * &g.b + &f.b * &g.d,
			c : &f.c * &g.a + &f.d * &g.c,
			d : &f.c * &g.b + &f.d * &g.d,
		}
}

fn mat_exp(f : &Mat,n : u32 ) -> Mat {
	let f2 : &Mat = &mat_mul(f,f);
	if n == 0 { return Mat::new() }
	else if n == 1 { return f.clone() }
	else if n % 2 == 0 { return mat_exp(f2, n / 2) }
	
	mat_mul(f,&mat_exp(f2,(n-1)/2))
}

fn num_digs(n : BigInt) -> u64 {
	n.to_string().len() as u64
}

pub fn go() -> u64 {
	let fib_mat = Mat::fib_mat();
	let mut start = 4096;
	let mut end = 8192;
	let mut mid = (start + end) / 2;
	let mut d = num_digs(mat_exp(&fib_mat,mid).b);
	while d != 1000 {
		mid = (start + end) / 2;
		let f = mat_exp(&fib_mat,mid);
		d = num_digs(f.b);
		if d < 1000 {
			start = mid;
		}else if d > 1000 {
			end = mid;
		}
	}
	while d == 1000 {
		mid = mid - 1;
		let f = mat_exp(&fib_mat,mid);
		d = num_digs(f.b);
		if d == 1000 {
			mid = mid - 1;
			d = num_digs(f.d);
		}
	}
	(mid + 1) as u64
}