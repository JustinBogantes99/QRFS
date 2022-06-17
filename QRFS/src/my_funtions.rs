///extern crate fuse;

use std::vec::Vec;
use std::mem;
use fuse::{Filesystem, Request, ReplyCreate, ReplyEmpty, ReplyAttr, ReplyEntry, ReplyOpen, ReplyData, ReplyDirectory, ReplyWrite, FileType, FileAttr};
use std::alloc::System;
use std::ptr;
use std::fmt::Display;


static mut dir_list:Vec<String> = Vec::new();
static mut files_list:Vec<String> = Vec::new();
static mut files_content: Vec<String>= Vec::new();

static mut curr_dir_idx:  i32 = 0;
static mut curr_file_idx: i32 =  0;
static mut curr_file_content_idx: i32 = 0;

//struct QRFS{}
pub fn add_dir( dir_name : String ) {
	unsafe {
		curr_dir_idx+= 1;
		dir_list.push(dir_name);
		println!("{:?}",dir_list)
	}
	
}
pub fn is_dir(  path : String ) -> bool  {
	//path+=1; // Eliminating "/" in the path
	
	let mut curr_idx:usize  =  0 ;
	let es_dir: bool = true;
	unsafe{
		while curr_idx < curr_dir_idx.try_into().unwrap() {
			if dir_list[ curr_idx ].eq(&path)==true {
				return es_dir;
			}
			curr_idx+=1; 
		}
		return false;
	} 	
}

//impl qrfs {}
/*
fn is_dir(  path: String )
{
	for ( int curr_idx = 0; curr_idx <= curr_dir_idx; curr_idx++ )
		if ( strcmp( path, dir_list[ curr_idx ] ) == 0 )
			return 1;
	return 0;
}*/