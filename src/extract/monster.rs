use byteorder::{ByteOrder as _, LE};

use crate::bcd::PackedBcdBe;
use crate::element::Elements;
use crate::monster::{
    Monster, MonsterAbilitys, MonsterHpDiceExpr, MonsterKinds, MonsterMeleeDiceExpr,
    MonsterSpawnDiceExpr,
};
use crate::rom::Rom;
use crate::string::GameString;
use crate::util::{SliceExt as _, U8SliceExt as _};

pub const MONSTER_COUNT: usize = 90;

/// 全モンスターをIDの昇順で抽出する。
pub fn extract_monsters(rom: &Rom) -> Vec<Monster> {
    (0..MONSTER_COUNT)
        .map(|id| extract_monster(rom, id))
        .collect()
}

/// 指定したIDのモンスターを抽出する。
pub fn extract_monster(rom: &Rom, id: usize) -> Monster {
    assert!(id < MONSTER_COUNT);

    let bank = rom.prg_bank(6);
    let table = &bank[..2 * MONSTER_COUNT];

    let buf = {
        let ptr = LE::read_u16(&table[2 * id..][..2]);
        &bank[usize::from(ptr - 0x8000)..]
    };

    let (name_known_singular, name_known_plural, buf) = split_first_name_pair(bank, buf);
    let (name_unknown_singular, name_unknown_plural, buf) = split_first_name_pair(bank, buf);

    let (kinds, buf) = buf.split_first_u16le().unwrap();
    let kinds = MonsterKinds::new(kinds).unwrap();
    let (spawn_dice_expr, buf) = split_first_dice_expr::<MonsterSpawnDiceExpr>(buf);
    let (hp_dice_expr, buf) = split_first_dice_expr::<MonsterHpDiceExpr>(buf);
    let (ac, buf) = buf.split_first_i8().unwrap();
    let (drain_xl, buf) = buf.split_first_u8().unwrap();
    let (healing, buf) = buf.split_first_i8().unwrap();
    let (drop_table_id_wandering, buf) = buf.split_first_u8().unwrap();
    let (drop_table_id_guardian, buf) = buf.split_first_u8().unwrap();
    let (follower_monster_id, buf) = buf.split_first_u8().unwrap();
    let (follower_probability, buf) = buf.split_first_u8().unwrap();
    let (mage_spell_lv, buf) = buf.split_first_u8().unwrap();
    let (cleric_spell_lv, buf) = buf.split_first_u8().unwrap();
    let (breath_elements, buf) = buf.split_first_u8().unwrap();
    let breath_elements = Elements::new(breath_elements).unwrap();
    let (spell_resistance, buf) = buf.split_first_u8().unwrap();
    let (element_resistance, buf) = buf.split_first_u8().unwrap();
    let element_resistance = Elements::new(element_resistance).unwrap();
    let (abilitys, buf) = buf.split_first_u8().unwrap();
    let abilitys = MonsterAbilitys::new(abilitys).unwrap();
    let (xp, buf) = split_first_bcd::<4>(buf);
    let xp = xp.to_u64();
    let (melee_count, mut buf) = buf.split_first_u8().unwrap();

    let mut melee_dice_exprs = Vec::<MonsterMeleeDiceExpr>::new();
    for _ in 0..melee_count {
        let dice_expr;
        (dice_expr, buf) = split_first_dice_expr(buf);
        melee_dice_exprs.push(dice_expr);
    }

    Monster {
        name_known_singular,
        name_known_plural,
        name_unknown_singular,
        name_unknown_plural,

        kinds,
        spawn_dice_expr,
        hp_dice_expr,
        ac,
        drain_xl,
        healing,
        drop_table_id_wandering,
        drop_table_id_guardian,
        follower_monster_id,
        follower_probability,
        mage_spell_lv,
        cleric_spell_lv,
        breath_elements,
        spell_resistance,
        element_resistance,
        abilitys,
        xp,
        melee_dice_exprs,
    }
}

