use crate::names::CNTB;
use nom::character::complete::multispace0;
use nom::number::complete::float;
use nom::IResult;
use nom::sequence::tuple;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimShapes{
    PyramidShape(Pyramid),
    BoxShape(Box),
    RectangularTorusShape(RectangularTorus),
    CircularTorusShape(CircularTorus),
    EllipticalDishShape(EllipticalDish),//5
    SphericalDishShape(SphericalDish),
    SnoutShape(Snout),
    CylinderShape(Cylinder),
    SphereShape(Sphere),
    LineShape(Line),
}
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub enum PrimShape{
    PyramidShape,
    BoxShape,
    RectangularTorusShape,
    CircularTorusShape,
    EllipticalDishShape,
    SphericalDishShape,//6
    SnoutShape,
    CylinderShape,
    SphereShape,
    LineShape,
}

//金字塔形 1
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct Pyramid{
    pub bottom:[f32;2],
    pub top:[f32;2],
    pub offset:[f32;2],
    pub height:f32,
}
//2
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct Box{
    pub lengths:[f32;3],
}
//矩形环面 3
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct RectangularTorus{
    pub inner_radius:f32,
    pub outer_radius:f32,
    pub height:f32,
    pub angle:f32,
}
//圆环 4
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct  CircularTorus{
    pub offset:f32,
    pub radius:f32,
    pub angle:f32,
}
//椭圆盘 5
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct EllipticalDish{
    pub baseRadius:f32,
    pub height:f32,
}
//球盘 6
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct SphericalDish{
    pub baseRadius:f32,
    pub height:f32,
}
//7
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct Snout{
    pub offset:[f32;2],
    pub bshear:[f32;2],
    pub tshear:[f32;2],
    pub radius_b:f32,
    pub radius_t:f32,
    pub height:f32,
}
//8
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct Cylinder{
    pub radius:f32,
    pub height:f32,
}
//9
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct Sphere{
    pub diameter:f32,
}
//10
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub struct Line{
    pub a:f32,
    pub b:f32,
}

//1
pub fn tuple_pyramid(input:&str) ->IResult<&str,Pyramid>{
    let (input,(_,x,_,y,_,z,_,a,_,b,_,c,_,d))=tuple((
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
        multispace0,
        float,
    ))(input)?;
    Ok((input,Pyramid{
        bottom: [x,y],
        top: [z,a],
        offset: [b,c],
        height: d
    }))

}
//2
pub fn tuple_box(input:&str)->IResult<&str,Box>{
    let (input,(_, x, _, y, _, z))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
    ))(input)?;
    Ok((input,Box{
        lengths: [x,y,z]
    }))
}
//3
pub fn tuple_rectangularTorus(input:&str)->IResult<&str,RectangularTorus>{
    let (input,(_, a, _, b,_, c,_,d))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
    ))(input)?;
    Ok((input,RectangularTorus{
        inner_radius: a,
        outer_radius: b,
        height: c,
        angle: d
    }))
}
//4
pub fn tuple_CircularTorus(input:&str)->IResult<&str,CircularTorus>{
    let (input,(_, x,_,y,_,z))=tuple((
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
    ))(input)?;
    Ok((input,CircularTorus{
        offset: x,
        radius: y,
        angle: z
    }))
}
//5
pub fn tuple_EllipticalDish(input:&str)->IResult<&str,EllipticalDish>{
    let (input,(_, x, _, y))=tuple((
        multispace0,
        float,
        multispace0,
        float,
    ))(input)?;
    Ok((input,EllipticalDish{
        baseRadius: x,
        height: y
    }))
}
//6
pub fn tuple_SphericalDish(input:&str)->IResult<&str,SphericalDish>{
    let (input,(_, x, _, y))=tuple((
        multispace0,
        float,
        multispace0,
        float,
    ))(input)?;
    Ok((input,SphericalDish{
        baseRadius: x,
        height: y
    }))
}
//7
pub fn tuple_Snout(input:&str)->IResult<&str,Snout>{
    let (input,(_,a,_,b,_,c,_,d,_,e,_,f,_,g,_,h,_,i)
    )=tuple((
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
        multispace0,
        float,
        multispace0,
        float,
        multispace0,
        float,
    ))(input)?;
    Ok((input,Snout{
        offset: [a,b],
        bshear: [c,d],
        tshear: [e,f],
        radius_b: g,
        radius_t: h,
        height:   i
    }))
}
//8
pub fn tuple_Cylinder(input:&str)->IResult<&str,Cylinder>{
    let (input,(_,x,_,y))=tuple((
        multispace0,
        float,
        multispace0,
        float,
    ))(input)?;
    Ok((input,Cylinder{
        radius: x,
        height: y
    }))
}
//9
pub fn tuple_Sphere(input:&str)->IResult<&str,Sphere>{
    let (input,(_,x))=tuple((
        multispace0,
        float,
    ))(input)?;
    Ok((input,Sphere{
        diameter: x
    }))
}
//10
pub fn tuple_Line(input:&str)->IResult<&str,Line>{
    let (input,(_,a, _,b))=tuple((
        multispace0,
        float,
        multispace0,
        float,
    ))(input)?;
    Ok((input,Line{
        a,
        b
    }))
}