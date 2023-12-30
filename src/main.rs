use math::{Vec2i, Vec3f};
use random::Source;
use renderer::{obj::ObjData, Img, ImgColor};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};
use std::{io::BufRead, time::Duration};

pub mod hsv;
pub mod math;

pub mod renderer;
pub mod robot;

pub(crate) type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

struct Model {
    obj: ObjData,
    texture: image::ImageBuffer<image::Rgba<f32>, Vec<f32>>,
}

fn main() -> Result<(), String> {
    let head = Model {
        obj: renderer::obj::read(
            std::io::BufReader::new(
                std::fs::File::open("african_head.obj").map_err(|e| e.to_string())?,
            )
            .lines(),
        )
        .map_err(|e| e.to_string())?,

        texture: image::open("african_head_diffuse.tga")
            .map_err(|e| e.to_string())?
            .into_rgba32f(),
    };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("3D test", 800, 600)
        .resizable()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut zbuffer = vec![0.; 1920 * 1080];
    'running: loop {
        canvas.set_draw_color(Color::RGB(75, 75, 75));
        canvas.clear();

        render(&mut canvas, &head, Some(&mut zbuffer))?;
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::Q),
                    ..
                } => break 'running,

                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }

    Ok(())
}

impl Img for Canvas {
    type Color = MyColor;
    type Err = String;

    fn set_px(&mut self, pos: Vec2i, color: Self::Color) -> Result<(), Self::Err> {
        self.set_draw_color(color.0);
        self.draw_point(Point::new(pos.x(), pos.y()))
    }

    fn line(&mut self, start: Vec2i, end: Vec2i, color: Self::Color) -> Result<(), Self::Err> {
        self.set_draw_color(color.0);
        self.draw_line(
            Point::new(start.x(), start.y()),
            Point::new(end.x(), end.y()),
        )
    }

    fn size(&self) -> Result<Vec2i, Self::Err> {
        let (w, h) = self.output_size().map(|(w, h)| (w as i32, h as i32))?;
        Ok(Vec2i::new([w, h]))
    }
}

fn render(
    canvas: &mut Canvas,
    model: &Model,
    mut zbuffer: Option<&mut [f64]>,
) -> Result<(), String> {
    if let Some(zbuffer) = zbuffer.as_deref_mut() {
        zbuffer.fill(f64::MIN);
    }

    renderer::flat_shaded(
        canvas,
        canvas.size()?,
        Vec3f::zero(),
        model,
        Vec3f::new([0., 0., -1.]),
        MyColor(Color::MAGENTA),
        zbuffer,
    )?;

    // println!("draw");

    Ok(())
}

#[derive(Debug, Clone)]
pub struct MyColor(Color);
impl From<(u8, u8, u8)> for MyColor {
    fn from(value: (u8, u8, u8)) -> Self {
        Self(value.into())
    }
}
impl From<(u8, u8, u8, u8)> for MyColor {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self(value.into())
    }
}
impl From<MyColor> for (u8, u8, u8, u8) {
    fn from(val: MyColor) -> Self {
        (val.0.r, val.0.g, val.0.b, val.0.a)
    }
}
impl From<(f64, f64, f64)> for MyColor {
    fn from(value: (f64, f64, f64)) -> Self {
        Self(Color::RGB(
            (value.0 * 255.).round() as u8,
            (value.1 * 255.).round() as u8,
            (value.2 * 255.).round() as u8,
        ))
    }
}
impl From<(f64, f64, f64, f64)> for MyColor {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self(Color::RGBA(
            (value.0 * 255.).round() as u8,
            (value.1 * 255.).round() as u8,
            (value.2 * 255.).round() as u8,
            (value.3 * 255.).round() as u8,
        ))
    }
}
impl From<MyColor> for (f64, f64, f64, f64) {
    fn from(val: MyColor) -> Self {
        (
            val.0.r as f64 / 255.,
            val.0.g as f64 / 255.,
            val.0.b as f64 / 255.,
            val.0.a as f64 / 255.,
        )
    }
}

impl ImgColor for MyColor {
    fn random() -> Self {
        let mut r = random::default(std::time::Instant::now().elapsed().as_nanos() as u64);

        Self(Color::RGB(
            (r.read_u64() % 0xff) as u8,
            (r.read_u64() % 0xff) as u8,
            (r.read_u64() % 0xff) as u8,
        ))
    }
}
