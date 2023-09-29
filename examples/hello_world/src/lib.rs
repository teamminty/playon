#![no_std]
use core::time::Duration;

use playon::prelude::*;

pub struct App {

}

#[pd]
impl Game for App {
    fn new() -> Self {
        Self {  }
    }
    fn update(&mut self, pd: Playdate, dt: Duration) {
        println!("{}", pd.crank().rotation().to_degrees())
    }
}