mod crank;
mod buttons;

impl crate::Playdate {
    pub fn crank(&self) -> crank::Crank {
        crank::Crank::new()
    }
    pub fn dpad(&self) -> buttons::DPad {
        buttons::DPad::new()
    }
}