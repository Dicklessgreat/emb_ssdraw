//! A simple screen saver for the display
//!
//! This is a simple screen saver that draws a random number of points on the display.
//!
//! # Usage
//!
//! Cargo.toml  
//! ```
//! [dependencies]
//! rand = { version = "0.8.5", features = ["small_rng"], default-features = false }
//! ```
//!
//! src/main.rs
//! ```
//! use rand::rngs::SmallRng;
//! use rand::SeedableRng;
//! 
//! let mut rng = SmallRng::seed_from_u64(42);
//! let mut screen_saver = ScreenSaver::<_, 32>::new(rng);
//! let mut display = Display::new();
//! loop {
//!     display.clear();
//!     screen_saver.tick_draw(&mut display);
//!     display.flush();
//! }
//! 
//! ```

#![no_std]
use heapless::Vec;
use rand::RngCore;
use embedded_graphics::{
    prelude::*,
    pixelcolor::BinaryColor,
    Drawable,
};

/// A screen saver that draws a random number of points on the display
pub struct ScreenSaver<R, const N: usize> {
    points: Vec<Point, N>,
    rng: R,
}
impl<R:RngCore, const N: usize> ScreenSaver<R, N> {
    /// Create a new screen saver
    pub fn new(rng: R) -> Self {
        Self {
            points: Vec::new(),
            rng,
        }
    }
    /// Tick the screen saver and draw it to the target
    /// this is alias for calling [`tick`](ScreenSaver::tick) with target size and after [`draw`](ScreenSaver::draw) with the target
    pub fn tick_draw<D>(&mut self, target: &mut D) -> Result<(), D::Error> 
    where D: DrawTarget<Color = BinaryColor> 
    {
        let size = target.bounding_box().size;
        self.tick(size.width, size.height);
        self.draw(target)
    }

    /// Tick the screen saver and add a new point to the list
    /// If the inner points buffer is full, the oldest point will be removed
    /// panics if either w or h are greater than i32::max().
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