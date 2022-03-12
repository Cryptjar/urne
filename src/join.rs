use alloc::vec::Vec;

use rand::Rng;

use crate::Urne;
use crate::Urnenmodell;

pub struct Joiner<ModelA, ModelB, Fun> {
	model_a: ModelA,
	model_b: ModelB,
	adapter: Fun,
}
impl<ModelA, ModelB, Fun> Joiner<ModelA, ModelB, Fun>
where
	ModelA: Urnenmodell,
	ModelB: Urnenmodell,
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
impl<ModelA, ModelB, Fun> Urnenmodell for Joiner<ModelA, ModelB, Fun>
where
	ModelA: Urnenmodell,
	ModelB: Urnenmodell,
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

	fn choose<R: Rng>(&mut self, mut rng: R) -> Self::Item {
		(self.fun)(self.urne_a.choose(&mut rng), self.urne_b.choose(&mut rng))
	}

	fn choose_multiple<R: Rng>(&mut self, mut rng: R, amount: usize) -> Vec<Self::Item> {
		self.urne_a
			.choose_multiple(&mut rng, amount)
			.into_iter()
			.zip(self.urne_b.choose_multiple(&mut rng, amount))
			.map(|(a, b)| (self.fun)(a, b))
			.collect()
	}
}
