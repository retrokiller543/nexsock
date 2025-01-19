#![cfg(feature = "git")]

use git2::Repository;
use std::path::Path;
use std::process::Command;

pub trait GitService {
    fn repository_path(&self) -> &Path;
    fn repository_url(&self) -> String;

    #[tracing::instrument(skip(self))]
    #[inline]
    fn clone_repo(&self) -> crate::error::Result<Repository> {
        let res = Command::new("git")
            .args([
                "clone",
                &self.repository_url(),
                self.repository_path()
                    .to_str()
                    .expect("Path was not set correctly for the Service"),
            ])
            .output();

        match res {
            Ok(_) => self.open(),
            Err(err) => Err(err.into()),
        }
    }

    #[tracing::instrument(skip(self))]
    #[inline]
    fn open(&self) -> crate::error::Result<Repository> {
        Repository::open(self.repository_path()).map_err(Into::into)
    }

    #[tracing::instrument(skip(self))]
    #[inline]
    fn clone_or_open(&self) -> crate::error::Result<Repository> {
        if self.repository_path().exists() {
            self.open()
        } else {
            self.clone_repo()
        }
    }
}
