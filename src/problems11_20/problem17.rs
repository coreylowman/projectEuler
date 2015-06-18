//http://en.wikipedia.org/wiki/English_numerals
fn num_word_count(n : u64) -> u64{
	let below_twenty = [0,3,3,5,4,4,3,5,5,4,3,6,6,8,8,7,7,9,8,8];
	let tens_digit = [0,0,6,6,5,5,5,7,6,6];
	if n < 20 {
		return below_twenty[n as usize];
	}else if n < 100 {
		return tens_digit[(n/10) as usize] + below_twenty[ (n % 10) as usize ]
	}else if n % 100 == 0 {
		return below_twenty[(n/100) as usize] + 7
	}else{
		return below_twenty[(n/100) as usize] + 7 + 3 + num_word_count(n % 100)
	}
}

pub fn go() -> u64 {
	let mut sum = 0;
	for i in 1..1000 {		
		sum += num_word_count(i);
	}
	sum + 11
}