use core::fmt;
use core::fmt::Display;

use rand::Rng;
use urne::join::Joiner;
use urne::list::List;
use urne::map::Mapper;
use urne::Urne;
use urne::UrneModel;

/*
// Aggregate as Pool
pub(crate) fn generate(length: usize) -> Vec<Self> {
	let given_names = given_names();
	let surnames = surnames();

	given_names.choose_multiple(&mut thread_rng(), length)
		.zip(given_names.choose_multiple(&mut thread_rng(), length))
		.zip(surnames.choose_multiple(&mut thread_rng(), length))
		.map(|((g,m),l)| {
			let with_middle_name = thread_rng().gen_ratio(
				MIDDLE_NAME_PROB_PER_CENT,
				100
			);
			Name::from_names(
				g.clone(),
				with_middle_name.as_some(m.clone()),
				l.clone()
			)
		})
		.collect()
}
*/

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

fn print_some_items<R: Rng, P: Urne>(mut pool: P, mut rng: R)
where
	P::Item: Display,
{
	println!("{}", pool.choose(&mut rng).unwrap());
	println!("{}", pool.choose(&mut rng).unwrap());
	println!("{}", pool.choose(&mut rng).unwrap());
}

fn print_multiple_items<R: Rng, P: Urne>(mut pool: P, rng: R)
where
	P::Item: Display,
{
	for e in pool.choose_multiple(rng, 3).unwrap() {
		println!("{}", e);
	}
}

fn test<R: Rng>(mut rng: R) {
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

fn main() {
	// Needs `default` feature to run (i.e. `rand/default`)
	#[cfg(feature = "default")]
	{
		use rand::thread_rng;

		test(thread_rng());
	}
}
