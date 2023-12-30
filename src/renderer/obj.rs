use crate::math::{Vec3f, Vec4f};
use std::io::{self, Lines};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ObjParseError {
    #[error("Missing or malformed float value")]
    BadFloat,

    #[error("Missing or malformed index")]
    BadIndex,

    #[error("{0} vertex components provided instead of {1}")]
    BadVertexComponentCount(u8, u8),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct ObjData {
    pub vertices: Vec<Vec4f>,
    pub texture_coordinates: Vec<Vec3f>,
    pub vertex_normals: Vec<Vec3f>,
    pub param_vertices: Vec<Vec3f>,

    pub faces: Vec<Face>,
    pub lines: Vec<Line>,
}

pub type Idx = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct FaceInd {
    pub v: Idx,
    pub vt: Option<Idx>,
    pub vn: Option<Idx>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Face {
    Tri([FaceInd; 3]),
    Poly(Vec<FaceInd>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Seg { start: Idx, end: Idx },
    Multi(Vec<Idx>),
}

pub fn read(r: Lines<impl io::BufRead>) -> Result<ObjData, ObjParseError> {
    let mut d = ObjData {
        vertices: vec![],
        texture_coordinates: vec![],
        vertex_normals: vec![],
        param_vertices: vec![],
        faces: vec![],
        lines: vec![],
    };

    for l in r {
        let l = l?;
        let mut l = l.trim_start();

        if let Some(h) = l.find('#') {
            l = &l[..h];
        }
        l = l.trim_end();

        let mut a = l.split_whitespace();
        match a.next() {
            Some("v") => d.vertices.push(get_vertex(a)?),
            Some("vt") => d.texture_coordinates.push(get_texture_coordinate(a)?),
            Some("vn") => d.vertex_normals.push(get_vertex_normal(a)?),
            Some("vp") => d.param_vertices.push(get_param_vertex(a)?),

            Some("f") => d.faces.push(get_face(a)?),
            Some("l") => d.lines.push(get_line(a)?),

            // TODO: stuffe
            Some("g" | "s") => (),

            Some(_prefix) => {
                //
                // panic!("Unsupported prefix: '{_prefix}'")
            }

            None => (),
        }
    }

    Ok(d)
}

fn get_vertex(a: std::str::SplitWhitespace<'_>) -> Result<Vec4f, ObjParseError> {
    let mut arr = [0., 0., 0., 1.];
    let mut i = 0;

    for v in a.take(4) {
        arr[i] = v.parse().map_err(|_| ObjParseError::BadFloat)?;
        i += 1;
    }

    if i < 3 {
        return Err(ObjParseError::BadVertexComponentCount(i as u8, 3));
    }

    Ok(Vec4f::new(arr))
}

fn get_face(a: std::str::SplitWhitespace<'_>) -> Result<Face, ObjParseError> {
    let mut fs = vec![];

    for ss in a {
        let mut s = ss.split('/');

        let v = s
            .next()
            .ok_or(ObjParseError::BadIndex)?
            .parse()
            .map_err(|_| ObjParseError::BadIndex)?;

        let vt = if let Some(vt) = s.next() {
            Some(vt.parse().map_err(|_| ObjParseError::BadIndex)?)
        } else {
            None
        };
        let mut vn = if let Some(vn) = s.next() {
            Some(vn.parse().map_err(|_| ObjParseError::BadIndex)?)
        } else {
            None
        };

        if vt.is_some() && vn.is_none() && ss.contains("//") {
            vn = vt;
        }

        fs.push(FaceInd { v, vt, vn });
    }

    let f = if fs.len() == 3 {
        Face::Tri([fs[0], fs[1], fs[2]])
    } else {
        Face::Poly(fs)
    };

    Ok(f)
}

fn get_texture_coordinate(a: std::str::SplitWhitespace<'_>) -> Result<Vec3f, ObjParseError> {
    let mut arr = [0., 0., 0.];
    let mut i = 0;

    for v in a.take(3) {
        arr[i] = v.parse().map_err(|_| ObjParseError::BadFloat)?;
        i += 1;
    }

    if i == 0 {
        return Err(ObjParseError::BadVertexComponentCount(i as u8, 1));
    }

    Ok(Vec3f::new(arr))
}
fn get_vertex_normal(a: std::str::SplitWhitespace<'_>) -> Result<Vec3f, ObjParseError> {
    let mut arr = [0., 0., 0.];
    let mut i = 0;

    for v in a.take(3) {
        arr[i] = v.parse().map_err(|_| ObjParseError::BadFloat)?;
        i += 1;
    }

    if i != 3 {
        return Err(ObjParseError::BadVertexComponentCount(i as u8, 3));
    }

    Ok(Vec3f::new(arr))
}

fn get_param_vertex(a: std::str::SplitWhitespace<'_>) -> Result<Vec3f, ObjParseError> {
    // they are the same right...?
    get_texture_coordinate(a)
}

fn get_line(a: std::str::SplitWhitespace<'_>) -> Result<Line, ObjParseError> {
    let mut v = vec![];
    for s in a {
        v.push(s.parse().map_err(|_| ObjParseError::BadIndex)?);
    }

    Ok(if v.len() == 2 {
        Line::Seg {
            start: v[0],
            end: v[1],
        }
    } else {
        Line::Multi(v)
    })
}
