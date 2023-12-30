use crate::math::{vec::Vec2i, Vec2f, Vec3f};

use self::obj::{Face, ObjData};

pub mod obj;
// pub mod tga;

pub trait ImgColor:
    std::fmt::Debug
    + Clone
    + From<(u8, u8, u8)>
    + From<(u8, u8, u8, u8)>
    + Into<(u8, u8, u8, u8)>
    + From<(f64, f64, f64)>
    + From<(f64, f64, f64, f64)>
    + Into<(f64, f64, f64, f64)>
{
    fn random() -> Self;
}

pub trait Img: Sized {
    type Color: ImgColor;
    type Err;

    fn set_px(&mut self, pos: Vec2i, color: Self::Color) -> Result<(), Self::Err>;

    fn line(&mut self, start: Vec2i, end: Vec2i, color: Self::Color) -> Result<(), Self::Err> {
        crate::renderer::line(self, start, end, color)
    }

    fn tri(
        &mut self,
        tri: [Vec3f; 3],
        color: Self::Color,
        zbuffer: Option<&mut [f64]>,
    ) -> Result<(), Self::Err> {
        crate::renderer::tri(self, tri, color, zbuffer)
    }

    fn size(&self) -> Result<Vec2i, Self::Err>;
}

pub fn line<I: Img>(c: &mut I, mut s: Vec2i, mut e: Vec2i, color: I::Color) -> Result<(), I::Err> {
    let mut steep = false;

    if i32::abs(s.x() - e.x()) < i32::abs(s.y() - e.y()) {
        (s[0], s[1]) = (s.y(), s.x());
        (e[0], e[1]) = (e.y(), e.x());
        steep = true;
    }

    if s.x() > e.x() {
        (s[0], e[0]) = (e.x(), s.x());
        (s[1], e[1]) = (e.y(), s.y());
    }

    let dx = e.x() - s.x();
    let dy = e.y() - s.y();
    let derror2 = i32::abs(dy) * 2;

    let mut error2 = 0;
    let mut y = s.y();

    for x in s.x()..=e.x() {
        if steep {
            c.set_px(Vec2i::new([y, x]), color.clone())?;
        } else {
            c.set_px(Vec2i::new([x, y]), color.clone())?;
        }
        error2 += derror2;
        if error2 > dx {
            y += if e.y() > s.y() { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
    Ok(())
}

pub fn flat_shaded<I: Img>(
    img: &mut I,
    size: Vec2i,
    offs: Vec3f,
    o: &ObjData,
    light_dir: Vec3f,
    color: I::Color,
    mut zbuffer: Option<&mut [f64]>,
) -> Result<(), I::Err> {
    let (cw, ch) = (size.width() as f64 * 0.5, size.height() as f64 * 0.5);

    for f in &o.faces {
        let Face::Tri(inds) = f else { todo!() };
        let wc = inds.map(|i| o.vertices[i.v as usize - 1].div_w());

        let sc = std::array::from_fn(|i| {
            Vec3f::new([((wc[i].x() + 1.) * cw), ((wc[i].y() + 1.) * ch), 0.]) + offs
        });

        let normal = (wc[2] - wc[0]).cross(wc[1] - wc[0]).normalized();

        let int = normal.dot(light_dir);
        if int > 0. {
            let (r, g, b, a): (f64, f64, f64, f64) = color.clone().into();

            img.tri(
                sc,
                (int * r, int * g, int * b, a).into(),
                zbuffer.as_deref_mut(),
            )?;
        }
    }

    Ok(())
}

pub fn wireframe<I: Img>(
    img: &mut I,
    size: Vec2i,
    offs: Vec2i,
    o: &ObjData,
    color: I::Color,
) -> Result<(), I::Err> {
    let (cw, ch) = (size.width() as f64 * 0.5, size.height() as f64 * 0.5);

    for f in &o.faces {
        let Face::Tri(inds) = f else { todo!() };

        for i in 0..3 {
            let v0 = inds[i].v;
            let v1 = inds[(i + 1) % 3].v;

            let v0 = o.vertices[v0 as usize - 1];
            let v1 = o.vertices[v1 as usize - 1];

            let s = Vec2i::new([
                ((v0.x() + 1.) * cw).round() as i32,
                ((v0.y() + 1.) * ch).round() as i32,
            ]);

            let e = Vec2i::new([
                ((v1.x() + 1.) * cw).round() as i32,
                ((v1.y() + 1.) * ch).round() as i32,
            ]);

            img.line(offs + s, offs + e, color.clone())?;
        }
    }

    Ok(())
}

fn barycentric([a, b, c]: [Vec3f; 3], p: Vec3f) -> Option<Vec3f> {
    let s: [Vec3f; 2] =
        std::array::from_fn(|i| Vec3f::new([c[i] - a[i], b[i] - a[i], a[i] - p[i]]));

    let u = s[0].cross(s[1]);
    if f64::abs(u.z()) > 1e-2 {
        // dont forget that u.z() is integer. If it is zero then triangle ABC is degenerate
        return Some(Vec3f::new([
            1. - (u.x() + u.y()) / u.z(),
            u.y() / u.z(),
            u.x() / u.z(),
        ]));
    }

    // in this case generate negative coordinates, it will be thrown away by the rasterizator
    None
}

fn tri<I: Img>(
    img: &mut I,
    tri: [Vec3f; 3],
    color: I::Color,
    mut zbuffer: Option<&mut [f64]>,
) -> Result<(), I::Err> {
    let size = img.size()?;

    let clamp = Vec2f::new([size.width() as f64 - 1., size.height() as f64 - 1.]);

    let mut bmin = Vec2f::new([f64::MAX, f64::MAX]);
    let mut bmax = Vec2f::new([f64::MIN, f64::MIN]);

    for t in tri {
        for i in 0..2 {
            bmin[i] = f64::max(0., f64::min(bmin[i], t[i]));
            bmax[i] = f64::min(clamp[i], f64::max(bmax[i], t[i]));
        }
    }

    for x in (bmin.x() as i32)..=(bmax.x() as i32) {
        for y in (bmin.y() as i32)..=(bmax.y() as i32) {
            let Some(bc_screen) = barycentric(tri, Vec3f::new([x as f64, y as f64, 0.])) else {
                continue;
            };
            if bc_screen.x() < 0. || bc_screen.y() < 0. || bc_screen.z() < 0. {
                continue;
            }

            let p = Vec2i::new([x, y]);
            if let Some(zbuffer) = zbuffer.as_deref_mut() {
                let mut z = 0.;
                for i in 0..3 {
                    z += tri[i].z() * bc_screen[i];
                }

                let zpos = (p.x() + p.y() * size.width()) as usize;
                if zbuffer[zpos] < z {
                    zbuffer[zpos] = z;
                    img.set_px(p, color.clone())?;
                }
            } else {
                img.set_px(p, color.clone())?;
            }
        }
    }

    Ok(())
}
