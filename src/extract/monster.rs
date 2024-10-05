use byteorder::{ByteOrder as _, LE};

use crate::element::Elements;
use crate::monster::{
    Monster, MonsterAbilitys, MonsterHpDiceExpr, MonsterKinds, MonsterMeleeDiceExpr,
    MonsterSpawnDiceExpr,
};
use crate::rom::Rom;
use crate::string::GameString;
use crate::util::{SliceExt as _, U8SliceExt as _};

const MONSTER_COUNT: usize = 90;

pub fn extract_monsters(rom: &Rom) -> Vec<Monster> {
    (0..MONSTER_COUNT)
        .map(|id| extract_monster(rom, id))
        .collect()
}

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
    let kinds = MonsterKinds::from_bits(kinds).unwrap();
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
    let (element_resistance, buf) = buf.split_first_u8().unwrap();
    let element_resistance = Elements::from_bits(element_resistance).unwrap();
    let (abilitys, buf) = buf.split_first_u8().unwrap();
    let abilitys = MonsterAbilitys::from_bits(abilitys).unwrap();

    // TODO

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
        element_resistance,
        abilitys,
        // TODO
        xp: 0,
        melee_dice_exprs: vec![],
    }
}

fn split_first_name_pair<'a>(bank: &'a [u8], buf: &'a [u8]) -> (GameString, GameString, &'a [u8]) {
    let (ptr, buf) = buf.split_first_u16le().unwrap();
    let (singular, plural) = read_name_pair(&bank[usize::from(ptr - 0x8000)..]);

    (singular, plural, buf)
}

fn read_name_pair(buf: &[u8]) -> (GameString, GameString) {
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
