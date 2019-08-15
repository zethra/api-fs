use super::ApiFS;
use crate::backend::Backend;
use failure::{err_msg, Error};
use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use libc::ENOENT;
use log::*;
use std::ffi::OsStr;
use std::rc::Rc;
use time::Timespec;

const TTL: Timespec = Timespec { sec: 1, nsec: 0 }; // 1 second

impl <T: Backend> Filesystem for ApiFS<T> {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        match (move || -> Result<FileAttr, Error> {
            let fs_endpoint = self
                .fs_endpoints
                .get(&parent)
                .ok_or(err_msg("FS EP doesn't exist"))?;
            let dir_contents = fs_endpoint.get_dir_contents()?;
            let file_inode = dir_contents
                .get(&*name.to_string_lossy().to_owned())
                .ok_or(err_msg("FS EP doesn't exist"))?;
            let file_ep = self
                .fs_endpoints
                .get(&file_inode)
                .ok_or(err_msg("FS EP doesn't exist"))?;
            Ok(file_ep.to_file_attr())
        })() {
            Ok(file_attr) => {
                reply.entry(&TTL, &file_attr, 0);
            }
            Err(e) => {
                error!("{:#?}", e);
                reply.error(ENOENT);
            }
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match (move || -> Result<FileAttr, Error> {
            let fs_endpoint = self
                .fs_endpoints
                .get(&ino)
                .ok_or(err_msg("FS EP doesn't exist"))?;
            Ok(fs_endpoint.to_file_attr())
        })() {
            Ok(attr) => {
                reply.attr(&TTL, &attr);
            }
            Err(e) => {
                error!("{:#?}", e);
                reply.error(ENOENT);
            }
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        reply: ReplyData,
    ) {
        match (move || -> Result<Rc<str>, Error> {
            let fs_endpoint = self
                .fs_endpoints
                .get(&ino)
                .ok_or(err_msg("FS EP doesn't exist"))?;
            let file_contents = fs_endpoint.get_file_contents()?;
            Ok(Rc::from(file_contents))
        })() {
            Ok(contents) => reply.data(&contents.as_bytes()[offset as usize..]),
            Err(e) => {
                error!("{:#?}", e);
                reply.error(ENOENT);
            }
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        match (|| -> Result<(), Error> {
            let fs_endpoint = self
                .fs_endpoints
                .get(&ino)
                .ok_or(err_msg("FS EP doesn't exist"))?;
            let dir_contents = fs_endpoint.get_dir_contents()?;
            if offset > 0 {
                reply.add(1, 2, FileType::Directory, ".");
            }
            if offset > 1 {
                reply.add(1, 2, FileType::Directory, "..");
            }
            for (i, (name, inode)) in dir_contents.iter().enumerate().skip(offset as usize + 2) {
                let file_ep = self
                    .fs_endpoints
                    .get(&inode)
                    .ok_or(err_msg("FS EP doesn't exist"))?;
                reply.add(*inode, i as i64 + 3, file_ep.get_file_type(), &name);
            }
            Ok(())
        })() {
            Ok(_) => {
                reply.ok();
            }
            Err(e) => {
                error!("{:#?}", e);
                reply.error(ENOENT);
            }
        }
    }
}
