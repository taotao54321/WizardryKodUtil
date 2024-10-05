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

impl MonsterKinds {
    /// 含まれる全てのモンスター種別名を ',' で連結して表示する。
    ///
    /// 例: "戦士,魔術師"
    pub fn display(self) -> MonsterKindsDisplay {
        MonsterKindsDisplay(self)
    }

    /// 含まれる全てのモンスター種別略称を連結して表示する。
    ///
    /// 例: "戦魔"
    pub fn display_abbrev(self) -> MonsterKindsDisplayAbbrev {
        MonsterKindsDisplayAbbrev(self)
    }
}

#[derive(Debug)]
pub struct MonsterKindsDisplay(MonsterKinds);

impl std::fmt::Display for MonsterKindsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TABLE: [(MonsterKinds, &str); 16] = [
            (MonsterKinds::FIGHTER, "戦士"),
            (MonsterKinds::MAGE, "魔術師"),
            (MonsterKinds::CLERIC, "僧侶"),
            (MonsterKinds::THIEF, "盗賊"),
            (MonsterKinds::TINY, "小人"),
            (MonsterKinds::GIANT, "巨人"),
            (MonsterKinds::MYTH, "神話"),
            (MonsterKinds::DRAGON, "竜"),
            (MonsterKinds::ANIMAL, "動物"),
            (MonsterKinds::UNUSED_9, "(未使用9)"),
            (MonsterKinds::UNDEAD, "不死"),
            (MonsterKinds::DEMON, "悪魔"),
            (MonsterKinds::INSECT, "昆虫"),
            (MonsterKinds::ENCHANTED, "魔法生物"),
            (MonsterKinds::LYCANTHROPE, "獣人"),
            (MonsterKinds::UNUSED_15, "(未使用15)"),
        ];

        let mut first = true;
        for (kind, name) in TABLE {
            if self.0.contains(kind) {
                if !first {
                    f.write_str(",")?;
                    first = false;
                }
                f.write_str(name)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct MonsterKindsDisplayAbbrev(MonsterKinds);

impl std::fmt::Display for MonsterKindsDisplayAbbrev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TABLE: [(MonsterKinds, &str); 16] = [
            (MonsterKinds::FIGHTER, "戦"),
            (MonsterKinds::MAGE, "魔"),
            (MonsterKinds::CLERIC, "僧"),
            (MonsterKinds::THIEF, "盗"),
            (MonsterKinds::TINY, "小"),
            (MonsterKinds::GIANT, "巨"),
            (MonsterKinds::MYTH, "神"),
            (MonsterKinds::DRAGON, "竜"),
            (MonsterKinds::ANIMAL, "動"),
            (MonsterKinds::UNUSED_9, "謎"),
            (MonsterKinds::UNDEAD, "不"),
            (MonsterKinds::DEMON, "悪"),
            (MonsterKinds::INSECT, "昆"),
            (MonsterKinds::ENCHANTED, "傀"), // 「傀儡」に由来。「魔」は重複するので。
            (MonsterKinds::LYCANTHROPE, "獣"),
            (MonsterKinds::UNUSED_15, "謎"),
        ];

        for (kind, name) in TABLE {
            if self.0.contains(kind) {
                f.write_str(name)?;
            }
        }

        Ok(())
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

// モンスターの 1 グループあたりの出現数ダイス式。
define_dice_expr!(MonsterSpawnDiceExpr);

impl MonsterSpawnDiceExpr {
    pub fn bias_decoded(self) -> i16 {
        decode_spawn_hp_bias(self.bias)
    }
}

impl std::fmt::Display for MonsterSpawnDiceExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}{:+}", self.count, self.face, self.bias_decoded())
    }
}

// モンスターのHPダイス式。
define_dice_expr!(MonsterHpDiceExpr);

impl MonsterHpDiceExpr {
    pub fn bias_decoded(self) -> i16 {
        decode_spawn_hp_bias(self.bias)
    }
}

impl std::fmt::Display for MonsterHpDiceExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}{:+}", self.count, self.face, self.bias_decoded())
    }
}

// モンスターの打撃 1 回のダメージダイス式。
define_dice_expr!(MonsterMeleeDiceExpr);

/// 出現数ダイス式およびHPダイス式の追加値を `i16` 値に変換する。
///
/// `0..=149` が正、`150..=255` が負 (2 の補数) とみなされる。
fn decode_spawn_hp_bias(bias: u8) -> i16 {
    if bias <= 149 {
        i16::from(bias)
    } else {
        i16::from(bias as i8)
    }
}
