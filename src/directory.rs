use std::{
    ops,
    marker,
};
use std::rc::Rc;

use utils::{
    FromPtr,
    NewFromPtr
};

use database::{Database, DatabasePtr};
use Filenames;

use ffi;

#[derive(Debug)]
pub(crate) struct DirectoryPtr {
    pub ptr: *mut ffi::notmuch_directory_t
}

impl ops::Drop for DirectoryPtr {
    fn drop(&mut self) {
        unsafe {
            ffi::notmuch_directory_destroy(self.ptr)
        };
    }
}
impl !Send for DirectoryPtr {}
impl !Sync for DirectoryPtr {}

#[derive(Debug)]
pub struct Directory(pub(crate) Rc<DirectoryPtr>, Database);

impl Directory{

    pub fn child_directories(self: &Self) -> Filenames{
        Filenames::new(unsafe {
            ffi::notmuch_directory_get_child_directories(self.0.ptr)
        }, self.clone())
    }
}

impl NewFromPtr<*mut ffi::notmuch_directory_t, Database> for Directory {
    fn new(ptr: *mut ffi::notmuch_directory_t, parent: Database) -> Directory {
        Directory(Rc::new(DirectoryPtr{ptr}), parent)
    }
}

impl Clone for Directory {
    fn clone(&self) -> Self {
        Directory(self.0.clone(), self.1.clone())
    }
}

// unsafe impl Send for Directory{}
// impl !Sync for Directory {}
