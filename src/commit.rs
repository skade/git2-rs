use std::kinds::marker;
use std::iter::Range;
use std::str;
use libc;

use {raw, Oid, Error, Signature, Tree, Time};

/// A structure to represent a git [commit][1]
///
/// [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects
pub struct Commit<'repo> {
    raw: *mut raw::git_commit,
    marker1: marker::ContravariantLifetime<'repo>,
    marker2: marker::NoSend,
    marker3: marker::NoSync,
}

/// An iterator over the parent commits of a commit.
pub struct Parents<'commit, 'repo: 'commit> {
    range: Range<uint>,
    commit: &'commit Commit<'repo>,
}

/// An iterator over the parent commits' ids of a commit.
pub struct ParentIds<'commit> {
    range: Range<uint>,
    commit: &'commit Commit<'commit>,
}

impl<'repo> Commit<'repo> {
    /// Create a new object from its raw component.
    ///
    /// This method is unsafe as there is no guarantee that `raw` is a valid
    /// pointer.
    pub unsafe fn from_raw(raw: *mut raw::git_commit) -> Commit<'repo> {
        Commit {
            raw: raw,
            marker1: marker::ContravariantLifetime,
            marker2: marker::NoSend,
            marker3: marker::NoSync,
        }
    }

    /// Get the id (SHA1) of a repository commit
    pub fn id(&self) -> Oid {
        unsafe { Oid::from_raw(raw::git_commit_id(&*self.raw)) }
    }

    /// Get the id of the tree pointed to by this commit.
    ///
    /// No attempts are made to fetch an object from the ODB.
    pub fn tree_id(&self) -> Oid {
        unsafe { Oid::from_raw(raw::git_commit_tree_id(&*self.raw)) }
    }

    /// Get the tree pointed to by a commit.
    pub fn tree(&self) -> Result<Tree<'repo>, Error> {
        let mut ret = 0 as *mut raw::git_tree;
        unsafe {
            try_call!(raw::git_commit_tree(&mut ret, &*self.raw));
            Ok(Tree::from_raw(ret))
        }
    }

    /// Get access to the underlying raw pointer.
    pub fn raw(&self) -> *mut raw::git_commit { self.raw }

    /// Get the full message of a commit.
    ///
    /// The returned message will be slightly prettified by removing any
    /// potential leading newlines.
    ///
    /// `None` will be returned if the message is not valid utf-8
    pub fn message(&self) -> Option<&str> {
        str::from_utf8(self.message_bytes()).ok()
    }

    /// Get the full message of a commit as a byte slice.
    ///
    /// The returned message will be slightly prettified by removing any
    /// potential leading newlines.
    pub fn message_bytes(&self) -> &[u8] {
        unsafe {
            ::opt_bytes(self, raw::git_commit_message(&*self.raw)).unwrap()
        }
    }

    /// Get the encoding for the message of a commit, as a string representing a
    /// standard encoding name.
    ///
    /// `None` will be returned if the encoding is not known
    pub fn message_encoding(&self) -> Option<&str> {
        let bytes = unsafe {
            ::opt_bytes(self, raw::git_commit_message(&*self.raw))
        };
        bytes.map(|b| str::from_utf8(b).unwrap())
    }

    /// Get the full raw message of a commit.
    ///
    /// `None` will be returned if the message is not valid utf-8
    pub fn message_raw(&self) -> Option<&str> {
        str::from_utf8(self.message_raw_bytes()).ok()
    }

    /// Get the full raw message of a commit.
    pub fn message_raw_bytes(&self) -> &[u8] {
        unsafe {
            ::opt_bytes(self, raw::git_commit_message_raw(&*self.raw)).unwrap()
        }
    }

    /// Get the full raw text of the commit header.
    ///
    /// `None` will be returned if the message is not valid utf-8
    pub fn raw_header(&self) -> Option<&str> {
        str::from_utf8(self.raw_header_bytes()).ok()
    }

    /// Get the full raw text of the commit header.
    pub fn raw_header_bytes(&self) -> &[u8] {
        unsafe {
            ::opt_bytes(self, raw::git_commit_raw_header(&*self.raw)).unwrap()
        }
    }

    /// Get the short "summary" of the git commit message.
    ///
    /// The returned message is the summary of the commit, comprising the first
    /// paragraph of the message with whitespace trimmed and squashed.
    ///
    /// `None` may be returned if an error occurs or if the summary is not valid
    /// utf-8.
    pub fn summary(&mut self) -> Option<&str> {
        self.summary_bytes().and_then(|s| str::from_utf8(s).ok())
    }

    /// Get the short "summary" of the git commit message.
    ///
    /// The returned message is the summary of the commit, comprising the first
    /// paragraph of the message with whitespace trimmed and squashed.
    ///
    /// `None` may be returned if an error occurs
    pub fn summary_bytes(&mut self) -> Option<&[u8]> {
        unsafe { ::opt_bytes(self, raw::git_commit_summary(self.raw)) }
    }

    /// Get the commit time (i.e. committer time) of a commit.
    ///
    /// The first element of the tuple is the time, in seconds, since the epoch.
    /// The second element is the offset, in minutes, of the time zone of the
    /// committer's preferred time zone.
    pub fn time(&self) -> Time {
        unsafe {
            Time::new(raw::git_commit_time(&*self.raw) as i64,
                      raw::git_commit_time_offset(&*self.raw) as int)
        }
    }

    /// Creates a new iterator over the parents of this commit.
    pub fn parents<'a>(&'a self) -> Parents<'a, 'repo> {
        let max = unsafe { raw::git_commit_parentcount(&*self.raw) as uint };
        Parents {
            range: range(0, max),
            commit: self,
        }
    }

    /// Creates a new iterator over the parents of this commit.
    pub fn parent_ids(&self) -> ParentIds {
        let max = unsafe { raw::git_commit_parentcount(&*self.raw) as uint };
        ParentIds {
            range: range(0, max),
            commit: self,
        }
    }

    /// Get the author of this commit.
    pub fn author(&self) -> Signature {
        unsafe {
            let ptr = raw::git_commit_author(&*self.raw);
            Signature::from_raw_const(self, ptr)
        }
    }

    /// Get the committer of this commit.
    pub fn committer(&self) -> Signature {
        unsafe {
            let ptr = raw::git_commit_committer(&*self.raw);
            Signature::from_raw_const(self, ptr)
        }
    }

    /// Amend this existing commit with all non-`None` values
    ///
    /// This creates a new commit that is exactly the same as the old commit,
    /// except that any non-`None` values will be updated. The new commit has
    /// the same parents as the old commit.
    ///
    /// For information about `update_ref`, see `new`.
    pub fn amend(&self,
                 update_ref: Option<&str>,
                 author: Option<&Signature>,
                 committer: Option<&Signature>,
                 message_encoding: Option<&str>,
                 message: Option<&str>,
                 tree: Option<&Tree<'repo>>) -> Result<Oid, Error> {
        let mut raw = raw::git_oid { id: [0, ..raw::GIT_OID_RAWSZ] };
        unsafe {
            try_call!(raw::git_commit_amend(&mut raw,
                                            &*self.raw(),
                                            update_ref.map(|s| s.to_c_str()),
                                            author.map(|s| &*s.raw()),
                                            committer.map(|s| &*s.raw()),
                                            message_encoding.map(|s| s.to_c_str()),
                                            message.map(|s| s.to_c_str()),
                                            tree.map(|t| &*t.raw())));
            Ok(Oid::from_raw(&raw))
        }
    }

    /// Get the specified parent of the commit.
    ///
    /// Use the `parents` iterator to return an iterator over all parents.
    pub fn parent(&self, i: uint) -> Result<Commit<'repo>, Error> {
        unsafe {
            let mut raw = 0 as *mut raw::git_commit;
            try_call!(raw::git_commit_parent(&mut raw, &*self.raw,
                                             i as libc::c_uint));
            Ok(Commit::from_raw(raw))
        }
    }

    /// Get the specified parent id of the commit.
    ///
    /// This is different from `parent`, which will attemptstempt to load the
    /// parent commit from the ODB.
    ///
    /// Use the `parent_ids` iterator to return an iterator over all parents.
    pub fn parent_id(&self, i: uint) -> Result<Oid, Error> {
        unsafe {
            let id = raw::git_commit_parent_id(self.raw, i as libc::c_uint);
            if id.is_null() {
                Err(Error::from_str("parent index out of bounds"))
            } else {
                Ok(Oid::from_raw(id))
            }
        }
    }
}

