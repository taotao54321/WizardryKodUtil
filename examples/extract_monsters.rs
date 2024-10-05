use std::path::PathBuf;

use clap::Parser;
use itertools::Itertools as _;

use wizardry_kod_util::*;

/// 原作の ROM からモンスターデータを抽出する。
#[derive(Debug, Parser)]
struct Cli {
    /// 原作の iNES ROM ファイル。
    path_ines: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let rom = Rom::from_ines_file(cli.path_ines)?;

    let monsters = extract::extract_monsters(&rom);

    output_markdown(&monsters);

    Ok(())
}

fn output_markdown(monsters: &[Monster]) {
    println!("| ID  | 確定名 | 不確定名 |");
    println!("| --: | -- | -- |");

    for (id, monster) in monsters.iter().enumerate() {
        output_markdown_row(id, monster);
    }
}

fn output_markdown_row(id: usize, monster: &Monster) {
    let Monster {
        name_known_singular: _,
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
    } = monster;

    let mut fields: Vec<String> = vec![];

    // ID
    fields.push(format!("{id}"));

    // 確定名
    fields.push(format!(
        "{}<br>{name_known_plural}",
        extract::monster_true_name(id)
    ));

    // 不確定名
    fields.push(format!("{name_unknown_singular}<br>{name_unknown_plural}",));

    println!("| {} |", fields.iter().join(" | "));
}
