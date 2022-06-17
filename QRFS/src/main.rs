mod my_inode;
mod my_funtions;
use std::fmt::Display;

fn mainadd_dir(){
    let mensaje="Hola";
    my_funtions::add_dir(mensaje.to_string());
}

fn main() {
    mainadd_dir();
    let mut resultado:bool = my_funtions::is_dir(("hola").to_string());
    println!("{}",resultado);
}
