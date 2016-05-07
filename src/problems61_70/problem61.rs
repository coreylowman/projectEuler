use std::collections::HashMap;

fn t(n:u64) -> u64 { n * (n + 1) / 2 }
fn s(n:u64) -> u64 { n * n }
fn p(n:u64) -> u64 { n * (3 * n - 1) / 2 }
fn hx(n:u64) -> u64 { n * (2 * n - 1) }
fn hp(n:u64) -> u64 { n * (5 * n - 3) / 2 }
fn o(n:u64) -> u64 { n * (3 * n - 2) }

fn can_concat(n:u64,q:u64)->bool{
	n%100 == q/100
}

fn f(mut list : &mut Vec<(u64,usize)>,map : &HashMap<u64,Vec<(u64,usize)>>,mut res : &mut u64) {
	let l = list.len();
	let mut frms = [0,0,0,0,0,0];
	for &(_,i) in list.iter() {
		if frms[i] > 0 { return }
		frms[i] += 1;
	}
	if l == 6 {
		if can_concat(list[l-1].0,list[0].0) {
			*res = list.iter().fold(0,|acc,&(item,_)| acc + item);
		}
		return
	}
	let last = list[l-1].0;
	if !map.contains_key(&last) { return }
	for &p in map.get(&last).unwrap().iter() {
		list.push(p);
		f(&mut list,map,&mut res);
		if *res > 0 {
			return
		}
		list.pop();
	}
}

pub fn go() -> u64{
	let funcs : [fn(u64)->u64;6] = [t,s,p,hx,hp,o];
	let mut vals : [Vec<u64>;6] = [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];
	for i in 0..6{
		let mut j = 1;
		loop {
			let v = funcs[i](j);
			if v > 9999 { break }
			if v >= 1000 { vals[i].push(v); }
			j += 1;		
		}
	}
	
	let mut map : HashMap<u64,Vec<(u64,usize)>> = HashMap::new();
	for i in 0..6 {
		for &n in vals[i].iter() {
			for j in 0..6 {
				if j == i { continue }
				for &q in vals[j].iter() {
					if can_concat(n,q) {
						if !map.contains_key(&n) {
							map.insert(n,Vec::new());
						}
						if let Some(l) = map.get_mut(&n) {
							l.push((q,j));
						}
					}
				}
			}
		}
	}
	
	let mut res = 0u64;
	for i in 0..6 {
		for &n in vals[i].iter() {
			let mut list = vec![(n,i)];
			f(&mut list,&map,&mut res);
			if res > 0 {
				return res
			}
		}
	}
	
	0
}