impl<'repo, 'commit> Iterator<Commit<'repo>> for Parents<'commit, 'repo> {
    fn next(&mut self) -> Option<Commit<'repo>> {
        self.range.next().map(|i| self.commit.parent(i).unwrap())
    }
    fn size_hint(&self) -> (uint, Option<uint>) { self.range.size_hint() }
}

impl<'repo, 'commit> DoubleEndedIterator<Commit<'repo>> for Parents<'commit, 'repo> {
    fn next_back(&mut self) -> Option<Commit<'repo>> {
        self.range.next_back().map(|i| self.commit.parent(i).unwrap())
    }
}

impl<'repo, 'commit> ExactSizeIterator<Commit<'repo>> for Parents<'commit, 'repo> {}

impl<'commit> Iterator<Oid> for ParentIds<'commit> {
    fn next(&mut self) -> Option<Oid> {
        self.range.next().map(|i| self.commit.parent_id(i).unwrap())
    }
    fn size_hint(&self) -> (uint, Option<uint>) { self.range.size_hint() }
}

impl<'commit> DoubleEndedIterator<Oid> for ParentIds<'commit> {
    fn next_back(&mut self) -> Option<Oid> {
        self.range.next_back().map(|i| self.commit.parent_id(i).unwrap())
    }
}

impl<'commit> ExactSizeIterator<Oid> for ParentIds<'commit> {}

#[unsafe_destructor]
impl<'repo> Drop for Commit<'repo> {
    fn drop(&mut self) {
        unsafe { raw::git_commit_free(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        let (_td, repo) = ::test::repo_init();
        let head = repo.head().unwrap();
        let target = head.target().unwrap();
        let mut commit = repo.find_commit(target).unwrap();
        assert_eq!(commit.message(), Some("initial"));
        assert_eq!(commit.id(), target);
        commit.message_raw().unwrap();
        commit.raw_header().unwrap();
        commit.message_encoding();
        commit.summary().unwrap();
        commit.tree_id();
        commit.tree().unwrap();
        assert_eq!(commit.parents().count(), 0);

        assert_eq!(commit.author().name(), Some("name"));
        assert_eq!(commit.author().email(), Some("email"));
        assert_eq!(commit.committer().name(), Some("name"));
        assert_eq!(commit.committer().email(), Some("email"));

        let sig = repo.signature().unwrap();
        let tree = repo.find_tree(commit.tree_id()).unwrap();
        let id = repo.commit(Some("HEAD"), &sig, &sig, "bar", &tree,
                             &[&commit]).unwrap();
        let head = repo.find_commit(id).unwrap();

        let new_head = head.amend(Some("HEAD"), None, None, None,
                                  Some("new message"), None).unwrap();
        let new_head = repo.find_commit(new_head).unwrap();
        assert_eq!(new_head.message(), Some("new message"));
    }
}

