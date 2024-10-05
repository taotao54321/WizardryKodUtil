use bitflags::bitflags;

use crate::dice::define_dice_expr;
use crate::element::Elements;
use crate::string::GameString;

/// モンスター。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monster {
    pub name_known_singular: GameString,
    pub name_known_plural: GameString,
    pub name_unknown_singular: GameString,
    pub name_unknown_plural: GameString,

    pub kinds: MonsterKinds,
    pub spawn_dice_expr: MonsterSpawnDiceExpr,
    pub hp_dice_expr: MonsterHpDiceExpr,
    pub ac: i8,
    pub drain_xl: u8,
    pub healing: i8,
    pub drop_table_id_wandering: u8,
    pub drop_table_id_guardian: u8,
    pub follower_monster_id: u8,
    pub follower_probability: u8,
    pub mage_spell_lv: u8,
    pub cleric_spell_lv: u8,
    pub breath_elements: Elements,
    pub spell_resistance: u8,
    pub element_resistance: Elements,
    pub abilitys: MonsterAbilitys,
    pub xp: u64,
    pub melee_dice_exprs: Vec<MonsterMeleeDiceExpr>,
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
        /// (未使用)
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
        /// (未使用)
        const UNUSED_15 = 1 << 15;
    }
}

bitflags! {
    /// モンスター特殊能力マスク。
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct MonsterAbilitys: u8 {
        /// 石化打撃。
        const PETRIFY = 1 << 0;
        /// 毒打撃。
        const POISON = 1 << 1;
        /// 麻痺打撃。
        const PARALYZE = 1 << 2;
        /// 首切り打撃。
        const CRITICAL = 1 << 3;
        /// 睡眠弱点。
        const SLEEPY = 1 << 4;
        /// 逃走する。
        const FLEE = 1 << 5;
        /// 仲間を呼ぶ。
        const CALL = 1 << 6;
        /// (未使用)
        const UNUSED_7 = 1 << 7;
    }
}

define_dice_expr!(MonsterSpawnDiceExpr);

define_dice_expr!(MonsterHpDiceExpr);

define_dice_expr!(MonsterMeleeDiceExpr);
