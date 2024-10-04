/// 原作の乱数生成器。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GameRng {
    state: u16,
}

impl GameRng {
    /// 指定した内部状態を持つ乱数生成器を作る。
    pub fn new(state: u16) -> Self {
        Self { state }
    }

    /// 内部状態を返す。
    pub fn state(self) -> u16 {
        self.state
    }

    /// `0..=255` の乱数を生成する。内部状態は 1 回更新される。
    pub fn gen(&mut self) -> u8 {
        self.state = self.state.wrapping_mul(257).wrapping_add(1);

        u8::try_from(self.state >> 8).unwrap()
    }

    /// `0..end` の乱数を生成する (`end` が 0 の場合、0 を返す)。内部状態は常に 1 回更新される。
    pub fn gen_range(&mut self, end: u8) -> u8 {
        let r = self.gen();

        let res = (u16::from(end) * u16::from(r)) >> 8;
        u8::try_from(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let mut rng = GameRng::new(0xA5A4);

        assert_eq!(rng.gen(), 0x49);
        assert_eq!(rng.state(), 0x49A5);

        assert_eq!(rng.gen(), 0xEE);
        assert_eq!(rng.state(), 0xEEA6);

        assert_eq!(rng.gen(), 0x94);
        assert_eq!(rng.state(), 0x94A7);
    }

    #[test]
    fn test_gen_range() {
        let mut rng = GameRng::new(0xA7DB);

        assert_eq!(rng.gen_range(7), 3);
        assert_eq!(rng.state(), 0x82DC);

        assert_eq!(rng.gen_range(7), 2);
        assert_eq!(rng.state(), 0x5EDD);

        assert_eq!(rng.gen_range(7), 1);
        assert_eq!(rng.state(), 0x3BDE);
    }
}
