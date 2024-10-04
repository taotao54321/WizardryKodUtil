use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::string::GameString;

/// モンスター。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monster {
    name_known_singular: GameString,
    name_known_plural: GameString,
    name_unknown_singular: GameString,
    name_unknown_plural: GameString,

    kind: MonsterKind,
}

/// モンスター種別。
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum MonsterKind {
    /// 戦士。
    Fighter = 0,
    /// 魔術師。
    Mage = 1,
    /// 僧侶。
    Cleric = 2,
    /// 盗賊。
    Thief = 3,
    /// 小人。
    Tiny = 4,
    /// 巨人。
    Giant = 5,
    /// 神話。
    Myth = 6,
    /// 竜。
    Dragon = 7,
    /// 動物。
    Animal = 8,
    // 9: 欠番
    /// 不死。
    Undead = 10,
    /// 悪魔。
    Demon = 11,
    /// 昆虫。
    Insect = 12,
    /// 魔法生物。
    Enchanted = 13,
    /// 獣人。
    Lycanthrope = 14,
}
