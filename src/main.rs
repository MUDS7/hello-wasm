#![feature(in_band_lifetimes)]
#![feature(trusted_random_access)]
#![feature(update_panic_count)]

mod shapes;
mod names;
mod parse_rvm;
mod parse_binary;

use nom::sequence::tuple;
use nom::character::complete::{alpha1, multispace0};
use nom::IResult;
use nom::bytes::complete::{tag, take_until};
use crate::names::{HEAD, MODL, CNTB, Group};
use crate::parse_rvm::{parse_version, parse_translation, parse_cntb, parse_prim, parse_cnte};
use std::mem::take;
use nom::number::complete::float;
//use nom::lib::std::iter::TrustedRandomAccess;
use std::fs::File;
use std::io::{Write, Read};
use slab::Slab;
use slab_tree::*;
use std::borrow::{ Borrow};
use wasm_bindgen::JsValue;
use crate::shapes::{parse_head, parse_modl, parse_kinds};
use std::io;

fn main() {
    let file=&String::from_utf8_lossy(include_bytes!("D:/RustProject/PIPE-100-b-1/rvm_txt.txt")).to_string() as &str ;
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
    println!("s={}",s);
}


#[test]
fn test_parse_head(){
    let head="HEAD
                     1     2
                AVEVA PDMS Design Mk12.1.SP4.0[4074]  (WINDOWS-NT 6.1)  (25 Jun 2013 : 20:47)

                Mon Jun 28 11:16:24 2021
                happyrust@LAPTOP-9LBAKCHI
                Unicode UTF-8
                MODL";

    let (input,HEAD)=parse_head(head).unwrap();
    if let Ok(mut file) =File::create("data.txt"){
        if let Ok(data)=serde_json::to_string(&HEAD){
            file.write_all(data.as_bytes());
        }
    }
    // let data=serde_json::to_string(&HEAD)?;
    // file.write_all(data.as_bytes());
     //println!("head={:?}",HEAD);
}
#[test]
fn test_parse_modl(){
    let data="MODL
                     1     1
                    SAM
                    /SAMPLE
                    CNTB";
    //let (input,MODL)=parse_modl(data).unwrap();
    if let Ok((input,MODL))=parse_modl(data){
    println!("MODL={:?}",MODL)};
}


fn read_data_text()->Result<HEAD,std::io::Error>{
    let mut file =File::open("data.txt")?;
    let mut data=String::new();
    file.read_to_string(&mut data)?;
    let head=serde_json::from_str(&data)?;
    Ok(head)
}
#[test]
fn test_read_data_text(){
    if let Ok(head)=read_data_text() {
        println!("head={:?}", head);
    }
}
fn read_arr_txt()->Result<Vec<Group>,std::io::Error>{
    let mut file=File::open("arr.txt")?;
    let mut data=String::new();
    file.read_to_string(&mut data)?;
    let arr=serde_json::from_str(&data)?;
    Ok(arr)
}
#[test]
fn test_read_arr_text(){
    if let Ok(arr)=read_arr_txt(){
        println!("arr={:?}",arr);
    }
}
#[test]
fn test_slab(){
    let mut slab=Slab::new();
    let hello={
        let entry=slab.vacant_entry();
        let key=entry.key();
        entry.insert((key,HEAD{
            info: "".to_string(),
            note: "".to_string(),
            data: "".to_string(),
            user: "".to_string(),
            encoding: "".to_string()
        }));
        key
    };
    let mut slab2=Slab::new();
    let data=slab2.insert("HEAD");
    println!("hello={:?}",slab[hello].1);
    println!("data={}",slab2[data]);
}
#[test]
fn test_slab_tree(){
    let mut tree = Tree::new();
    tree.set_root(1);

    let mut root = tree.root_mut().expect("root doesn't exist");
    root.append(2);
    root.append(3);
    root.append(4);
    root.append(5);
    root.append(6);
    let values = [2, 3, 4, 5,6];
    let root = root.as_ref();

    for (i, node_ref) in root.children().enumerate() {
        println!("node_ref={:?}",node_ref.data());
        assert_eq!(node_ref.data(), &values[i]);
    }
}
#[test]
fn test_slab_tree_2() {
    let mut tree = Tree::new();
    tree.set_root("hello");
    let mut root = tree.root_mut().expect("root doesn't exist");
    root.append("world");
    root.append("slab");
    root.append("tree");
    let mut root2 = root.append("a");
    root2.append("b");
    // let brr=next_level(root,"c");
    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    println!("s={}", s);
}
