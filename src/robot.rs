use std::f64::consts::PI;

use sdl2::{gfx::primitives::DrawRenderer, pixels::Color};

use crate::{
    hsv,
    math::{vec::Vecf, Vec2f},
};

pub struct RobotLeg {
    pub base_rot: f64,
    pub knee_rot: f64,

    pub upper_len: f64,
    pub lower_len: f64,
}

impl RobotLeg {
    pub fn new(base_rot: f64, upper_len: f64, knee_rot: f64, lower_len: f64) -> Self {
        Self {
            base_rot,
            upper_len,

            knee_rot,
            lower_len,
        }
    }

    pub fn knee_pos(&self) -> Vec2f {
        Vec2f::new([
            self.upper_len * self.base_rot.cos(),
            self.upper_len * self.base_rot.sin(),
        ])
    }

    pub fn joint_positions(&self) -> (Vec2f, Vec2f) {
        let knee_rot = self.base_rot + self.knee_rot;
        let knee = self.knee_pos();

        (
            knee,
            knee + Vec2f::new([
                self.lower_len * knee_rot.cos(),
                self.lower_len * knee_rot.sin(),
            ]),
        )
    }

    pub fn min_len(&self) -> f64 {
        f64::abs(self.upper_len - self.lower_len)
    }

    pub fn max_len(&self) -> f64 {
        self.upper_len + self.lower_len
    }

    pub fn place_end_ik(&mut self, end: Vec2f) -> bool {
        let e_len = end.len();
        if e_len == 0. || e_len > self.max_len() || e_len < self.min_len() {
            return false;
        }

        let rot = f64::atan2(end.y(), end.x());

        let a = self.lower_len;
        let b = self.upper_len;
        let c = e_len;

        // law of cosines
        let bx = (b * b + c * c - a * a) / (2. * b * c);
        let base = f64::acos(bx);

        // law of sines
        let kx = (c * f64::sin(base)) / a;
        let mut knee = f64::asin(kx);

        if a * a + b * b > c * c {
            knee = PI - knee;
        }

        self.base_rot = base + rot;
        self.knee_rot = -knee;

        // println!("base: {base:.2} knee: {knee:.2}");

        true
    }
}

pub fn draw_leg(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    leg: &RobotLeg,
    at: Vec2f,
    (ox, oy): (i16, i16),
) -> Result<(), String> {
    const LEG_COLOR: Color = Color::WHITE;
    const LEG_THICKNESS: u8 = 3;
    const JOINT_RADIUS: i16 = 8;

    let (knee_pos, end_pos) = leg.joint_positions();

    let (bx, by) = (Vecf::zero() + at).offset((ox, oy));
    let (kx, ky) = (knee_pos + at).offset((ox, oy));
    let (ex, ey) = (end_pos + at).offset((ox, oy));

    // guides
    canvas.circle(bx, by, leg.max_len().round() as i16, Color::RED)?;

    let x = f64::sqrt(leg.upper_len * leg.upper_len + leg.lower_len * leg.lower_len);
    canvas.circle(bx, by, x.round() as i16, Color::RED)?;
    canvas.circle(bx, by, leg.min_len().round() as i16, Color::RED)?;
    // END guides

    canvas.thick_line(bx, by, kx, ky, LEG_THICKNESS, LEG_COLOR)?;
    canvas.thick_line(kx, ky, ex, ey, LEG_THICKNESS, LEG_COLOR)?;

    canvas.filled_circle(bx, by, JOINT_RADIUS, joint_color(leg.base_rot))?;
    canvas.filled_circle(kx, ky, JOINT_RADIUS, joint_color(leg.knee_rot))?;

    Ok(())
}

pub fn joint_color(j: f64) -> Color {
    let j = j.abs() / std::f64::consts::FRAC_PI_2 - 1.;
    let j = (j.signum() - j).abs();
    hsv::hsv_to_rgb(j * 120., 0.6, 1.).into()
}

// pub fn place_end_ik(&mut self, end: Vec2f) -> bool {
//     let e_len = end.len();
//     if e_len == 0. || e_len > self.max_len() || e_len < self.min_len() {
//         return false;
//     }

//     let rot = f64::atan2(end.y(), end.x());

//     let a = self.lower_len;
//     let b = self.upper_len;
//     let c = e_len;

//     // law of cosines
//     let bx = (b * b + c * c - a * a) / (2. * b * c);
//     let base = f64::acos(bx);

//     // law of sines
//     let kx = (c * f64::sin(base)) / a;
//     let mut knee = f64::asin(kx);

//     if a * a + b * b > c * c {
//         knee = PI - knee;
//     }

//     self.base_rot = base + rot;
//     self.knee_rot = -knee;

//     // println!("base: {base:.2} knee: {knee:.2}");

//     true
// }
