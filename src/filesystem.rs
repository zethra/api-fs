use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use std::ffi::OsStr;

pub(super) struct ApiFileSystem;

impl Filesystem for ApiFileSystem {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {}

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {}

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        reply: ReplyData,
    ) {

    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {

    }
}
