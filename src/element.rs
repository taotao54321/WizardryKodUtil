use flagset::{flags, FlagSet};

flags! {
    /// 攻撃属性。
    #[repr(u8)]
    pub enum Element: u8 {
        /// 友好。
        Friendly = 1 << 0,
        /// 火。
        Fire = 1 << 1,
        /// 冷気。
        Cold = 1 << 2,
        /// 毒。
        Poison = 1 << 3,
        /// 吸収。
        Drain = 1 << 4,
        /// 石化。
        Petrify = 1 << 5,
        /// 呪文。
        Spell = 1 << 6,
        /// (未使用)
        Unused7 = 1 << 7,
    }
}

impl Element {
    /// 正式名称を返す。
    pub fn name(self) -> &'static str {
        match self {
            Self::Friendly => "友好",
            Self::Fire => "火",
            Self::Cold => "冷気",
            Self::Poison => "毒",
            Self::Drain => "吸収",
            Self::Petrify => "石化",
            Self::Spell => "呪文",
            Self::Unused7 => "(未使用7)",
        }
    }

    /// 略称を返す。
    pub fn name_abbrev(self) -> &'static str {
        match self {
            Self::Friendly => "友",
            Self::Fire => "火",
            Self::Cold => "冷",
            Self::Poison => "毒",
            Self::Drain => "吸",
            Self::Petrify => "石",
            Self::Spell => "呪",
            Self::Unused7 => "謎",
        }
    }

    /// 全ての攻撃属性を昇順で返す。
    pub fn iter(
    ) -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator + std::iter::FusedIterator + Clone
    {
        [
            Self::Friendly,
            Self::Fire,
            Self::Cold,
            Self::Poison,
            Self::Drain,
            Self::Petrify,
            Self::Spell,
            Self::Unused7,
        ]
        .into_iter()
    }
}

/// 攻撃属性マスク。
pub type Elements = FlagSet<Element>;

/// 攻撃属性マスクをフォーマットし、正式名称のカンマ区切り文字列にする。
#[derive(Debug)]
pub struct ElementsDisplay(Elements);

impl ElementsDisplay {
    pub fn new(elements: Elements) -> Self {
        Self(elements)
    }
}

impl std::fmt::Display for ElementsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for element in Element::iter() {
            if self.0.contains(element) {
                if !first {
                    f.write_str(",")?;
                }
                f.write_str(element.name())?;
                first = false;
            }
        }

        Ok(())
    }
}

/// 攻撃属性マスクをフォーマットし、略称を繋げた文字列にする。
#[derive(Debug)]
pub struct ElementsDisplayAbbrev(Elements);

impl ElementsDisplayAbbrev {
    pub fn new(elements: Elements) -> Self {
        Self(elements)
    }
}

impl std::fmt::Display for ElementsDisplayAbbrev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for kind in Element::iter() {
            if self.0.contains(kind) {
                f.write_str(kind.name_abbrev())?;
            }
        }

        Ok(())
    }
}
