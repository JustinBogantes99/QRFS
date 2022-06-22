use std::str;
use std::mem;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use time::now;
use time::Timespec;
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
use fuse::{FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
use libc::{ENOSYS, ENOENT, EIO, EISDIR, ENOSPC};
//Estructura necesaria para crear un disco
pub struct Disk {
    super_block: Box<[Option<Inode>]>,//arreglo que 
    memory_blocks: Box<[MemoryBlock]>,
    max_files: usize,
    block_size: usize,
    root_path: String
}
//Estructura necesaria para crear un Inode
pub struct Inode {
    pub name: [char; 64],
    pub attributes: FileAttr,
    pub references: [Option<usize>; 128]
}
//Estructura necesaria para el bloque de memoria
pub struct MemoryBlock {
    data: Option<Box<[u8]>>
}
//implementacion del disco
impl Disk {
    //creacion del nuevo volumen
    //toma como parametro la direccion , el tamño de memoria y el tamaño del bloque
    pub fn new(root_path:String, memory_size_in_bytes:usize, block_size:usize) -> Disk {
        let memory_block_quantity:usize = (memory_size_in_bytes / block_size) - 1;//cantidad del bloque de meoria
        let inode_size = mem::size_of::<Box<[Inode]>>() + mem::size_of::<Inode>();//tamaño que tendra el Inode
        let max_files = block_size / inode_size;//cantidad de archivos que se van a manejar 
        let disk_file_path = format!("{}/disco", &root_path);//direccion del disco
        let inode_table_file_path = format!("{}/inode", &root_path);//direccion de los inode
        let mut memory_blocks: Vec<MemoryBlock>;   //vector de memoria 
        let mut super_block: Vec<Option<Inode>>;//vector que posee los inode y las referencias

        
        File::create(&disk_file_path).expect("Error al crear archivos");
        File::create(&inode_table_file_path).expect("Error al crear archivos");

        super_block = Vec::with_capacity(1);//inicializacion del vector 
        memory_blocks = Vec::new();;//inicializacion del vector 

        let ts = time::now().to_timespec();//
        //atributos del inode
        let attr = FileAttr {
            ino: 1,
            size: 0,
            blocks: 0,
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

        let mut name = ['\0'; 64];
        name[0] = '.';

        let initial_inode = Inode {
            name,
            attributes: attr,
            references: [None; 128]
        };

        super_block.push(Some(initial_inode));
        println!("{}",root_path);
    

        for _ in super_block.len()..max_files {
            let value: Option<Inode> = Option::None;
            super_block.push(value);
        }

        for _ in memory_blocks.len()..memory_block_quantity {
            let value: MemoryBlock = MemoryBlock { data: Option::None };
            memory_blocks.push(value);
        }

        println!("Done =)");
        println!("\nTamaño del disco (kbytes): {}", memory_size_in_bytes / 1024);
        println!("Tamaño del bloque de memoria (kbytes): {}", block_size / 1024);
        println!("Cantidad máxima de archivos (Inode {} bytes): {}", inode_size, max_files);

        Disk {
            memory_blocks: memory_blocks.into_boxed_slice(),
            super_block: super_block.into_boxed_slice(),
            max_files,
            block_size,
            root_path
        }
    }
    //lee los atributos del inode
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        let ts = Timespec::new(0, 0);
        let attr = FileAttr {
            ino: 1,
            size: 0,
            blocks: 0,
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
        let ttl = Timespec::new(1, 0);
        if ino == 1 {
            reply.attr(&ttl, &attr);
        } else {
            reply.error(ENOSYS);
        }
    }
    //describe el directorio con informacion sobre el mismo
    fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: u64, mut reply: ReplyDirectory) {
        println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, &Path::new("."));
                reply.add(1, 1, FileType::Directory, &Path::new(".."));
            }
            reply.ok();
        } else {
            reply.error(ENOENT);
        }
    }
    //buscar_inode_rerefenciadoxnombre
    pub fn buscar_inode_rerefenciadoxnombre(&self, parent_inode_ino: u64, name: &str) -> Option<&Inode> {
        let index = (parent_inode_ino as usize) - 1;//saca el indice por merdio del padre del nodo
        let parent_inode = &self.super_block[index];//busca en el super bloque el nodo padre
        match parent_inode {
            Some(parent_inode) => {
                for ino_ref in parent_inode.references.iter() {//por cada nodo referenciado 
                    if let Some(ino) = ino_ref {
                        let index: usize = (ino.clone() as usize) - 1;//saca el tamaño del inode
                        let inode_ref = &self.super_block[index];//referebncia en el super bloque al nodo
                        match inode_ref {
                            Some(inode) => {
                                let name_from_inode: String = inode.name.iter().collect::<String>();//obtiene el nombre del inode
                                let name_from_inode: &str = name_from_inode.as_str().trim_matches(char::from(0)); 
                                let name = name.trim();
                                println!("    - lookup(name={:?}, name_from_inode={:?}, equals={})", name, name_from_inode, name_from_inode == name);
                                
                                if name_from_inode == name {
                                    return Some(inode);
                                }
                            },
                            None => panic!("referencia de nodo no encontrada")
                        }
                    }
                }
            },
            None => panic!("fn get_inode_by_name: Padre de inodo no encontrado")
        }
    
        return None;
    }
    
}