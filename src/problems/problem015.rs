/*
starting at top right you need 2 moves right and 2 moves down...
so the number of permutations of RRDD is the total number of ways
4!/2!2! = 4 * 3 / 2 = 2 * 3=  6


so for a 20x20 board you have
RR...RDD...D... 20 of each
you need 20 moves right and 20 moves down
40!/20!20!
40 choose 20
*/
/*
fn quick_fact(n:usize) -> u64{
	let mut res : Vec<u64> = vec![0;n];
	res[0] = 1;
	for i in 1..n {
		let x : u64 = i as u64 + 1;
		res[i] = x * res[(i-1)];
	}
	res[n-1]
}
*/

pub fn go() -> String {
	//40 *... * 21 / 20 * ... * 1
	//4 * 39.37.11.31.29.5.23.21
	/*
	overflows :|
	println!("{}",quick_fact(40));
	let fact40 = quick_fact(40);
	let fact20 = quick_fact(20);
	fact40 / (fact20 * fact20)*/
	(4 * 39 * 37 * 11 * 31 * 29 * 5 * 23 * 21 as u64).to_string()
}

problem!(go, 137846528820);
