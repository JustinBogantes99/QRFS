use fuse::{FileAttr};
use std::str;
use std::mem;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use std::time::{Duration, SystemTime};
use time::now;
//use serde::{Serialize, Deserialize};
//use crate::serialization::FileAttrDef;
//use bincode::{serialize, deserialize};
use fuse::{FileType};

//big_array! { BigArray; }
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

pub struct MemoryBlock {
    data: Option<Box<[u8]>>
}

impl Disk {
    pub fn new(root_path:String, memory_size_in_bytes:usize, block_size:usize) -> Disk {

        let memory_block_quantity:usize = (memory_size_in_bytes / block_size) - 1;
        let inode_size = mem::size_of::<Box<[Inode]>>() + mem::size_of::<Inode>();
        let max_files = block_size / inode_size;
        let disk_file_path = format!("{}/.disco.risos", &root_path);
        let inode_table_file_path = format!("{}/.inode.risos", &root_path);
        let mut memory_blocks: Vec<MemoryBlock>;    
        let mut super_block: Vec<Option<Inode>>;
    
        super_block = Vec::with_capacity(1);
        memory_blocks = Vec::new();

        let ts = time::now().to_timespec();
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
        Disk {
            memory_blocks: memory_blocks.into_boxed_slice(),
            super_block: super_block.into_boxed_slice(),
            max_files,
            block_size,
            root_path
        }
    }
    pub fn find_ino_available(&self) -> Option<u64> {
        for index in 0..self.super_block.len() - 1 {
            if let Option::None = self.super_block[index] {
                let ino = (index as u64) + 1;
                return Option::Some(ino);
            }
        }
        Option::None
    }

    pub fn find_index_of_empty_memory_block(&self) -> Option<usize> {
        for index in 0..self.memory_blocks.len() - 1 {
            if let Option::None = self.memory_blocks[index].data {
                return Option::Some(index);
            }
        }
        Option::None
    }
    
    pub fn find_index_of_empty_reference_in_inode(&self, ino: u64) -> Option<usize> {
        let index = (ino as usize) - 1;
        match &self.super_block[index] {
            Some(inode) => inode.references.iter().position(|r| r == &None),
            None => panic!("Acceso de memoria invalido")
        }
    }

    pub fn write_inode(&mut self, inode: Inode) {
        if mem::size_of_val(&inode) > self.block_size {
            println!("No se puede guardar el inodo: tamaño mayor que el tamaño del bloque de memoria");
            return;
        }

        let index = (inode.attributes.ino - 1) as usize;
        self.super_block[index] = Some(inode);
    }

    pub fn write_reference_in_inode(&mut self, ino: u64, ref_index: usize, ref_content: usize) {
        let index = (ino as usize) - 1;
        match &mut self.super_block[index] {
            Some(inode) => {
                inode.references[ref_index] = Some(ref_content);
            },
            None => panic!("fn write_reference_in_inode: Inode não encontrado!")
        }
    }

    pub fn find_inode_in_references_by_name(&self, parent_inode_ino: u64, name: &str) -> Option<&Inode> {
        let index = (parent_inode_ino as usize) - 1;
        let parent_inode = &self.super_block[index];

        match parent_inode {
            Some(parent_inode) => {
                // Procura pelo vetor de references do Inode
                for ino_ref in parent_inode.references.iter() {
                    // Se houver algum dado dentro de ino_ref, então entra no bloco e pega esse conteúdo
                    if let Some(ino) = ino_ref {
                        let index: usize = (ino.clone() as usize) - 1;
                        let inode_ref = &self.super_block[index];

                        match inode_ref {
                            Some(inode) => {
                                let name_from_inode: String = inode.name.iter().collect::<String>();
                                let name_from_inode: &str = name_from_inode.as_str().trim_matches(char::from(0)); // Remoção de caracteres '\0'
                                let name = name.trim();
                                println!("    - lookup(name={:?}, name_from_inode={:?}, equals={})", name, name_from_inode, name_from_inode == name);
                                
                                if name_from_inode == name {
                                    return Some(inode);
                                }
                            },
                            None => panic!("fn get_inode_by_name: Inode reference não encontrado")
                        }
                    }
                }
            },
            None => panic!("fn get_inode_by_name: Inode parent não encontrado")
        }

        return None;
    }

    pub fn clear_reference_in_inode(&mut self, ino: u64, ref_value: usize) {
        let index = (ino - 1) as usize;
        let inode: &mut Option<Inode> = &mut self.super_block[index];
        
        match inode {
            Some(inode) => {
                let reference_index: Option<usize> = inode.references.iter().position(|r| match r {
                    Some(reference) => *reference == ref_value,
                    None => false
                });

                match reference_index {
                    Some(reference_index) => inode.references[reference_index] = None,
                    None => panic!("fn clear_reference_in_inode: Referência não encontrada no Inode.")
                }
            },
            None => panic!("fn clear_reference_in_inode: Tentativa de remoção de referência em um Inode vazio.")
        }
    }
    
    pub fn clear_inode(&mut self, ino: u64) {
        let index = (ino - 1) as usize;
        self.super_block[index] = None;
    }
}