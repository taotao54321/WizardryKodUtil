use flagset::{flags, FlagSet};
use num_enum::TryFromPrimitive;

flags! {
    /// 職業。
    #[repr(u8)]
    #[derive(TryFromPrimitive)]
    pub enum Class: u8 {
        Fighter = 1 << 0,
        Mage = 1 << 1,
        Cleric = 1 << 2,
        Thief = 1 << 3,
        Wizard = 1 << 4,
        Samurai = 1 << 5,
        Lord = 1 << 6,
        Ninja = 1 << 7,
    }
}

impl Class {
    /// 職業ID (`0..=7`) から職業を作る。
    pub fn from_id(id: u8) -> Option<Self> {
        (id <= 7).then(|| Self::try_from(1 << id).unwrap())
    }

    /// 職業ID (`0..=7`) を返す。
    pub fn to_id(self) -> u8 {
        match self {
            Self::Fighter => 0,
            Self::Mage => 1,
            Self::Cleric => 2,
            Self::Thief => 3,
            Self::Wizard => 4,
            Self::Samurai => 5,
            Self::Lord => 6,
            Self::Ninja => 7,
        }
    }

    /// 正式名称を返す。
    pub fn name(self) -> &'static str {
        match self {
            Self::Fighter => "Fighter",
            Self::Mage => "Mage",
            Self::Cleric => "Cleric",
            Self::Thief => "Thief",
            Self::Wizard => "Wizard",
            Self::Samurai => "Samurai",
            Self::Lord => "Lord",
            Self::Ninja => "Ninja",
        }
    }

    /// 名称の頭文字を返す。
    pub fn name_initial(self) -> &'static str {
        match self {
            Self::Fighter => "F",
            Self::Mage => "M",
            Self::Cleric => "C",
            Self::Thief => "T",
            Self::Wizard => "W",
            Self::Samurai => "S",
            Self::Lord => "L",
            Self::Ninja => "N",
        }
    }

    /// 全ての職業を昇順で返す。
    pub fn iter(
    ) -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator + std::iter::FusedIterator + Clone
    {
        [
            Self::Fighter,
            Self::Mage,
            Self::Cleric,
            Self::Thief,
            Self::Wizard,
            Self::Samurai,
            Self::Lord,
            Self::Ninja,
        ]
        .into_iter()
    }
}

/// 職業マスク。
pub type Classes = FlagSet<Class>;

/// 職業マスクをフォーマットし、正式名称のカンマ区切り文字列にする。
#[derive(Debug)]
pub struct ClassesDisplay(Classes);

impl ClassesDisplay {
    pub fn new(classes: Classes) -> Self {
        Self(classes)
    }
}

impl std::fmt::Display for ClassesDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for class in Class::iter() {
            if self.0.contains(class) {
                if !first {
                    f.write_str(",")?;
                }
                f.write_str(class.name())?;
                first = false;
            }
        }

        Ok(())
    }
}

/// 職業マスクをフォーマットし、頭文字を繋げた文字列にする。
#[derive(Debug)]
pub struct ClassesDisplayInitial(Classes);

impl ClassesDisplayInitial {
    pub fn new(classes: Classes) -> Self {
        Self(classes)
    }
}

impl std::fmt::Display for ClassesDisplayInitial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for class in Class::iter() {
            if self.0.contains(class) {
                f.write_str(class.name_initial())?;
            }
        }

        Ok(())
    }
}
