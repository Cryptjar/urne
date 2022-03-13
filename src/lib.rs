#![no_std]
#![feature(generic_associated_types)]
#![feature(unboxed_closures)]


extern crate alloc;

use alloc::vec::Vec;

use rand::Rng;

// TODO: add aggregate functionality

pub mod join;
pub mod list;
pub mod chain;
pub mod map;


/// An Urn Model
///
/// Describes a family of [`Urne`]n.
pub trait UrneModel {
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
impl<T: ?Sized> UrneModel for &'_ T
where
	T: UrneModel,
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
pub trait Urne {
	type Item;
	type MultiItem: IntoIterator<Item = Self::Item>;

	/// Returns a random item
	///
	/// # Returns
	/// `None` if `self` is empty.
	fn choose<R: Rng>(&mut self, rng: R) -> Option<Self::Item> {
		self.choose_multiple(rng, 1)
			.and_then(|iter| iter.into_iter().next())
	}

	/// Returns a list of mutually unique items form this urn
	///
	/// # Returns
	/// `None` if `self` contains less than `amount` items.
	fn choose_multiple<R: Rng>(&mut self, rng: R, amount: usize) -> Option<Self::MultiItem>;
}
impl<T: ?Sized> Urne for &'_ mut T
where
	T: Urne,
{
	type Item = T::Item;
	type MultiItem = T::MultiItem;

	fn choose<R: Rng>(&mut self, rng: R) -> Option<Self::Item> {
		T::choose(self, rng)
	}

	fn choose_multiple<R: Rng>(&mut self, rng: R, amount: usize) -> Option<Self::MultiItem> {
		T::choose_multiple(self, rng, amount)
	}
}

pub trait UrneObj<R: Rng> {
	type Item;

	fn choose(&mut self, rng: &mut R) -> Option<Self::Item>;

	fn choose_multiple(&mut self, rng: &mut R, amount: usize) -> Option<Vec<Self::Item>>;
}
impl<T, R: Rng> UrneObj<R> for T
where
	T: Urne,
{
	type Item = T::Item;

	fn choose(&mut self, rng: &mut R) -> Option<Self::Item> {
		<T as Urne>::choose(self, rng)
	}

	fn choose_multiple(&mut self, rng: &mut R, amount: usize) -> Option<Vec<Self::Item>> {
		<T as Urne>::choose_multiple(self, rng, amount).map(|iter| iter.into_iter().collect())
	}
}