fn split_first_name_pair<'a>(bank: &'a [u8], buf: &'a [u8]) -> (GameString, GameString, &'a [u8]) {
    let (ptr, buf) = buf.split_first_u16le().unwrap();
    let (singular, plural) = read_name_pair(&bank[usize::from(ptr - 0x8000)..]);

    (singular, plural, buf)
}

fn read_name_pair(buf: &[u8]) -> (GameString, GameString) {
    // NOTE: モンスターの場合、名前データは必ず 0 終端まで読まれる。
    // (その後 16 バイトまでで切り捨てたものがロードされるが、原作ではこの切り捨ては起こらない)

    let (singular, buf) = buf.split_once_(|&b| b == 0).unwrap();
    let singular = GameString::from_bytes(singular).unwrap();

    let (plural, _) = buf.split_once_(|&b| b == 0).unwrap();
    let plural = GameString::from_bytes(plural).unwrap();

    (singular, plural)
}

fn split_first_dice_expr<T>(buf: &[u8]) -> (T, &[u8])
where
    T: From<[u8; 3]>,
{
    let (&dice_expr, buf) = buf.split_first_chunk::<3>().unwrap();
    let dice_expr = T::from(dice_expr);

    (dice_expr, buf)
}

fn split_first_bcd<const LEN: usize>(buf: &[u8]) -> (PackedBcdBe<LEN>, &[u8]) {
    let (&bcd, buf) = buf.split_first_chunk::<LEN>().unwrap();
    let bcd = PackedBcdBe::new(bcd).unwrap();

    (bcd, buf)
}

/// 指定したIDのモンスターの正体名を返す。
pub fn monster_true_name(id: usize) -> &'static str {
    const TABLE: [&str; MONSTER_COUNT] = [
        "Bubbly Slime",
        "Orc",
        "Kobold",
        "Undead Kobold",
        "Dink",
        "Hippopotamus",
        "Magician",
        "Depraved Cleric",
        "Creeping Coin?",
        "Highwayman",
        "Man at Arms",
        "Zombie",
        "Mummy",
        "Witch",
        "Ninja",
        "Rabid Rat",
        "Were Panther",
        "Fuzzball [A]",
        "Vorpal Bunny",
        "Cleric",
        "Dragon Fly",
        "Doom Toad",
        "Smog Beast",
        "Banshee",
        "Boring Beetle",
        "Swordsman",
        "Blade Bear",
        "Were Lion",
        "Seraph",
        "Master Ninja",
        "Carrier",
        "Rhino Beetle",
        "Fuzzball [B]",
        "Were Amoeba",
        "No-see um",
        "Sorceress",
        "Medusalizard",
        "Horrid Hound",
        "Scorpion",
        "Evil Eye",
        "Manticore",
        "Webspinner",
        "Constrictor",
        "Lesser Demon",
        "Scryll",
        "Ferocious Fiend",
        "Orc Lord",
        "Giant Bat",
        "Were Bat",
        "Vampire Bat",
        "Corrosive Slime",
        "Foaming Mold",
        "Murphy's Ghost",
        "Succubus",
        "Giant Wasp",
        "Giant Viper",
        "The High Master",
        "High Cleric [A]",
        "Arch Mage [A]",
        "Poison Giant",
        "Vampire",
        "Will o' Wisp",
        "Greater Demon",
        "Air Giant",
        "Giant Zombie",
        "Fuzzball [C]",
        "Giant Hornet",
        "Giant Crab",
        "Blob",
        "Arch Mage [B]",
        "High Cleric [B]",
        "High Master",
        "Iron Golem",
        "Black Dragon",
        "Gold Dragon",
        "Flack",
        "Sidelle",
        "Raver Lord",
        "Mifune",
        "Arch Demon",
        "Maelific",
        "Vampire Lord",
        "Lycurgus",
        "Demon Lord",
        "Kobold King",
        "Magic Sword",
        "Magic Helmet",
        "Magic Shield",
        "Magic Gauntlet",
        "Magic Armor",
    ];

    TABLE[id]
}
