use prime_gen::{is_prime,gen};
use num::traits::PrimInt;
use std::collections::HashMap;

struct Graph {
	data : HashMap<u64,Vec<u64>>
}

impl Graph {

	fn new() -> Graph {
		Graph {
			data : HashMap::new()
		}
	}

	fn has_node(&self,n : u64) -> bool {
		self.data.contains_key(&n)
	}
	
	fn get_nodes(&self) -> Vec<u64> {
		let ks : Vec<&u64> = self.data.keys().collect();
		let mut res : Vec<u64> = Vec::new();
		for &q in ks {
			res.push(q);
		}
		res
	}
	
	fn get_pivot(&self, ns : Vec<u64>) -> u64 {
		let mut max_length = 0;
		let mut max_val = 0;
		for n in ns {
			let l = self.data.get(&n).unwrap().len();
			if l > max_length {
				max_length = l;
				max_val = n;
			}
		}
		max_val
	}
	
	fn try_insert(&mut self,n : u64) {
		if !self.data.contains_key(&n) {
			self.data.insert(n,Vec::new());
		}
	}
	
	fn neighbors_of(&self,n : u64) -> &Vec<u64> {
		self.data.get(&n).unwrap()
	}
	
	fn add_biedge(&mut self, from : u64, to : u64) {
		self.try_insert(from);
		self.try_insert(to);
		
		if let Some(l) = self.data.get_mut(&to) {
			l.push(from);
		}
		if let Some(l) = self.data.get_mut(&from) {
			l.push(to);
		}
	}
	
	fn bron_kerbosch(&self,mut r : &mut Vec<u64>,mut p : &mut Vec<u64>,mut x: &mut Vec<u64>,mut ans : &mut u64){
		if p.len() == 0 && x.len() == 0 && r.len() > 4 {
			*ans = r.iter().fold(0,|acc,&item| acc + item);
			return
		}
		let u = self.get_pivot(union(p,x));
		let mut uns = &Vec::new();
		if self.has_node(u) { uns = self.neighbors_of(u) }
		for v in minus(p,&uns) {
			let mut ns = &Vec::new();
			if self.has_node(v) { ns = self.neighbors_of(v) }
			r.push(v);
			self.bron_kerbosch(&mut r,&mut intersect(p,&ns),&mut intersect(x,&ns),&mut ans);
			if *ans > 0 {
				return
			}
			r.pop();
			let i : usize = p.iter().position(|&x| x == v).unwrap();
			p.remove(i);
			x.push(v);
		}
	}
	
	fn get_max_clique(&self) -> u64 {
		let mut res = 0;
		self.bron_kerbosch(&mut vec![],&mut self.get_nodes(),&mut vec![],&mut res);
		res
	}
}

fn intersect(a : &Vec<u64>, b : &Vec<u64>) -> Vec<u64>{
	a.iter().filter_map(|&x| if b.contains(&x) { Some(x) } else { None }).collect()
}

fn union(a : &Vec<u64>,b : &Vec<u64>) -> Vec<u64> {
	let mut res = minus(a,b);
	for &t in b {
		res.push(t);
	}
	res
}

fn minus(a : &Vec<u64>, b : &Vec<u64>) -> Vec<u64> {
	a.iter().filter_map(|&x| if !b.contains(&x) { Some(x) } else { None }).collect()	
}

fn concat(a : u64,b:u64)->u64 {
	a * 10.pow(b.to_string().len() as u32) + b
}

pub fn go() -> u64 {
	let mut gr = Graph::new();
	let primes = gen(10_000);
	for i in 0..primes.len() {
		for j in i+1..primes.len() {
			let p = primes[i];
			let q = primes[j];
			if is_prime(concat(p,q)) && is_prime(concat(q,p)) {
				gr.add_biedge(p,q);
			}
		}
	}
	gr.get_max_clique()
}