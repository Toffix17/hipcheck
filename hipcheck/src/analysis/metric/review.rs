// SPDX-License-Identifier: Apache-2.0

use crate::analysis::MetricProvider;
use crate::context::Context as _;
use crate::data::PullRequest;
use crate::error::Result;
use serde::Serialize;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ReviewOutput {
	pub pull_reviews: Vec<PullReview>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct PullReview {
	pub pull_request: Rc<PullRequest>,
	pub has_review: bool,
}

pub fn review_metric(db: &dyn MetricProvider) -> Result<Rc<ReviewOutput>> {
	log::debug!("running review metric");

	let pull_requests = db
		.pull_request_reviews()
		.context("failed to get pull request reviews")?;

	log::trace!("got pull requests [requests='{:#?}']", pull_requests);

	let mut pull_reviews = Vec::with_capacity(pull_requests.len());

	for pull_request in pull_requests.as_ref() {
		let has_review = pull_request.reviews > 0;
		pull_reviews.push(PullReview {
			pull_request: pull_request.clone(),
			has_review,
		});
	}

	log::info!("completed review metric");

	Ok(Rc::new(ReviewOutput { pull_reviews }))
}
