#![no_std]
use heapless::Vec;
use rand::RngCore;
use embedded_graphics::{
    prelude::*,
    pixelcolor::BinaryColor,
    Drawable,
};

pub struct ScreenSaver<R, const N: usize> {
    points: Vec<Point, N>,
    rng: R,
}
impl<R:RngCore, const N: usize> ScreenSaver<R, N> {
    pub fn new(rng: R) -> Self {
        Self {
            points: Vec::new(),
            rng,
        }
    }
    pub fn tick(&mut self) {
        if let Err(e) = self.points.push(Point {
            x: (self.rng.next_u32() / 33554432) as i32,
            y: (self.rng.next_u32() / 67108864) as i32,
        }) {
            self.points.remove(0);
            let _ = self.points.push(e);
        }
    }
}

impl<R:RngCore, const N: usize> Drawable for ScreenSaver<R, N> {
    type Color = BinaryColor;
    type Output = ();
    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {

        for p in self.points.iter() {
            let _ = Pixel(*p, BinaryColor::On).draw(target);
        }
        Ok(())
    }
}