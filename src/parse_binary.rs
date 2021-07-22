use std::fs::OpenOptions;
use std::io::Read;
use nom::IResult;
use nom::sequence::tuple;
use nom::number::complete::{le_u8, be_u8, le_f32, be_f32};
use nom::multi::separated_list0;
use nom::bytes::complete::{tag, take_until, is_not, take_while, is_a};
use std::convert::TryInto;
use nom::character::complete::{multispace0, multispace1};
use std::str::from_utf8;
use crate::parse_rvm::{parse_version, parse_cnte, parse_bboxLocal};
use nom::character::{is_alphanumeric, is_space, is_digit};
use std::rt::panic_count::count_is_zero;
use serde::__private::from_utf8_lossy;
use crate::{parse_modl, shapes};
use crate::names::{Group, HEAD, MODL, CNTB, CNTE, PRIM};
use slab_tree::{Tree, NodeMut};
use crate::shapes::PrimShapes::*;
use crate::shapes::{Pyramid, PrimShapes, Box, RectangularTorus, CircularTorus, EllipticalDish, SphericalDish, Snout, Cylinder, Sphere, Line, FacetGroup};


#[test]
//HEAD { info: "HEAD", note: "AVEVA PDMS Design Mk12.1.SP4.0[4074]  (WINDOWS-NT 6.1)
//  (25 Jun 2013 : 20:47)\r", data: "Mon Jun 28 11:16:24 2021\r",
// user: "happyrust@LAPTOP-9LBAKCHI\r", encoding: "Unicode UTF-8\r"}
fn test_parse_binary(){
    let file=include_bytes!("D:/test_binary.rvm");
    let (input,head)=parse_head_binary(file).unwrap();
    //声明一个slab_tree
    let mut tree=Tree::new();
    tree.set_root(Group::Head(head));
    let mut root = tree.root_mut().expect("root doesn't exist");
    //sum用于查看cntb与cnte是否匹配
    let sum=0u8;

    let (input,modl)=parse_modl_binary(input).unwrap();
    root.append(Group::MODL(modl));
    let (input,out)=parse_kind_binary(input,root,sum).unwrap();
    //输出tree
    let mut s=String::new();
    tree.write_formatted(&mut s).unwrap();
    println!("s={}",s);
}
fn parse_kind_binary<'a>(input:&'a[u8], mut root:NodeMut<Group>, mut sum:u8) ->IResult<&'a[u8],u8>{
    let (input,(kind,))=tuple((
        parse_head_name,
        ))(input)?;
    println!("kind={:?}",kind);
    match kind {
        [67u8,78,84,66]=>{
            let (input,cntb)=parse_cntb_binary(input).unwrap();
            let root=root.append(Group::CNTB(cntb));
            sum+=1;
            let (input,sum)=parse_kind_binary(input,root,sum).unwrap();
            return Ok((input,sum))
        }
        [67u8,78,84,69]=>{
            let (input,cnte)=parse_cnte_binary(input).unwrap();
            sum-=1;
            root.append(Group::CNTE(CNTE));
            let (input,sum)=parse_kind_binary(input,root.parent().unwrap(),sum).unwrap();
            return Ok((input,sum))
        }
        [80u8,82,73,77]=>{
            let (input,prim)=parse_prim_binayy(input).unwrap();
            root.append(Group::PRIM(prim));
            let (input,sum)=parse_kind_binary(input,root,sum).unwrap();
            return Ok((input,sum))
        }
        _=>{
            return Ok((input,sum))
        }
        }

    }
fn parse_head_binary(input:&[u8]) ->IResult<&[u8],HEAD>{
    let (input,(a,_,note,date,user,unicode))=tuple((
        parse_head_name,
        parse_head_version,
        parse_head_note,
        parse_head_date,
        parse_head_user,
        parse_head_unicode,
        ))(input)?;
    let note=from_utf8(note).unwrap();
    let date=from_utf8(date).unwrap();
    let user=from_utf8(user).unwrap();
    let unicode=from_utf8(unicode).unwrap();
    //println!("a={:?},note={:?},date={:?},user={:?},unicode={:?}",a,note,date,user,unicode);
    let HEAD=HEAD{
        info: "HEAD".to_string(),
        note: note.to_string(),
        data: date.to_string(),
        user: user.to_string(),
        encoding: unicode.to_string()
    };
    Ok((input,HEAD))
}

