
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug,PartialEq,Eq,Clone)]
enum Suit {
	Hearts,
	Diamond,
	Spades,
	Clubs,
}
 
#[derive(Debug)]
struct Card {
	val : u8,
	suit : Suit,
}

#[derive(Debug)]
enum HandType {
	HighCard(u8),
	OnePair(u8), //num1
	TwoPairs(u8,u8), //num1 num2
	ThreeOfAKind(u8), //val
	Straight(u8), //start number
	Flush(Suit), //suit
	FullHouse(u8,u8), //num1 num2
	FourOfAKind(u8), //num
	StraightFlush(u8,Suit), //start suit
	RoyalFlush(Suit),
}
 
#[derive(Debug)]
struct Hand{
	cards : [Card;5],
	hand_type : HandType,
	val : u8,
}

impl Hand {
	fn new_empty() -> Hand {
		Hand {
			cards : [ Card { val : 0, suit : Suit::Clubs } ,
						Card { val : 0, suit : Suit::Clubs },
						Card { val : 0, suit : Suit::Clubs },
						Card { val : 0, suit : Suit::Clubs },
						Card { val : 0, suit : Suit::Clubs }],
			hand_type : HandType::HighCard(0),
			val : 0,
		}	
	}
	
	fn new(init : Vec<&str>) -> Hand {
		let mut i = 0;
		let mut hand = Hand::new_empty();
		for s in init {
			let chs : Vec<char> = s.chars().collect();
			hand.cards[i].val = get_val(chs[0]);
			hand.cards[i].suit = get_suit(chs[1]);
			i+=1;
		}
		hand.cards.sort_by(|a,b| a.val.cmp(&b.val));
		hand.assign_type();
		hand
	}
	
	fn assign_type(&mut self) {
		if self.is_a_run() && self.cards_in_same_suit() {
			if self.cards[0].val == 10 {
				//royal flush
				self.hand_type = HandType::RoyalFlush(self.cards[0].suit.clone());
				self.val = 9;
			}else {
				//straight flush
				self.hand_type = HandType::StraightFlush(self.cards[0].val,self.cards[0].suit.clone());
				self.val = 8;
			}
			return
		}
		let q = self.is_four_of_a_kind();
		if q > 0 {
			self.hand_type = HandType::FourOfAKind(q);
			self.val = 7;
			return
		}
		let (q,e) = self.is_full_house();
		if q > 0 {
			self.hand_type = HandType::FullHouse(q,e);
			self.val = 6;
			return
		}
		
		if self.cards_in_same_suit() {
			self.hand_type = HandType::Flush(self.cards[0].suit.clone());
			self.val = 5;
			return
		}
		
		if self.is_a_run() {
			self.hand_type = HandType::Straight(self.cards[0].val);
			self.val = 4;
			return
		}
		
		let q = self.is_three_of_a_kind();
		if q > 0 {
			self.hand_type = HandType::ThreeOfAKind(q);
			self.val = 3;
			return
		}
		
		let (q,e) = self.is_two_pair();
		if q > 0 {
			self.hand_type = HandType::TwoPairs(q,e);
			self.val = 2;
			return
		}
		
		let q = self.is_pair();
		if q > 0 {
			self.hand_type = HandType::OnePair(q);
			self.val = 1;
			return
		}
		
		self.hand_type = HandType::HighCard(self.cards[4].val);
	}
	
	fn is_better(&self,other : &Hand) -> bool {
		if self.val == other.val {
			if self.get_rank() == other.get_rank() {
				let mut i = 4;
				while i >= 1 {
					if self.cards[i].val == other.cards[i].val {
						i -=1;
						continue;
					}
					return self.cards[i].val > other.cards[i].val
				}
				return self.cards[0].val > self.cards[0].val
			}
			return self.get_rank() > other.get_rank()
		}
		
		self.val > other.val
	}
	
	fn get_rank(&self) -> u8 {
		match self.hand_type {
			HandType::HighCard(q) => q,			
			HandType::OnePair(q) => q,
			HandType::TwoPairs(q,_) => q,
			HandType::ThreeOfAKind(q) => q,
			HandType::Straight(q) => q,
			HandType::Flush(_) => 1,
			HandType::FullHouse(q,_) => q,
			HandType::FourOfAKind(q) => q,
			HandType::StraightFlush(q,_) => q,
			HandType::RoyalFlush(_) => 1,
		}
	}
	
