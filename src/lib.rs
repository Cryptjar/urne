#![no_std]
#![feature(generic_associated_types)]
#![feature(unboxed_closures)]


extern crate alloc;

use alloc::vec::Vec;

use rand::Rng;

// TODO: add aggregate functionality

pub mod join;
pub mod list;
pub mod map;


/// An Urn Model
pub trait Urnenmodell {
	type Item<'a>
	where
		Self: 'a;

	type Peeking<'a>: Urne<Item = Self::Item<'a>>
	where
		Self: 'a;

	type Taking<'a>: Urne<Item = Self::Item<'a>>
	where
		Self: 'a;

	fn peeking(&self) -> Self::Peeking<'_>;
	fn taking(&self) -> Self::Taking<'_>;
}
impl<T> Urnenmodell for &'_ T
where
	T: Urnenmodell,
{
	type Item<'a> = T::Item<'a> where T: 'a, Self: 'a;
	type Peeking<'a> = T::Peeking<'a> where T: 'a, Self: 'a;
	type Taking<'a> = T::Taking<'a> where T: 'a, Self: 'a;

	fn peeking(&self) -> Self::Peeking<'_> {
		T::peeking(self)
	}

	fn taking(&self) -> Self::Taking<'_> {
		T::taking(self)
	}
}

/// An Urn
// Maybe bound Iterator
pub trait Urne {
	// maybe replace with `Iterator::Item`
	type Item;

	// maybe replace with `Iterator::next`
	fn choose<R: Rng>(&mut self, rng: R) -> Self::Item {
		self.choose_multiple(rng, 1).remove(0)
	}

	fn choose_multiple<R: Rng>(&mut self, rng: R, amount: usize) -> Vec<Self::Item>;
}
impl<T> Urne for &'_ mut T
where
	T: Urne,
{
	type Item = T::Item;

	fn choose<R: Rng>(&mut self, rng: R) -> Self::Item {
		T::choose(self, rng)
	}

	fn choose_multiple<R: Rng>(&mut self, rng: R, amount: usize) -> Vec<Self::Item> {
		T::choose_multiple(self, rng, amount)
	}
}
