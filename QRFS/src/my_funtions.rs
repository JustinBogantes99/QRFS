use std::vec::Vec;
use std::mem;

use std::alloc::System;
use std::ptr;


static mut dir_list:Vec<String> = Vec::new();
static mut files_list:Vec<String> = Vec::new();
static mut files_content: Vec<String>= Vec::new();

static mut curr_dir_idx:  i32 = -1;
static mut curr_file_idx: i32 =  -1;
static mut curr_file_content_idx: i32 = -1;

pub unsafe fn add_dir( dir_name : String ) {
	curr_dir_idx+= 1;
	dir_list.push(dir_name);
}

/*
fn is_dir(  path: String )
{
	for ( int curr_idx = 0; curr_idx <= curr_dir_idx; curr_idx++ )
		if ( strcmp( path, dir_list[ curr_idx ] ) == 0 )
			return 1;
	return 0;
}*/