fn parse_head_name(input:&[u8])->IResult<&[u8],[u8;4]>{
    let (input,(_,a,_,b,_,c,_,d))=tuple((
        take_while(is_zero),
        le_u8,
        take_while(is_zero),
        le_u8,
        take_while(is_zero),
        le_u8,
        take_while(is_zero),
        le_u8,
        ))(input)?;
    Ok((input,[a,b,c,d]))
}
fn parse_head_version(input:&[u8])->IResult<&[u8],u8>{
    let (input,(_,a,_,b,_,c))=tuple((
        take_while(is_zero),
        le_u8,
        take_while(is_zero),
        le_u8,
        take_while(is_zero),
        le_u8,
        ))(input)?;
    Ok((input,1))
}
fn parse_head_note(input:&[u8])->IResult<&[u8],&[u8]>{
    //let mut arr=vec![];
    let (input,(_,_,note,))=tuple((
        take_while(is_zero),
        le_u8,
        take_while(is_not_zero),
        ))(input)?;
    Ok((input,note))
}
fn parse_head_date(input:&[u8])->IResult<&[u8],&[u8]>{
    let (input,(_,_,date))=tuple((
        take_while(is_zero),
        le_u8,
        take_while(is_not_zero),
        ))(input)?;
    Ok((input,date))
}
fn parse_head_user(input:&[u8])->IResult<&[u8],&[u8]>{
    let (input,(_,_,user))=tuple((
        take_while(is_zero),
        le_u8,
        take_while(is_not_zero),
        ))(input)?;
    Ok((input,user))
}
fn parse_head_unicode(input:&[u8])->IResult<&[u8],&[u8]>{
    let (input,(_,_,unicode))=tuple((
        take_while(is_zero),
        le_u8,
        take_while(is_not_zero),
        ))(input)?;
    Ok((input,unicode))
}
//MODL(MODL { project: "SAM\r", name: "/SAMPLE\r" })
fn parse_modl_binary(input:&[u8])->IResult<&[u8],MODL>{
    let (input,(name,_,_,_,project,_,_,names))=tuple((
        parse_head_name,
        parse_head_version,
        take_while(is_zero),
        le_u8,
        take_while(is_not_zero),
        take_while(is_zero),
        le_u8,
        take_while(is_not_zero),
        ))(input)?;
    //println!("name={:?},project={:?},names={:?}",name,from_utf8(project),from_utf8(names));
    let project=from_utf8(project).unwrap();
    let names=from_utf8(names).unwrap();
    let MODL=MODL{
        project: project.to_string(),
        name: names.to_string()
    };
    Ok((input,MODL))
}

fn parse_cntb_binary(input:&[u8])->IResult<&[u8],CNTB>{
    //println!("input={:?}",input);
    let (input,(_,_,_,_,str_size,))=tuple((
        take_while(is_zero),
        take_while(is_not_zero),
        parse_cnte_version,
        take_while(is_zero),
        le_u8,))(input)?;
    println!("str_size={}",str_size);
    let str_size:usize= (str_size * 4) as usize;
    let name=&input[..str_size];
    let input=&input[str_size..];
    let (input,(a,b,c,_,material))=tuple((
        be_f32,
        be_f32,
        be_f32,
        take_while(is_zero),
        le_u8,
        ))(input)?;
        let name=from_utf8(name).unwrap();
        Ok((input,CNTB{
            name:name.to_string(),
            translation: [a,b,c],
            material: material as f32,
        }))
    }

fn parse_cnte_binary(input:&[u8])->IResult<&[u8],CNTE>{
    let (input,(_,_,_))=tuple((
        take_while(is_zero),
        take_while(is_not_zero),
        parse_cnte_version,
        ))(input)?;
    Ok((input,CNTE))
}
fn parse_cnte_version(input:&[u8])->IResult<&[u8],u8>{
    let (input,(_,_,_,_))=tuple((
        take_while(is_zero),
        le_u8,
        take_while(is_zero),
        le_u8,
        ))(input)?;
    Ok((input,1))
}

