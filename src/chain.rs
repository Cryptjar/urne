use alloc::vec::Vec;

pub use either::Either;
use rand::Rng;

use crate::Urne;
use crate::UrneModel;


// TODO: consider removing the `adapter` and directly returning `Either`


pub struct Chain<ModelA, ModelB, Fun> {
	model_a: ModelA,
	model_b: ModelB,
	ratio_left: (u32, u32),
	adapter: Fun,
}
impl<ModelA, ModelB, Fun> Chain<ModelA, ModelB, Fun>
where
	ModelA: UrneModel,
	ModelB: UrneModel,
	Fun: for<'n> Fn<(Either<ModelA::Item<'n>, ModelB::Item<'n>>,)>,
{
	pub fn new(
		model_a: ModelA,
		model_b: ModelB,
		weigh_left: u32,
		weigh_right: u32,
		adapter: Fun,
	) -> Self {
		Chain {
			model_a,
			model_b,
			ratio_left: (weigh_left, weigh_left + weigh_right),
			adapter,
		}
	}
}
impl<ModelA, ModelB, Fun> UrneModel for Chain<ModelA, ModelB, Fun>
where
	ModelA: UrneModel,
	ModelB: UrneModel,
	Fun: for<'n> Fn<(Either<ModelA::Item<'n>, ModelB::Item<'n>>,)>,
{
	type Item<'a> = <Fun as FnOnce<(Either<ModelA::Item<'a>, ModelB::Item<'a>>,)>>::Output where Fun: 'a, ModelA: 'a, ModelB: 'a;
	type Peeking<'a> = ChainUrne<'a, ModelA::Peeking<'a>, ModelB::Peeking<'a>, Fun> where Fun: 'a, ModelA: 'a, ModelB: 'a;
	type Taking<'a> = ChainUrne<'a, ModelA::Taking<'a>, ModelB::Taking<'a>, Fun> where Fun: 'a, ModelA: 'a, ModelB: 'a;

	fn peeking(&self) -> Self::Peeking<'_> {
		ChainUrne {
			urne_a: self.model_a.peeking(),
			urne_b: self.model_b.peeking(),
			ratio_left: self.ratio_left,
			fun: &self.adapter,
		}
	}

	fn taking(&self) -> Self::Taking<'_> {
		ChainUrne {
			urne_a: self.model_a.taking(),
			urne_b: self.model_b.taking(),
			ratio_left: self.ratio_left,
			fun: &self.adapter,
		}
	}

	fn size(&self) -> usize {
		self.model_a.size() + self.model_b.size()
	}
}

pub struct ChainUrne<'a, UrneA, UrneB, Fun> {
	urne_a: UrneA,
	urne_b: UrneB,
	ratio_left: (u32, u32),
	fun: &'a Fun,
}
impl<'a, UrneA, UrneB, Fun> Urne for ChainUrne<'a, UrneA, UrneB, Fun>
where
	UrneA: Urne,
	UrneB: Urne,
	Fun: Fn<(Either<UrneA::Item, UrneB::Item>,)>,
{
	type Item = <Fun as FnOnce<(Either<UrneA::Item, UrneB::Item>,)>>::Output;
	type MultiItem = Vec<Self::Item>;

	fn choose<R: Rng>(&mut self, mut rng: R) -> Option<Self::Item> {
		let take_left = rng.gen_ratio(self.ratio_left.0, self.ratio_left.1);

		let either = {
			if take_left {
				self.urne_a
					.choose(&mut rng)
					.map(|a| Either::Left(a))
					.or_else(|| self.urne_b.choose(&mut rng).map(|b| Either::Right(b)))
			} else {
				self.urne_b
					.choose(&mut rng)
					.map(|b| Either::Right(b))
					.or_else(|| self.urne_a.choose(&mut rng).map(|a| Either::Left(a)))
			}
		};

		either.map(self.fun)
	}

	fn choose_multiple<R: Rng>(&mut self, mut rng: R, amount: usize) -> Option<Self::MultiItem> {
		// TODO consider, relaxing `Option<impl Iterator>` as a return type
		// Because this the current design can easily lead to a None, especially,
		// in the taking-path.

		let distribution: Vec<bool> = (0..amount)
			.map(|_| rng.gen_ratio(self.ratio_left.0, self.ratio_left.1))
			.collect();

		let left_amt = distribution.iter().filter(|b| **b).count();
		let right_amt = amount - left_amt;

		let lefts = self.urne_a.choose_multiple(&mut rng, left_amt);

		let rights = self.urne_b.choose_multiple(&mut rng, right_amt);

		lefts.zip(rights).map(|(iter_a, iter_b)| {
			let (mut iter_a, mut iter_b) = (iter_a.into_iter(), iter_b.into_iter());

			distribution
				.into_iter()
				.map(|take_left| {
					if take_left {
						Either::Left(iter_a.next().unwrap())
					} else {
						Either::Right(iter_b.next().unwrap())
					}
				})
				.map(self.fun)
				.collect()
		})
	}

	fn size(&self) -> usize {
		self.urne_a.size() + self.urne_b.size()
	}
}
