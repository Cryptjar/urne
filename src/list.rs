use alloc::vec::Vec;
use core::marker::PhantomData;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::Urne;
use crate::UrneModel;

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
impl<L, T> UrneModel for List<L, T>
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

	fn size(&self) -> usize {
		self.list.as_ref().len()
	}
}

pub struct PeekingList<'a, T> {
	list: &'a [T],
}
impl<'a, T> Urne for PeekingList<'a, T> {
	type Item = &'a T;
	type MultiItem = rand::seq::SliceChooseIter<'a, [T], T>;

	fn choose<R: Rng>(&mut self, mut rng: R) -> Option<Self::Item> {
		self.list.choose(&mut rng)
	}

	fn choose_multiple<R: Rng>(&mut self, mut rng: R, amount: usize) -> Option<Self::MultiItem> {
		(amount <= self.list.len()).then(|| self.list.choose_multiple(&mut rng, amount))
	}

	fn size(&self) -> usize {
		self.list.len()
	}
}

use rand::seq::index::IndexVec;

/// Returns a list of `amount` many indices into a list of `length` size.
///
/// It is assumed that each returned index is remove from the list, reducing
/// the highest valid value for later indices.
fn sample_removable_index<R: Rng>(mut rng: R, length: usize, amount: usize) -> IndexVec {
	let length: u32 = length.try_into().unwrap();
	let amount: u32 = amount.try_into().unwrap();

	let mut indices = Vec::with_capacity(amount as usize);
	for limit in ((length - amount)..length).rev() {
		indices.push(rng.gen_range(0..=limit));
	}

	IndexVec::from(indices)
}

pub struct TakingList<T> {
	list: Vec<T>,
}
impl<T> Urne for TakingList<T> {
	type Item = T;
	type MultiItem = Vec<Self::Item>;

	fn choose<R: Rng>(&mut self, mut rng: R) -> Option<Self::Item> {
		(!self.list.is_empty()).then(|| {
			let i = rng.gen_range(0..self.list.len());
			self.list.swap_remove(i)
		})
	}

	fn choose_multiple<R: Rng>(&mut self, rng: R, amount: usize) -> Option<Self::MultiItem> {
		(amount <= self.list.len()).then(|| {
			let indices = sample_removable_index(rng, self.list.len(), amount);
			// TODO: consider wether this can be optimized
			indices
				.into_iter()
				.map(|i| self.list.swap_remove(i))
				.collect()
		})
	}

	fn size(&self) -> usize {
		self.list.len()
	}
}