fn parse_prim_binayy(input:&[u8])->IResult<&[u8],PRIM>{
    let (input,(_,_,_,_,types, M3x4,bboxLocal))=tuple((
        take_while(is_zero),
        take_while(is_not_zero),
        parse_cnte_version,
        take_while(is_zero),
        le_u8,
        parse_M3x4_binary,
        parse_bboxLocal_binary,
        ))(input)?;
    match types {
        1=>{
            let (input,pyramid)=parse_pyramid_binary(input)?;
            Ok((input,PRIM{
                kind: "Pyramid".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: (PyramidShape(pyramid))
            }))
        }
        2=>{
            let (input,boxs)=parse_box_binary(input)?;
            Ok((input,PRIM{
                kind: "Box".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: (BoxShape(boxs))
            }))
        }
        3=>{
            let (input,rectangularTorus)=parse_rectangularTorus_binary(input)?;
            Ok((input,PRIM{
                kind: "RectangularTorus".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: (RectangularTorusShape(rectangularTorus))
            }))
        }
        4=>{
            let (input,circularTorus)=parse_circularTorus_binary(input).unwrap();
            let data=PRIM{
                kind: "CircularTorus".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: CircularTorusShape(circularTorus),
            };
            Ok((input,data))
        }
        5=>{
            let (input,ellipticaldish)=parse_ellipticalDish_binary(input).unwrap();
            let data=PRIM{
                kind: "EllipticalDish".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: EllipticalDishShape(ellipticaldish),
            };
            Ok((input,data))
        }
        6=>{
            let (input,sphericalDish)=parse_sphericalDish_binary(input).unwrap();
            let data=PRIM{
                kind: "SphericalDish".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: SphericalDishShape(sphericalDish),
            };
            Ok((input,data))
        }
        7=>{
            let (input,snout)=parse_snout_binary(input).unwrap();
            let data=PRIM{
                kind: "Snout".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: SnoutShape(snout),
            };
            Ok((input,data))
        }
        8=>{
            let (input,cylinder)=parse_cylinder_binary(input).unwrap();
            let data=PRIM{
                kind: "Cylinder".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: CylinderShape(cylinder),
            };
            Ok((input,data))
        }
        9=>{
            let (input,sphere)=parse_sphere_binary(input).unwrap();
            let data=PRIM{
                kind: "Sphere".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: SphereShape(sphere),
            };
            Ok((input,data))
        }
        10=>{
            let (input,lineShape)=parse_line_binary(input).unwrap();
            let data=PRIM{
                kind: "Line".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: LineShape(lineShape),
            };
            Ok((input,data))
        }
        11=>{
            let arr=vec![];
            let (input,FacetGroup)=parse_facetgroup_binary(input,arr).unwrap();
            let data=PRIM{
                kind: "FacetGroup".to_string(),
                M_3x4: M3x4,
                bboxLocal,
                group: FacetGroupShape(FacetGroup),
            };
            Ok((input,data))
        }
        _ => Err(panic!("Problem opening the file")),
    }
}
fn parse_M3x4_binary(input:&[u8])->IResult<&[u8],Vec<f32>>{
    let (input,(a,b,c,d))=tuple((
        parse_3float_binary,
        parse_3float_binary,
        parse_3float_binary,
        parse_3float_binary,
        ))(input)?;
    Ok((input,vec![a,b,c,d].concat()))
}
fn parse_bboxLocal_binary(input:&[u8])->IResult<&[u8],[f32;6]>{
    let (input,(a,b,c,d,e,f))=tuple((
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        ))(input)?;
    Ok((input,[a,b,c,d,e,f]))
}
fn parse_3float_binary(input:&[u8])->IResult<&[u8],[f32;3]>{
    let (input,(a,b,c,))=tuple((
        be_f32,
        be_f32,
        be_f32,
        ))(input)?;
    Ok((input,[a,b,c]))
}

