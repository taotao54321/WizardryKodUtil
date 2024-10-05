use anyhow::ensure;

/// packed BCD (ビッグエンディアン、`LEN` バイト (つまり `2 * LEN` 桁))。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackedBcdBe<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> PackedBcdBe<LEN> {
    /// バイト列をそのまま packed BCD として解釈する。
    pub fn new(inner: [u8; LEN]) -> anyhow::Result<Self> {
        // 原作では最大 12 桁なので、これで十分。
        assert!(LEN <= 6);

        for (i, b) in inner.into_iter().enumerate() {
            ensure!(Self::is_valid_byte(b), "byte {i} is invalid BCD: 0x{b:02X}");
        }

        Ok(Self(inner))
    }

    /// `u64` 値を packed BCD に変換する。
    pub fn from_u64(mut x: u64) -> Self {
        let mut buf = [0; LEN];

        for b in buf.iter_mut().rev() {
            if x == 0 {
                break;
            }
            *b = Self::encode_byte(x % 100);
            x /= 100;
        }

        Self(buf)
    }

    /// packed BCD を `u64` 値に変換する。
    pub fn to_u64(self) -> u64 {
        let mut x = 0;

        for b in self.0 {
            x *= 100;
            x += Self::decode_byte(b);
        }

        x
    }

    /// `0..=99` の値を packed BCD バイトに変換する。
    fn encode_byte(x: u64) -> u8 {
        assert!(x <= 99);

        let x = u8::try_from(x).unwrap();
        let tens = x / 10;
        let ones = x % 10;

        ones | (tens << 4)
    }

    /// packed BCD バイトを `0..=99` の値に変換する。
    fn decode_byte(bcd: u8) -> u64 {
        let ones = bcd & 0x0F;
        let tens = bcd >> 4;

        u64::from(ones + 10 * tens)
    }

    /// `b` が有効な packed BCD バイトかどうかを返す。
    fn is_valid_byte(b: u8) -> bool {
        matches!(b & 0x0F, 0..=9) && matches!(b >> 4, 0..=9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_bcd_be() {
        assert!(PackedBcdBe::<6>::new([0, 0, 0, 0, 0, 0x0A]).is_err());

        fn roundtrip(buf: [u8; 6], x: u64) {
            let bcd = PackedBcdBe::new(buf).unwrap();
            assert_eq!(bcd.to_u64(), x);
            assert_eq!(PackedBcdBe::from_u64(x), bcd);
        }

        roundtrip([0; 6], 0);
        roundtrip([0, 0, 0, 0, 0x12, 0x34], 1234);
        roundtrip([0x12, 0x34, 0, 0, 0, 0], 123400000000);
        roundtrip([0x12, 0x34, 0x56, 0x78, 0x90, 0x12], 123456789012);
    }
}
