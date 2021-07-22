use wasm_bindgen::prelude::*;
use slab_tree::Tree;
use crate::names::Group;
use nom::Err;
use crate::shapes::{parse_head, parse_modl, parse_kinds};
use std::io;
use std::io::Read;

mod names;
mod shapes;
mod parse_rvm;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(){
    alert("hello");
}
#[wasm_bindgen]
pub fn wasm_out() ->String {

    let file=&String::from_utf8_lossy(include_bytes!("PIPE-100-B-1.rvm")).to_string() as &str ;//PIPE-100-B-1.rvm
    // let mut file = io::stdin(); // We get `Stdin` here.
    // file.read_to_string(&mut path)?;
    //let input=&input[..];
    let (input,HEAD)=parse_head(file).unwrap();
    println!("head={:?}",HEAD);
    let (input,MODL)=parse_modl(input).unwrap();
    println!("modl={:?}",MODL);

    let mut tree=Tree::new();
    tree.set_root(Group::Head(HEAD));
    let mut root = tree.root_mut().expect("root doesn't exist");
    let sum=0u8;

    root.append(Group::MODL(MODL));
    let (input,sum)=parse_kinds(input,root,sum).unwrap();
    let mut s=String::new();
    tree.write_formatted(&mut s).unwrap();
    s
}
