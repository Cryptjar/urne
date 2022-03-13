use rand::Rng;

use crate::Urne;
use crate::UrneModel;

pub struct Mapper<Model, Fun> {
	model: Model,
	adapter: Fun,
}
impl<Model, Fun> Mapper<Model, Fun>
where
	Model: UrneModel,
	Fun: for<'n> Fn<(Model::Item<'n>,)>,
{
	pub fn new(model: Model, adapter: Fun) -> Self {
		Mapper {
			model,
			adapter,
		}
	}
}
impl<Model, Fun> UrneModel for Mapper<Model, Fun>
where
	Model: UrneModel,
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
	type MultiItem = core::iter::Map<<<U as Urne>::MultiItem as IntoIterator>::IntoIter, &'a Fun>;

	// Using #![feature(type_alias_impl_trait)]:
	//type MultiItem = impl IntoIterator<Item=Self::Item>;

	fn choose<R: Rng>(&mut self, rng: R) -> Option<Self::Item> {
		self.urne.choose(rng).map(self.fun)
	}

	fn choose_multiple<R: Rng>(&mut self, rng: R, amount: usize) -> Option<Self::MultiItem> {
		let foo = self
			.urne
			.choose_multiple(rng, amount)
			.map(|iter| iter.into_iter().map(self.fun));
		foo
	}
}
