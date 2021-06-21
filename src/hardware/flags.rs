

pub struct Flags {
    pub carry: bool,
    pub zero: bool,
    pub inter_disable: bool,
    pub decimal: bool,
    pub breakf: bool,
    pub overflow: bool,
    pub negative: bool
}

impl Flags {
    pub fn new() -> Self {
        return Flags {
            carry: false,
            zero: false,
            inter_disable: false,
            decimal: false,
            breakf: false,
            overflow: false,
            negative: false,
        }
    }
}