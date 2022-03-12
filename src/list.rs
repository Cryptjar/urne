use alloc::vec::Vec;
use core::marker::PhantomData;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::Urne;
use crate::Urnenmodell;

pub struct List<L, T> {
	pub list: L,
	_t: PhantomData<T>,
}
impl<L, T> List<L, T>
where
	L: AsRef<[T]>,
{
	pub fn new(list: L) -> Self {
		List {
			list,
			_t: PhantomData,
		}
	}
}
impl<L, T> Urnenmodell for List<L, T>
where
	L: AsRef<[T]>,
{
	type Item<'a> = &'a T where T: 'a, L: 'a;
	type Peeking<'a> = PeekingList<'a, T> where T: 'a, L: 'a;
	type Taking<'a> = TakingList<&'a T> where T: 'a, L: 'a;

	fn peeking(&self) -> Self::Peeking<'_> {
		PeekingList {
			list: self.list.as_ref(),
		}
	}

	fn taking(&self) -> Self::Taking<'_> {
		TakingList {
			list: self.list.as_ref().iter().collect(),
		}
	}
}

pub struct PeekingList<'a, T> {
	list: &'a [T],
}
impl<'a, T> Urne for PeekingList<'a, T> {
	type Item = &'a T;

	fn choose<R: Rng>(&mut self, mut rng: R) -> Self::Item {
		self.list.choose(&mut rng).unwrap()
	}

	fn choose_multiple<R: Rng>(&mut self, mut rng: R, amount: usize) -> Vec<Self::Item> {
		self.list.choose_multiple(&mut rng, amount).collect()
	}
}

pub struct TakingList<T> {
	list: Vec<T>,
}
impl<T> Urne for TakingList<T> {
	type Item = T;

	fn choose<R: Rng>(&mut self, mut rng: R) -> Self::Item {
		let i = rng.gen_range(0..self.list.len());
		// TODO: consider a `swap_remove`
		self.list.remove(i)
	}

	fn choose_multiple<R: Rng>(&mut self, mut rng: R, amount: usize) -> Vec<Self::Item> {
		(0..amount).map(|_| self.choose(&mut rng)).collect()
	}
}
