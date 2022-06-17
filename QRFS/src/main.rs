mod my_inode;
mod my_funtions;
mod structs;

use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
use time::{Timespec};
use std::env;
use std::mem;
use std::ffi::OsStr;
use std::fmt::Display;
use crate::structs::{Disk, Inode};
use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};

struct QRFS {
    disk: Disk
}

impl QRFS {
    fn new(root_path: String) -> Self {
        let max_files: usize = 1024;
        let memory_size: usize = 1024 * 1024 * 1024;
        let block_size: usize = max_files * (mem::size_of::<Box<[Inode]>>() + mem::size_of::<Inode>());
        let disk = Disk::new(root_path, memory_size, block_size);
        QRFS {
            disk
        }
    }
}

impl Filesystem for QRFS {
    fn mkdir(&mut self, _req: &Request, parent: u64, name: &OsStr, _mode: u32, reply: ReplyEntry) {
        let reference_index = self.disk.find_index_of_empty_reference_in_inode(parent);
        match reference_index {
            Some(reference_index) => {
                let ino = self.disk.find_ino_available();
                match ino {
                    Some(ino) => {
                        let ts = time::now().to_timespec();
                        let attr = FileAttr {
                            ino: ino as u64,
                            size: 0,
                            blocks: 1,
                            atime: ts,
                            mtime: ts,
                            ctime: ts,
                            crtime: ts,
                            kind: FileType::Directory,
                            perm: 0o755,
                            nlink: 0,
                            uid: 0,
                            gid: 0,
                            rdev: 0,
                            flags: 0,
                        };

                        let name = name.to_str().unwrap();
                        let name: Vec<char> = name.chars().collect();

                        let mut name_char = ['\0'; 64];
                        name_char[..name.len()].clone_from_slice(&name);

                        let inode = Inode {
                            name: name_char,
                            attributes: attr,
                            references: [None; 128]
                        };

                        self.disk.write_inode(inode);
                        self.disk.write_reference_in_inode(parent, reference_index, ino as usize);

                        reply.entry(&ts, &attr, 0);
                    },
                    None => reply.error(ENOSPC) // “No space left on device.”
                }
            },
            None => { println!("Limite de arquivos dentro da pasta atingido!"); reply.error(EIO); }
        }
    }

    fn rmdir(
        &mut self, 
        _req: &Request, 
        parent: u64, 
        name: &OsStr, 
        reply: ReplyEmpty
    ) {
        let name = name.to_str().unwrap();
        let inode = self.disk.find_inode_in_references_by_name(parent, name);

        match inode {
            Some(inode) => {
                let ino = inode.attributes.ino;
                self.disk.clear_reference_in_inode(parent, ino as usize);
                self.disk.clear_inode(ino);

                reply.ok();
            },
            None => reply.error(EIO) // "Input/output error."
        }
    }
}

fn main() {

    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
            return;
        }
    };
    let fs = QRFS::new(mountpoint.clone());
}
 