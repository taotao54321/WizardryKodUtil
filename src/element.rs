use bitflags::bitflags;

bitflags! {
    /// 属性マスク。
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Elements: u8 {
        /// 友好。
        const FRIENDLY = 1 << 0;
        /// 火。
        const FIRE = 1 << 1;
        /// 冷気。
        const COLD = 1 << 2;
        /// 毒。
        const POISON = 1 << 3;
        /// ドレイン。
        const DRAIN = 1 << 4;
        /// 石化。
        const PETRIFY = 1 << 5;
        /// 呪文。
        const SPELL = 1 << 6;
        /// 欠番 7。
        const UNUSED_7 = 1 << 7;
    }
}
