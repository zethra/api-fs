mod filesystem;

use failure::Error;
use fuse::{FileAttr, FileType};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use time::Timespec;

pub struct ApiFS {
    fs_endpoints: Box<FSEndPoint>,
    api_definition: ApiDefinition,
    api_def_path: PathBuf,
}

impl ApiFS {
    pub fn init(path: impl AsRef<Path>) -> Result<ApiFS, Error> {
        let api_def_path = path.as_ref().to_path_buf();
        let api_definition = ApiDefinition::load(&path)?;
        let fs_endpoints = get_objects_from_api(&api_definition)?;
        Ok(ApiFS {
            api_definition,
            fs_endpoints,
            api_def_path,
        })
    }

    pub fn sync() {
        unimplemented!();
    }
}

fn get_objects_from_api(api_definition: &ApiDefinition) -> Result<Box<FSEndPoint>, Error> {
    unimplemented!();
}

enum FSEndPoint {
    File {
        last_updated: SystemTime,
        inode_number: u64,
        contents: String,
    },
    Directory {
        last_updated: SystemTime,
        inode_number: u64,
        contents: Vec<Box<FSEndPoint>>,
    },
}

impl FSEndPoint {
    fn new_file(inode_number: u64, contents: String) -> FSEndPoint {
        FSEndPoint::File {
            inode_number,
            contents,
            last_updated: SystemTime::now(),
        }
    }

    fn new_directory(inode_number: u64, contents: Vec<Box<FSEndPoint>>) -> FSEndPoint {
        FSEndPoint::Directory {
            inode_number,
            contents,
            last_updated: SystemTime::now(),
        }
    }

    fn to_file_attr(&self) -> FileAttr {
        match self {
            FSEndPoint::File {
                last_updated,
                inode_number,
                contents,
            } => FileAttr {
                ino: *inode_number,
                size: contents.len() as u64,
                blocks: 0,
                atime: convert_time(&last_updated),
                mtime: convert_time(&last_updated),
                ctime: convert_time(&last_updated),
                crtime: convert_time(&last_updated),
                kind: FileType::RegularFile,
                perm: 0o644,
                nlink: 1,
                uid: 501,
                gid: 20,
                rdev: 0,
                flags: 0,
            },
            FSEndPoint::Directory {
                last_updated,
                inode_number,
                contents,
            } => FileAttr {
                ino: *inode_number,
                size: 0,
                blocks: 0,
                atime: convert_time(&last_updated),
                mtime: convert_time(&last_updated),
                ctime: convert_time(&last_updated),
                crtime: convert_time(&last_updated),
                kind: FileType::Directory,
                perm: 0o644,
                nlink: 1,
                uid: 501,
                gid: 20,
                rdev: 0,
                flags: 0,
            },
        }
    }
}

fn convert_time(time: &SystemTime) -> Timespec {
    let duration = time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    Timespec {
        sec: duration.as_secs() as i64,
        nsec: duration.subsec_nanos() as i32,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiDefinition {
    url: String,
    endpoints: Vec<String>,
}

impl ApiDefinition {
    fn load(path: impl AsRef<Path>) -> Result<ApiDefinition, Error> {
        Ok(serde_yaml::from_reader(File::open(path.as_ref())?)?)
    }
}