fn parse_pyramid_binary(input:&[u8])->IResult<&[u8],Pyramid>{
    let (input,(a,b,c,d,e,f,g))=tuple((
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        ))(input)?;
    Ok((input,Pyramid{
        bottom: [a,b],
        top: [c,d],
        offset: [e,f],
        height: g
    }))
}
fn parse_box_binary(input:&[u8])->IResult<&[u8],Box>{
    let (input,(x,y,z))=tuple((
        be_f32,
        be_f32,
        be_f32,
        ))(input)?;
    Ok((input,Box{
        lengths: [x,y,z]
    }))
}
fn parse_rectangularTorus_binary(input:&[u8])->IResult<&[u8],RectangularTorus>{
    let (input,(a,b,c,d))=tuple((
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        ))(input)?;
    Ok((input,RectangularTorus{
        inner_radius: a,
        outer_radius: b,
        height: c,
        angle: d
    }))
}
fn parse_circularTorus_binary(input:&[u8])->IResult<&[u8],CircularTorus>{
    let (input,(a,b,c))=tuple((
        be_f32,
        be_f32,
        be_f32,
        ))(input)?;
    Ok((input,CircularTorus{
        offset: a,
        radius: b,
        angle:  c
    }))
}
fn parse_ellipticalDish_binary(input:&[u8])->IResult<&[u8],EllipticalDish>{
    let (input,(x,y))=tuple((
        be_f32,
        be_f32,
        ))(input)?;
    Ok((input,EllipticalDish{
        baseRadius: x,
        height: y
    }))
}
fn parse_sphericalDish_binary(input:&[u8])->IResult<&[u8],SphericalDish>{
    let (input,(x,y))=tuple((
        be_f32,
        be_f32,
    ))(input)?;
    Ok((input,SphericalDish{
        baseRadius: x,
        height: y
    }))
}
fn parse_snout_binary(input:&[u8])->IResult<&[u8],Snout>{
    let (input,(a,b,c,d,e,f,g,h,i))=tuple((
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        be_f32,
        be_f32,
    ))(input)?;
    Ok((input,Snout{
        offset: [a,b],
        bshear: [c,d],
        tshear: [e,f],
        radius_b: g,
        radius_t: h,
        height: i
    }))
}
fn parse_cylinder_binary(input:&[u8])->IResult<&[u8],Cylinder>{
    let (input,(x,y))=tuple((
        be_f32,
        be_f32,
    ))(input)?;
    Ok((input,Cylinder{
        radius: x,
        height: y
    }))
}
fn parse_sphere_binary(input:&[u8])->IResult<&[u8],Sphere>{
    let (input,(x,))=tuple((
        be_f32,
    ))(input)?;
    Ok((input,Sphere{
        diameter: x
    }))
}
fn parse_line_binary(input:&[u8])->IResult<&[u8],Line>{
    let (input,(x,y))=tuple((
        be_f32,
        be_f32,
    ))(input)?;
    Ok((input,Line{
        a: x,
        b: y
    }))
}
fn parse_facetgroup_binary(mut input:&[u8], mut arr:Vec<[f32;6]>) ->IResult<&[u8],FacetGroup>{
    //println!("input={:?}",input);
    let (input_1,(_,mut times))=tuple((
        take_while(is_zero),
        be_u8,
        ))(input)?;
    let mut tep=0;
    input=input_1;
    println!("times={}",times);
    while tep<times {
        let (input_1,(_,in_times))=tuple((
            take_while(is_zero),
            be_u8,
            ))(input)?;
        input=input_1;
        println!("in_times={}",in_times);
        let mut in_tep=0;
        while in_tep<in_times{
            let (input_1,(_,tuple_times))=tuple((
                take_while(is_zero),
                be_u8,
                ))(input)?;
        input=input_1;
        let mut tuple_time=0;
        while tuple_time<tuple_times{
            let (input_1,(x,))=tuple((
                parse_bboxLocal_binary,
                ))(input)?;
            input=input_1;
            arr.push(x);
            tuple_time+=1;
        }
        in_tep+=1;
    }
    tep+=1;
}
    Ok((input,FacetGroup{
        data:arr,
    }))
}
fn parse_string(input:&[u8])->IResult<&[u8],&[u8]>{
    let mut sum=0;
    let data=input.clone();
    let (input,(result,))=tuple((
        take_while(is_not_zero),
        ))(data)?;
    let mut lentgth =result.len();

    while lentgth%4!=0{
        lentgth+=1;
        sum+=1;
    }
    println!("sum={}",sum);
    Ok((&input[sum..],result))
}
fn take_zero_out(input:[u8;4], mut arr:Vec<u8>) ->Vec<u8>{
    for val in input.iter(){
        if *val!=0{
            arr.push(*val);
        }
    };
    arr
}

pub fn is_zero(chr: u8) -> bool {
    chr==0
}
pub fn is_not_zero(chr:u8)->bool{
    chr!=0
}
fn parse_f32(input:&[u8])->IResult<&[u8],f32>{
    let (input,(out,))=tuple((
        be_f32,
        ))(input)?;
    Ok((input,out))
}
#[test]
fn test_parse_f32(){
    //let input=&[68u8,122,0,0][..];
    let input=&[2,0,0,0];
    let (input,out)=parse_f32(input).unwrap();

    println!("out={:?}",out);
}
#[test]
fn test_str(){
    //let data=&[47,49,48,48,45,66,45,49,0,0][..];
    let data=&[47, 83, 84, 65, 66, 73, 76, 73, 90, 69, 82,0][..];
    let (input,out)=parse_string(data).unwrap();
    println!("out={:?}",out);
    println!("input={:?}",input);
    println!("out={:?}",from_utf8(out));
}