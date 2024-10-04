use std::path::Path;

use anyhow::{ensure, Context as _};

/// 原作の ROM。
///
/// headerless SHA-1 hash: 98cbf6d8d410c6725b59c08c35a22f29c3531aa8
#[derive(Debug)]
pub struct Rom(Box<[u8; ROM_LEN]>);

const PRG_BANK_COUNT: usize = 16;
const PRG_BANK_LEN: usize = 0x2000;
const PRG_LEN: usize = PRG_BANK_COUNT * PRG_BANK_LEN;

const CHR_BANK_COUNT: usize = 128;
const CHR_BANK_LEN: usize = 0x400;
const CHR_LEN: usize = CHR_BANK_COUNT * CHR_BANK_LEN;

const ROM_LEN: usize = PRG_LEN + CHR_LEN;

impl Rom {
    /// iNES 形式のファイルから ROM をロードする。
    pub fn from_ines_file<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        Self::_from_ines_file(path.as_ref())
    }

    fn _from_ines_file(path: &Path) -> anyhow::Result<Self> {
        let ines =
            std::fs::read(path).with_context(|| format!("cannot read '{}'", path.display()))?;

        Self::from_ines_bytes(&ines)
    }

    /// iNES 形式のバイト列から ROM をロードする。
    pub fn from_ines_bytes(ines: &[u8]) -> anyhow::Result<Self> {
        const INES_HEADER_LEN: usize = 16;
        const INES_FILE_LEN: usize = INES_HEADER_LEN + ROM_LEN;
        const INES_MAGIC: &[u8] = b"NES\x1A";

        ensure!(
            ines.len() == INES_FILE_LEN,
            "ROM size mismach: (actual={}, expect={INES_FILE_LEN})",
            ines.len(),
        );

        ensure!(ines.starts_with(INES_MAGIC), "iNES magic not found");

        let (_, rom) = ines.split_first_chunk::<INES_HEADER_LEN>().unwrap();
        let rom: Box<[u8; ROM_LEN]> = Box::<[u8]>::from(rom).try_into().unwrap();

        Ok(Self(rom))
    }

    /// PRG-ROM 全体を返す。
    pub fn prg(&self) -> &[u8; PRG_LEN] {
        self.split_prg_chr().0
    }

    /// 指定したIDの PRG バンク (0x2000 バイト単位) を返す。
    pub fn prg_bank(&self, id: usize) -> &[u8; PRG_BANK_LEN] {
        assert!(id < PRG_BANK_COUNT);

        self.prg()[PRG_BANK_LEN * id..][..PRG_BANK_LEN]
            .try_into()
            .unwrap()
    }

    /// 固定 PRG バンク (PRG-ROM 末尾の 0x4000 バイト) を返す。
    pub fn prg_fixed(&self) -> &[u8; 2 * PRG_BANK_LEN] {
        self.prg()[PRG_BANK_LEN * 14..][..2 * PRG_BANK_LEN]
            .try_into()
            .unwrap()
    }

    /// CHR-ROM 全体を返す。
    pub fn chr(&self) -> &[u8; CHR_LEN] {
        self.split_prg_chr().1
    }

    /// 指定したIDの CHR バンク (0x400 バイト単位) を返す。
    pub fn chr_bank(&self, id: usize) -> &[u8; CHR_BANK_LEN] {
        assert!(id < CHR_BANK_COUNT);

        self.chr()[CHR_BANK_LEN * id..][..CHR_BANK_LEN]
            .try_into()
            .unwrap()
    }

    fn split_prg_chr(&self) -> (&[u8; PRG_LEN], &[u8; CHR_LEN]) {
        let (prg, chr) = self.0.split_first_chunk::<PRG_LEN>().unwrap();
        let chr: &[u8; CHR_LEN] = chr.try_into().unwrap();

        (prg, chr)
    }
}
