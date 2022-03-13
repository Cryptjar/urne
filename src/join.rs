use alloc::vec::Vec;

use rand::Rng;

use crate::Urne;
use crate::UrneModel;

pub struct Joiner<ModelA, ModelB, Fun> {
	model_a: ModelA,
	model_b: ModelB,
	adapter: Fun,
}
impl<ModelA, ModelB, Fun> Joiner<ModelA, ModelB, Fun>
where
	ModelA: UrneModel,
	ModelB: UrneModel,
	Fun: for<'n> Fn<(ModelA::Item<'n>, ModelB::Item<'n>)>,
{
	pub fn new(model_a: ModelA, model_b: ModelB, adapter: Fun) -> Self {
		Joiner {
			model_a,
			model_b,
			adapter,
		}
	}
}
impl<ModelA, ModelB, Fun> UrneModel for Joiner<ModelA, ModelB, Fun>
where
	ModelA: UrneModel,
	ModelB: UrneModel,
	Fun: for<'n> Fn<(ModelA::Item<'n>, ModelB::Item<'n>)>,
{
	type Item<'a> = <Fun as FnOnce<(ModelA::Item<'a>, ModelB::Item<'a>)>>::Output where Fun: 'a, ModelA: 'a, ModelB: 'a;
	type Peeking<'a> = JoinUrne<'a, ModelA::Peeking<'a>, ModelB::Peeking<'a>, Fun> where Fun: 'a, ModelA: 'a, ModelB: 'a;
	type Taking<'a> = JoinUrne<'a, ModelA::Taking<'a>, ModelB::Taking<'a>, Fun> where Fun: 'a, ModelA: 'a, ModelB: 'a;

	fn peeking(&self) -> Self::Peeking<'_> {
		JoinUrne {
			urne_a: self.model_a.peeking(),
			urne_b: self.model_b.peeking(),
			fun: &self.adapter,
		}
	}

	fn taking(&self) -> Self::Taking<'_> {
		JoinUrne {
			urne_a: self.model_a.taking(),
			urne_b: self.model_b.taking(),
			fun: &self.adapter,
		}
	}

	fn size(&self) -> usize {
		core::cmp::min(self.model_a.size() , self.model_b.size())
	}
}

pub struct JoinUrne<'a, UrneA, UrneB, Fun> {
	urne_a: UrneA,
	urne_b: UrneB,
	fun: &'a Fun,
}
impl<'a, UrneA, UrneB, Fun> Urne for JoinUrne<'a, UrneA, UrneB, Fun>
where
	UrneA: Urne,
	UrneB: Urne,
	Fun: Fn<(UrneA::Item, UrneB::Item)>,
{
	type Item = <Fun as FnOnce<(UrneA::Item, UrneB::Item)>>::Output;
	type MultiItem = Vec<Self::Item>;

	fn choose<R: Rng>(&mut self, mut rng: R) -> Option<Self::Item> {
		self.urne_a
			.choose(&mut rng)
			.zip(self.urne_b.choose(&mut rng))
			.map(|(a, b)| (self.fun)(a, b))
	}

	fn choose_multiple<R: Rng>(&mut self, mut rng: R, amount: usize) -> Option<Self::MultiItem> {
		self.urne_a
			.choose_multiple(&mut rng, amount)
			.zip(self.urne_b.choose_multiple(&mut rng, amount))
			.map(|(iter_a, iter_b)| {
				iter_a
					.into_iter()
					.zip(iter_b)
					.map(|(a, b)| (self.fun)(a, b))
					.collect()
			})
	}

	fn size(&self) -> usize {
		core::cmp::min(self.urne_a.size() , self.urne_b.size())
	}
}
