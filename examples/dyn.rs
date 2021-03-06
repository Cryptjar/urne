use core::fmt;
use core::fmt::Display;

use rand::Rng;
use urne::join::Joiner;
use urne::list::List;
use urne::map::Mapper;
use urne::UrneModel;
use urne::UrneObj;



#[derive(Debug)]
struct Person<'a> {
	adj: &'a str,
	name: &'a str,
}
impl<'a> Person<'a> {
	pub fn new<'n>(adj: &'n str, name: &'n str) -> Person<'n> {
		Person {
			adj,
			name,
		}
	}
}
impl fmt::Display for Person<'_> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "the {} {}", self.adj, self.name)
	}
}

fn copy_str<'a, 'b>(s: &'a &'b str) -> &'a str {
	*s
}

fn print_multiple_items<R: Rng, I: Display>(pool: &mut dyn UrneObj<R, Item = I>, mut rng: R) {
	for e in pool.choose_multiple(&mut rng, 3).unwrap() {
		println!("{}", e);
	}
}

fn main() {
	let mut rng = rand::thread_rng();

	let adj_list = vec!["flat", "nice", "tall", "small", "avr", "cold"];
	let adj_model = List::new(adj_list);

	let name_list = vec!["Foo", "Bar", "It", "Stuff", "Thing", "Tree"];
	let name_model = List::new(name_list);

	let adj_model = Mapper::new(adj_model, copy_str);
	let name_model = Mapper::new(name_model, copy_str);

	let person_model = Joiner::new(adj_model, name_model, Person::new);


	println!();
	println!("Peeking:");
	let mut persons = person_model.peeking();
	print_multiple_items(&mut persons, &mut rng);
	println!();
	print_multiple_items(&mut persons, &mut rng);

	println!();
	println!("Taking:");
	let mut persons = person_model.taking();
	print_multiple_items(&mut persons, &mut rng);
	println!();
	print_multiple_items(&mut persons, &mut rng);
}
