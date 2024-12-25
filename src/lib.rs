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
    pub fn tick(&mut self, w: u32, h: u32) {
        let x = (self.rng.next_u32() % w) as i32;
        let y = (self.rng.next_u32() % h) as i32;
        if let Err(overflowed) = self.points.push(Point::new(x, y)) {
            self.points.remove(0);
            let _ = self.points.push(overflowed);
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