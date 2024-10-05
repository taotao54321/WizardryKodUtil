use std::path::PathBuf;

use clap::Parser;

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

    dbg!(monsters);

    Ok(())
}
