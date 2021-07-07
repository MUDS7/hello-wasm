use crate::shapes::{PrimShapes, PrimShape};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub struct HEAD{
    pub info:String,
    pub note:String,
    pub data:String,
    pub user:String,
    pub encoding:String,
}
#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub struct MODL{
    pub project:String,
    pub name:String,
}
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct CNTB{
    pub name:String,
    pub translation:[f32;3],
    pub material:f32,
}
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub struct PRIM{
    pub kind:String,
    pub M_3x4:Vec<f32>,
    pub bboxLocal:[f32;6],
    pub group:PrimShapes,
}
#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub struct CNTE;
// #[derive(Debug)]
// pub struct  Group{
//     CNTB:CNTB,
//     pub(crate) PRIM:PRIM,
// }
#[derive(Debug,Serialize,Deserialize,PartialEq)]
pub enum Group{
    CNTB(CNTB),
    PRIM(PRIM),
    CNTE(CNTE),
    Head(HEAD),
    MODL(MODL),
}