use bitflags::bitflags;

use crate::dice::define_dice_expr;
use crate::element::Elements;
use crate::string::GameString;

/// モンスター。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monster {
    name_known_singular: GameString,
    name_known_plural: GameString,
    name_unknown_singular: GameString,
    name_unknown_plural: GameString,

    kinds: MonsterKinds,
    spawn_dice_expr: MonsterSpawnDiceExpr,
    hp_dice_expr: MonsterHpDiceExpr,
    ac: i8,
    drain_xl: u8,
    healing: i8,
    drop_table_id_wandering: u8,
    drop_table_id_guardian: u8,
    follower_monster_id: u8,
    follower_probability: u8,
    mage_spell_lv: u8,
    cleric_spell_lv: u8,
    element_resistance: Elements,
}

bitflags! {
    /// モンスター種別マスク。
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct MonsterKinds: u16 {
        /// 戦士。
        const FIGHTER = 1 << 0;
        /// 魔術師。
        const MAGE = 1 << 1;
        /// 僧侶。
        const CLERIC = 1 << 2;
        /// 盗賊。
        const THIEF = 1 << 3;
        /// 小人。
        const TINY = 1 << 4;
        /// 巨人。
        const GIANT = 1 << 5;
        /// 神話。
        const MYTH = 1 << 6;
        /// 竜。
        const DRAGON = 1 << 7;
        /// 動物。
        const ANIMAL = 1 << 8;
        /// 欠番 9。
        const UNUSED_9 = 1 << 9;
        /// 不死。
        const UNDEAD = 1 << 10;
        /// 悪魔。
        const DEMON = 1 << 11;
        /// 昆虫。
        const INSECT = 1 << 12;
        /// 魔法生物。
        const ENCHANTED = 1 << 13;
        /// 獣人。
        const LYCANTHROPE = 1 << 14;
        /// 欠番 15。
        const UNUSED_15 = 1 << 15;
    }
}

define_dice_expr!(MonsterSpawnDiceExpr);

define_dice_expr!(MonsterHpDiceExpr);
