mod crank;

impl crate::Playdate {
    pub fn crank(&self) -> crank::Crank {
        crank::Crank::new()
    }
}