#[derive(Debug, Copy, Clone)]
enum Suit {
	Spades, Clubs, Hearts, Diamonds
}

#[derive(Debug, Ord, Copy, Clone)]
enum CardVal {
	Ace,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King
}

#[derive(Debug)]
pub struct Card {
	suit: Suit,
	value: CardVal,
	visible: bool
}

pub struct Klondike {
	foundations: [Vec<Card>; 4]
	tableaux: [Vec<Card>; 7]
	waste: Vec<Card>
	waste_view: Vec<Card>
}

impl Klondike {
	pub fn new() {
		// TODO: create card deck, shuffle, init waste and tableaux
		Klondike { foundations: [Vec::new(); 4], tableaux: [Vec::new(); 7], waste: Vec::new(), waste_view: Vec::new() }
	}

	pub fn turnWasteCardOrReset(&mut self) {
		match self.waste.pop() {
			Some(x) => { self.wasteView.push(x); }
			None => {
				swap(&mut self.waste, &mut self.waste_view);
				self.waste.reverse();
			}
		}
	}

	pub fn takeWasteViewCard(&mut self) {
		self.waste_view.pop()
	}
	pub fn takeFoundationCard(&mut self, f) {
		self.foundations[f].pop()
	}
	pub fn takeTableauCards(&mut self, t, n) {
		let len = self.tableaux[t].length();
		if n <= 0 || n > len || !self.tableaux[t][len-n].visible {
			None
		} else {
			let cards = self.tableaux[t].split_off(len-n);
			match self.tableaux[t].last_mut() {
				Some(ref mut x) => { x.visible = true; },
				None => ()
			};
			cards
		}
	}

	pub fn wasteEmpty(&self) {
		self.waste.is_empty()
	}

	pub fn viewWasteViewCard(&self) {
		self.waste_view.last()
	}
	pub fn viewFoundationCard(&self, f) {
		self.foundations[f].last()
	}
	pub fn viewTableau(&self, 

}
