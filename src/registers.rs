#[derive(Debug, Default, Clone, Copy)]
pub struct RegisterState {
    a: u8,
    x: u8,
    y: u8,
    
    pub c: bool,
    pub z: bool,
    pub i: bool,
    pub d: bool,
    pub v: bool,
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

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn update_nz_flags(&mut self, value: u8) {
        self.n = (value as i8).is_negative();
        self.z = value == 0;
    }

    pub fn get_status(&self, brk: bool) -> u8 {
        let b = |flag, shift| (flag as u8) << shift;
        b(self.c, 0) |
        b(self.z, 1) |
        b(self.i, 2) |
        b(self.d, 3) |
        b(brk,    4) |
        b(true,   5) |
        b(self.v, 6) |
        b(self.n, 7)
    }

    pub fn set_status(&mut self, value: u8) {
        let b = |shift: u8| value & (1 << shift) != 0;
        self.c = b(0);
        self.z = b(1);
        self.i = b(2);
        self.d = b(3);
        self.v = b(6);
        self.n = b(7);
    }
}