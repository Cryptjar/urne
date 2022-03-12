use alloc::vec::Vec;

use rand::Rng;

use crate::Urne;
use crate::Urnenmodell;

pub struct Mapper<Model, Fun> {
	model: Model,
	adapter: Fun,
}
impl<Model, Fun> Mapper<Model, Fun>
where
	Model: Urnenmodell,
	Fun: for<'n> Fn<(Model::Item<'n>,)>,
{
	pub fn new(model: Model, adapter: Fun) -> Self {
		Mapper {
			model,
			adapter,
		}
	}
}
impl<Model, Fun> Urnenmodell for Mapper<Model, Fun>
where
	Model: Urnenmodell,
	Fun: for<'n> Fn<(Model::Item<'n>,)>,
{
	type Item<'a> = <Fun as FnOnce<(Model::Item<'a>,)>>::Output where Fun: 'a, Model: 'a;
	type Peeking<'a> = MapUrne<'a, Model::Peeking<'a>, Fun> where Fun: 'a, Model: 'a;
	type Taking<'a> = MapUrne<'a, Model::Taking<'a>, Fun> where Fun: 'a, Model: 'a;

	fn peeking(&self) -> Self::Peeking<'_> {
		MapUrne {
			urne: self.model.peeking(),
			fun: &self.adapter,
		}
	}

	fn taking(&self) -> Self::Taking<'_> {
		MapUrne {
			urne: self.model.taking(),
			fun: &self.adapter,
		}
	}
}

pub struct MapUrne<'a, U, Fun> {
	urne: U,
	fun: &'a Fun,
}
impl<'a, U, Fun, Out> Urne for MapUrne<'a, U, Fun>
where
	U: Urne,
	Fun: Fn(U::Item) -> Out,
{
	type Item = Out;

	fn choose<R: Rng>(&mut self, rng: R) -> Self::Item {
		(self.fun)(self.urne.choose(rng))
	}

	fn choose_multiple<R: Rng>(&mut self, rng: R, amount: usize) -> Vec<Self::Item> {
		self.urne
			.choose_multiple(rng, amount)
			.into_iter()
			.map(self.fun)
			.collect()
	}
}
