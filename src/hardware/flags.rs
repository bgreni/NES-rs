

pub struct Flags {
    pub carry: bool,
    pub zero: bool,
    pub inter_disable: bool,
    pub decimal: bool,
    pub break1: bool,
    pub break2: bool,
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
            break1: false,
            break2: false,
            overflow: false,
            negative: false,
        }
    }

    pub fn to_u8(&self) -> u8 {
        let mut res = 0;

        let flags = [
            self.negative,
            self.overflow,
            self.break1,
            self.break2,
            self.decimal,
            self.inter_disable,
            self.zero,
            self.carry
        ];

        for (ind, flag) in flags.iter().enumerate() {
            if *flag {
                res |= 1 << (7 - ind);
            }
        }

        return res as u8;
    }

    pub fn set_breaks(&mut self) {
        self.break1 = true;
        self.break2 = true;
    }

    pub fn reset_breaks(&mut self) {
        self.break1 = false;
        self.break2 = false;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u81() {
        let flags = Flags {
            carry: false,
            zero: false,
            inter_disable: false,
            decimal: false,
            break1: true,
            break2: true,
            overflow: true,
            negative: true,
        };
        assert_eq!(flags.to_u8(), 0xF0);
    }

    #[test]
    fn test_to_u82() {
        let flags = Flags {
            carry: true,
            zero: false,
            inter_disable: false,
            decimal: false,
            break1: true,
            break2: true,
            overflow: true,
            negative: true,
        };
        assert_eq!(flags.to_u8(), 0xF1);
    }
}