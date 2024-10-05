use byteorder::{ByteOrder as _, LE};

use crate::monster::Monster;
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
    dbg!(name_known_singular, name_known_plural);
    dbg!(name_unknown_singular, name_unknown_plural);

    todo!();
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
