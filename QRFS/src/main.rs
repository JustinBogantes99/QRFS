mod structs;

use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
use time::{Timespec};
use std::env;
use std::mem;
use std::ffi::OsStr;
use std::fmt::Display;
use crate::structs::{Disk, Inode};
use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
//Estrucutura del disco
struct QRFS {
    disk: Disk
}

impl QRFS {
    //implementacion del nuevo volumen que se crea con el path 
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
//implementacion de las funciones del file system
impl Filesystem for QRFS {
    //implementacion del getattr de structs 
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        reply.error(ENOSYS);
    }
    //implementacion del readdir de structs 
    fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: i64, mut reply: ReplyDirectory) {
        println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
        reply.error(ENOSYS);

    }
    //funcion que permite ver el contenido del disco 
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("lookup(parent={:?}, name={:?})", parent, name);
        let file_name = name.to_str().unwrap();
        let inode = self.disk.buscar_inode_rerefenciadoxnombre(parent, file_name);
        match inode {
            Some(inode) => {
                let ttl = time::now().to_timespec();
                println!("        - lookup(parent={:?}, attr={:?})", parent, inode.attributes);
                reply.entry(&ttl, &inode.attributes, 0)
            },
            None => reply.error(ENOENT) // “No such file or directory.”
        }
    }
    fn create(&mut self, _req: &Request, parent: u64, name: &OsStr, mode: u32, flags: u32, reply: ReplyCreate){
        println!("create(name={:?}, mode={}, flags={})", name, mode, flags);
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
    let options = ["-o", "nonempty"].iter().map(|o| o.as_ref()).collect::<Vec<&OsStr>>();
    fuse::mount(fs, &mountpoint, &options).unwrap();
}
 