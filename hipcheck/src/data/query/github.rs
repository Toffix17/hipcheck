// SPDX-License-Identifier: Apache-2.0

//! Query group for data pertaining to a remote GitHub source.

use crate::data::source::Remote;
use crate::data::source::SourceQuery;
use crate::error::Error;
use crate::error::Result;
use std::rc::Rc;

/// Queries about a remote GitHub source
#[salsa::query_group(GitHubProviderStorage)]
pub trait GitHubProvider: SourceQuery {
	/// Returns the `Remote` struct for a remote GitHub source
	///
	/// Prefer using the other queries in this group over using
	/// the `Remote` directly
	fn remote_github(&self) -> Result<Rc<Remote>>;

	/// Returns the repository owner
	fn owner(&self) -> Result<Rc<String>>;

	/// Returns the repository name
	fn repo(&self) -> Result<Rc<String>>;

	/// Returns the pull request number if this is a single pull
	/// request query. Returns an error otherwise.
	fn pull_request(&self) -> Result<u64>;
}

/// Derived query implementations.  Return values are wrapped in an
/// `Rc` to keep cloning cheap.

fn remote_github(db: &dyn GitHubProvider) -> Result<Rc<Remote>> {
	let remote = db
		.remote()
		.ok_or_else(|| Error::msg("git source is not remote"))?;
	match remote.as_ref() {
		Remote::Unknown(_) => Err(Error::msg("unknown remote host (supported: github.com)")),
		Remote::GitHub { .. } => Ok(remote),
		Remote::GitHubPr { .. } => Ok(remote),
	}
}

fn owner(db: &dyn GitHubProvider) -> Result<Rc<String>> {
	let remote = db.remote_github()?;

	match remote.as_ref() {
		Remote::GitHub { owner, .. } => Ok(Rc::new(owner.to_string())),
		Remote::GitHubPr { owner, .. } => Ok(Rc::new(owner.to_string())),
		_ => Err(Error::msg(
			"unsupported remote host (supported: github.com)",
		)),
	}
}

fn repo(db: &dyn GitHubProvider) -> Result<Rc<String>> {
	let remote = db.remote_github()?;

	match remote.as_ref() {
		Remote::GitHub { repo, .. } => Ok(Rc::new(repo.to_string())),
		Remote::GitHubPr { repo, .. } => Ok(Rc::new(repo.to_string())),
		_ => Err(Error::msg(
			"unsupported remote host (supported: github.com)",
		)),
	}
}

fn pull_request(db: &dyn GitHubProvider) -> Result<u64> {
	let remote = db.remote_github()?;

	match remote.as_ref() {
		Remote::GitHub { .. } => Err(Error::msg("This is the wrong kind of Remote")),
		Remote::GitHubPr { pull_request, .. } => Ok(*pull_request),
		_ => Err(Error::msg("This is not a single github.com pull request")),
	}
}
