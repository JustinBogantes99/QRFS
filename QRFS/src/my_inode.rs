use std::mem;
const my_block_size: u32 = 1024;
/*
Estructura utilizada para almacenar informacion de los fichero o archivos
*/
//#[repr(C)]
struct my_inode {

    uid:u16,        //Identificador del usuario
    gid:u16,        //identificador del grupo
    mode:u32,       //Permisos
    ctime:u32,      //Tiempo de creacion
    mtime:u32,      //Ultima modificacion
    size:i32,       //Tamaño en bytes
    indir_1:i32,    //
    indir_2:i32,    //
    pad:i32         //
}
// char filename[MY_FILENAME_SIZE];
//#[repr(C)]
struct my_dirent {
    valid:i32,			// Bandera que indica si la entrada es válido o no (por defecto en 0)
    isDir:i32,		    // Bandera que indica si una entrada es un directorio o no
    inode:i32,		    // Índice como inodo
    filename:String
}

enum ConstBloque {
    //DIR_ENTS_PER_BLK = my_block_size / mem::size_of::<my_dirent>(),
    //INODES_PER_BLK = my_block_size / mem::size_of::<my_inode>(),
    //PTRS_PER_BLK = my_block_size / mem::size_of::<u32>()
}
//mode,size,which_iNode,direct_array[NUM_DIRECT_ENT],indir_1,  indir_2
fn create_inode(){
    
}