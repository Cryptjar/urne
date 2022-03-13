use core::fmt::Display;

use either::Either;
use rand::Rng;
use urne::chain::Chain;
use urne::list::List;
use urne::Urne;
use urne::UrneModel;


fn print_multiple_items<R: Rng, P: Urne>(mut pool: P, rng: R)
where
	P::Item: Display,
{
	for e in pool.choose_multiple(rng, 3).unwrap() {
		println!("{}", e);
	}
}

fn selector<'a>(either: Either<&'a &'_ str, &'a String>) -> &'a str {
	match either {
		Either::Left(adj) => *adj,
		Either::Right(name) => name.as_str(),
	}
}

fn test<R: Rng>(mut rng: R) {
	let adj_list = vec!["flat", "nice", "tall", "small", "avr", "cold"];
	let adj_model = List::new(adj_list);

	let name_list = vec!["Foo".to_string(), "Bar".to_string(), "Stuff".to_string()];
	let name_model = List::new(name_list);

	let chained_model = Chain::new(adj_model, name_model, 2, 1, selector);

	println!();
	println!("Peeking:");
	let mut chained = chained_model.peeking();
	print_multiple_items(&mut chained, &mut rng);
	println!();
	print_multiple_items(&mut chained, &mut rng);

	println!();
	println!("Taking:");
	let mut chained = chained_model.taking();
	print_multiple_items(&mut chained, &mut rng);
	println!();
	print_multiple_items(&mut chained, &mut rng);
}

fn main() {
	// Needs `default` feature to run (i.e. `rand/default`)
	#[cfg(feature = "default")]
	{
		use rand::thread_rng;

		test(thread_rng());
	}
}
