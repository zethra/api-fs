use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use futures::future::{lazy, Future};
use libc::ENOENT;
use log::*;
use reqwest;
use std::env;
use std::ffi::OsStr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{thread, time::Duration};
use time::Timespec;

const TTL: Timespec = Timespec { sec: 1, nsec: 0 }; // 1 second

const HELLO_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: Timespec { sec: 0, nsec: 0 }, // 1970-01-01 00:00:00
    mtime: Timespec { sec: 0, nsec: 0 },
    ctime: Timespec { sec: 0, nsec: 0 },
    crtime: Timespec { sec: 0, nsec: 0 },
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

const HELLO_TXT_CONTENT: &str = "Hello World!\n";

const HELLO_TXT_ATTR: FileAttr = FileAttr {
    ino: 2,
    size: 13,
    blocks: 1,
    atime: Timespec { sec: 0, nsec: 0 }, // 1970-01-01 00:00:00
    mtime: Timespec { sec: 0, nsec: 0 },
    ctime: Timespec { sec: 0, nsec: 0 },
    crtime: Timespec { sec: 0, nsec: 0 },
    kind: FileType::RegularFile,
    perm: 0o644,
    nlink: 1,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

struct HelloFS {
    rx: Receiver<String>,
    poke: Sender<()>,
}

impl Filesystem for HelloFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 && name.to_str() == Some("hello.txt") {
            reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
            2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
            _ => reply.error(ENOENT),
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
        if ino == 2 {
            // let s = match get_data() {
            //     Ok(s) => s,
            //     Err(e) => {
            //         error!("{:?}", e);
            //         reply.error(ENOENT);
            //         return;
            //     }
            // };
            // thread::sleep(Duration::from_secs(1));
            // let s = "[\n{}\n]";
            self.poke.send(()).unwrap();
            let s = self.rx.recv().unwrap();
            info!("Size: {}", _size);
            info!("Sending: {}", s);
            // let s = "Hello test";
            reply.data(&s.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
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
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
            (2, FileType::RegularFile, "hello.txt"),
        ];

        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            // i + 1 means the index of the next entry
            reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
        }
        reply.ok();
    }
}

fn get_data() -> reqwest::Result<String> {
    reqwest::get("http://localhost:9000/static/some.txt")?.text()
}

fn main() {
    env_logger::init();

    let (tx, rx) = mpsc::channel();
    let (poke, poked) = mpsc::channel();

    thread::spawn(move || loop {
        poked.recv().unwrap();
        // tx.send("Test".to_owned()).unwrap();
        tx.send(get_data().unwrap()).unwrap();
    });

    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["-o", "fsname=hello"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();
    fuse::mount(HelloFS { rx, poke }, &mountpoint, &options).unwrap();
}
