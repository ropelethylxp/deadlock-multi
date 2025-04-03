pub struct Features {
    pub aimbot: bool,
    pub esp: bool,
    pub speed_hack: bool,
    pub no_recoil: bool,
}

impl Features {
    pub fn new() -> Self {
        Features {
            aimbot: false,
            esp: false,
            speed_hack: false,
            no_recoil: false,
        }
    }

    pub fn toggle_aimbot(&mut self) {
        self.aimbot = !self.aimbot;
    }

    pub fn toggle_esp(&mut self) {
        self.esp = !self.esp;
    }

    pub fn toggle_speed_hack(&mut self) {
        self.speed_hack = !self.speed_hack;
    }

    pub fn toggle_no_recoil(&mut self) {
        self.no_recoil = !self.no_recoil;
    }
}