#[derive(Debug, Default, Clone, Copy)]
pub struct RegisterState {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    
    pub c: bool,
    pub z: bool,
    pub i: bool,
    pub d: bool,
    pub o: bool,
    pub n: bool,
}

impl RegisterState {
    pub fn update_a(&mut self, value: u8) {
        self.a = value;
        self.update_nz_flags(self.a);
    }

    pub fn update_x(&mut self, value: u8) {
        self.x = value;
        self.update_nz_flags(self.x);
    }

    pub fn update_y(&mut self, value: u8) {
        self.y = value;
        self.update_nz_flags(self.y);
    }

    pub fn update_nz_flags(&mut self, value: u8) {
        self.n = (value as i8).is_negative();
        self.z = value == 0;
    }
}