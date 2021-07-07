use nom::{IResult, Err};
use nom::sequence::tuple;
use nom::character::complete::multispace0;
use nom::number::complete::float;
use crate::names::{CNTB, Group, PRIM, HEAD, CNTE};
use nom::bytes::complete::{take_until, tag};
use nom::error::ParseError;
use nom::multi::separated_list0;
use std::mem::transmute;
use nom::combinator::opt;
use nom::branch::alt;
use crate::shapes::{tuple_pyramid, tuple_Cylinder, tuple_box, tuple_rectangularTorus, tuple_CircularTorus, tuple_EllipticalDish, tuple_SphericalDish, SphericalDish, PrimShapes, tuple_Snout, tuple_Sphere, tuple_Line};
use crate::shapes::PrimShapes::*;
use slab::Slab;
use slab_tree::NodeMut;
use std::borrow::Borrow;


pub fn parse_version(input:&str)->IResult<&str,Vec<f32>>{
    let (input,(_,x,_,y))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    let arr=vec![x,y];
    Ok((input,arr))
}
pub fn parse_translation(input:&str)->IResult<&str,[f32;3]>{
    let (input,(_,x,_,y,_,z))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,[x,y,z]))
}
pub fn parse_cntb(input: &str, mut sum:u8) ->IResult<&str, (u8, CNTB)> {
    let (input,(_,_,name,translation,_,material))=tuple((
        parse_version,
        multispace0,
        take_until("\r"),
        parse_translation,
        multispace0,
        float,
    ))(input)?;
    let CNTB=CNTB{
        name: name.to_string(),
        translation,
        material
    };
    sum+=1;
    Ok((input,(sum,CNTB)))
}
pub fn parse_prim(input: &str) ->IResult<&str, PRIM> {
    let (input,(_,_,types,M3x4,bboxLocal))=tuple((
        parse_version,
        multispace0,
        float,
        parse_M3x4,
        parse_bboxLocal,
    ))(input)?;
    match types {
        1.0=>{
            let (input,pyramid)=tuple_pyramid(input).unwrap();
            let data=PRIM{
                kind: "Pyramid".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: PyramidShape(pyramid),
            };
            Ok((input,data))
        }
        2.0=>{
            let (input,boxs)=tuple_box(input).unwrap();
            let data=PRIM{
                kind: "Box".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: BoxShape(boxs),
            };
            Ok((input,data))
        }
        3.0=>{
            let (input,rectangularTorus)=tuple_rectangularTorus(input).unwrap();
            let data=PRIM{
                kind: "RectangularTorus".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: RectangularTorusShape(rectangularTorus),
            };
            Ok((input,data))
        }
        4.0=>{
            let (input,circularTorus)=tuple_CircularTorus(input).unwrap();
            let data=PRIM{
                kind: "CircularTorus".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: CircularTorusShape(circularTorus),
            };
            Ok((input,data))
        }
        5.0=>{
            let (input,ellipticaldish)=tuple_EllipticalDish(input).unwrap();
            let data=PRIM{
                kind: "EllipticalDish".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: EllipticalDishShape(ellipticaldish),
            };
            Ok((input,data))
        }
        6.0=>{
            let (input,sphericalDish)=tuple_SphericalDish(input).unwrap();
            let data=PRIM{
                kind: "SphericalDish".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: SphericalDishShape(sphericalDish),
            };
            Ok((input,data))
        }
        7.0=>{
            let (input,snout)=tuple_Snout(input).unwrap();
            let data=PRIM{
                kind: "Snout".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: SnoutShape(snout),
            };
            Ok((input,data))
        }
        8.0=>{
            let (input,cylinder)=tuple_Cylinder(input).unwrap();
            let data=PRIM{
                kind: "Cylinder".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: CylinderShape(cylinder),
            };
            Ok((input,data))
        }
        9.0=>{
            let (input,sphere)=tuple_Sphere(input).unwrap();
            let data=PRIM{
                kind: "Sphere".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: SphereShape(sphere),
            };
            Ok((input,data))
        }
        10.0=>{
            let (input,lineShape)=tuple_Line(input).unwrap();
            let data=PRIM{
                kind: "Line".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: LineShape(lineShape),
            };
            Ok((input,data))
        }
        _ => Err(panic!("Problem opening the file")),
    }

}
pub fn parse_cnte(input: &str, mut sum:u8) ->IResult<&str, (u8, CNTE)> {
    let (input,(_,_))=tuple((
        multispace0,
        parse_version,
    ))(input)?;
    sum-=1;
    Ok((input,(sum,CNTE)))
}
pub fn parse_M3x4(input:&str)->IResult<&str,Vec<f32>>{
    let (input,(x,y,z))=tuple((
        parse_4float,
        parse_4float,
        parse_4float,
        ))(input)?;
    let arr=[x,y,z].concat();
    Ok((input,arr))
}
pub fn parse_4float(input:&str)->IResult<&str,[f32;4]>{
    let (input,(_,x,_,y,_,z,_,a))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,[x,y,z,a]))
}
pub fn parse_bboxLocal(input:&str)->IResult<&str,[f32;6]>{
    let (input,(_,a,_,b,_,c,_,d,_,e,_,f))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        ))(input)?;
    Ok((input,[a,b,c,d,e,f]))
}
#[test]
fn test_parse_M3x4(){
    let data="-0.0010000     0.0000000     0.0000000    12.4900000
                     0.0000000    -0.0010000     0.0000000    12.2800000
                     0.0000000     0.0000000     0.0010000     2.1625000";
    let (input,out)=parse_M3x4(data).unwrap();
    //let (input,out)=parse_4float(data).unwrap();
    println!("out={:?}",out);
}
#[test]
fn test_join(){
    let arr=[1f32];
    let brr=[2f32];
    let crr=[arr,brr].concat();
    println!("crr={:?}",crr);
}
#[test]
fn test_parse_bboxLocal(){
    let data="-57.00        -57.00        -68.58
                     57.00         57.00         68.58";
    let (input,out)=parse_bboxLocal(data).unwrap();
    println!("out={:?}",out);
}
#[test]
fn test_parse_prim(){
    let data="1     1
     7
    -0.0010000     0.0000000     0.0000000    12.4900000
     0.0000000    -0.0010000     0.0000000    12.2800000
     0.0000000     0.0000000     0.0010000     2.1974250
        -42.00        -42.00        -23.93
         42.00         42.00         23.93
    42.0000000    30.0000000    47.8500000     0.0000000     0.0000000
     0.0000000     0.0000000     0.0000000     0.0000000";
    //let mut arr = vec![];
    //let (input,arr)=parse_prim(data,arr).unwrap();
    //println!("arr={:?}",arr);
}
#[test]
fn test_separated(){
    let data="0.0000000     0.0000000     0.0010000     2.1625000";
    if let Ok((input,arr))=parse_separated(data){
        println!("arr={:?}",arr);
    }
}
pub fn parse_separated(input:&str)->IResult<&str,Vec<f32>>{
    let (input,out)=separated_list0(
        multispace0,
        float
    )(input)?;

    Ok((input,out))
}