	fn cards_in_same_suit(&self) -> bool {
		self.cards[0].suit == self.cards[1].suit
		&& self.cards[0].suit == self.cards[2].suit
		&& self.cards[0].suit == self.cards[3].suit
		&& self.cards[0].suit == self.cards[4].suit
	}
	
	fn is_a_run(&self) -> bool {
		let v = self.cards[0].val;
		self.cards[1].val == v + 1
		&& self.cards[2].val == v + 2
		&& self.cards[3].val == v + 3
		&& self.cards[4].val == v + 4	
	}
	
	fn is_pair(&self) -> u8 {
		if self.cards[0].val == self.cards[1].val {
			return self.cards[0].val
		}else if self.cards[1].val == self.cards[2].val {
			return self.cards[1].val
		}else if self.cards[2].val == self.cards[3].val {
			return self.cards[2].val
		}else if self.cards[3].val == self.cards[4].val {
			return self.cards[3].val
		}
		0
	}
	
	fn is_two_pair(&self) -> (u8,u8) {
		let q = self.cards[0].val == self.cards[1].val;
		let w = self.cards[1].val == self.cards[2].val;
		let e = self.cards[2].val == self.cards[3].val;
		let r = self.cards[3].val == self.cards[4].val;
		
		if q && e {
			return (self.cards[0].val,self.cards[2].val)
		}else if q && r {
			return (self.cards[0].val,self.cards[3].val)
		}else if w && r {
			return (self.cards[1].val,self.cards[3].val)
		}
		(0,0)
	}
	
	fn is_three_of_a_kind(&self) -> u8 {
		if self.cards[0].val == self.cards[1].val
		&& self.cards[0].val == self.cards[2].val {
			return self.cards[0].val
		}else if self.cards[1].val == self.cards[2].val
		&& self.cards[1].val == self.cards[3].val {
			return self.cards[1].val
		}else if self.cards[2].val == self.cards[3].val
		&& self.cards[2].val == self.cards[4].val {
			return self.cards[2].val
		}
		0
	}
	
	//since the cards are sorted... the four cards will either be
	//less than the 5th or greater than the 5th
	fn is_four_of_a_kind(&self) -> u8 {
		let v = self.cards[0].val;
		let q = self.cards[1].val;
		if self.cards[1].val == v
		&& self.cards[2].val == v
		&& self.cards[3].val == v {
			return v
		}else if self.cards[2].val == q
		&& self.cards[3].val == q
		&& self.cards[4].val == q {
			return q
		}
		0
	}
	
	//since the cards are sorted... the 3 cards will either be
	//less than the pair or greater than the pair
	fn is_full_house(&self) -> (u8,u8) {		
		if self.cards[0].val == self.cards[1].val
		&& self.cards[2].val == self.cards[3].val
		&& self.cards[2].val == self.cards[4].val {
			return (self.cards[0].val,self.cards[2].val)
		}else if self.cards[0].val == self.cards[1].val
		&& self.cards[0].val == self.cards[2].val
		&& self.cards[3].val == self.cards[4].val {
			return (self.cards[3].val,self.cards[0].val)
		}
		(0,0)
	}
}

fn get_val(c : char) -> u8 {
	match c {
		'2' => 2,
		'3' => 3,
		'4' => 4,
		'5' => 5,
		'6' => 6,
		'7' => 7,
		'8' => 8,
		'9' => 9,
		'T' => 10,
		'J' => 11,
		'Q' => 12,
		'K' => 13,
		'A' => 14,	
		_ => {println!("no val"); 0}
	}
}

fn get_suit(c : char) -> Suit {
	match c {
		'D' => Suit::Diamond,
		'S' => Suit::Spades,
		'C' => Suit::Clubs,
		'H' => Suit::Hearts,
		_ => { println!("no suit"); Suit::Hearts }
	}
}

pub fn go() -> String {
	let mut f = File::open("data/p054_poker.txt").ok().expect("file open fail");
	let mut s = String::new();
	f.read_to_string(&mut s).ok().expect("file read fail");
	let mut num = 0;
	for l in s.lines() {
		let cards = l.split(' ');
		assert!(cards.clone().count() == 10);
		let first : Vec<&str> = cards.clone().take(5).collect();
		let second : Vec<&str> = cards.skip(5).take(5).collect();
		let first : Hand = Hand::new(first);
		let second : Hand = Hand::new(second);
		if first.is_better(&second) {
			num += 1;
		}
	}
	num.to_string()
}

problem!(go);
