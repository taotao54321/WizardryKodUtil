/// ダイス式を表す型を定義する。
///
/// 微妙に実装が異なる各種ダイスロールを扱う際のボイラープレート軽減用。
macro_rules! define_dice_expr {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name {
            count: u8,
            face: u8,
            bias: u8,
        }

        impl $name {
            pub fn new(count: u8, face: u8, bias: u8) -> Self {
                Self { count, face, bias }
            }

            pub fn count(self) -> u8 {
                self.count
            }

            pub fn face(self) -> u8 {
                self.face
            }

            pub fn bias(self) -> u8 {
                self.bias
            }
        }

        impl From<[u8; 3]> for $name {
            fn from(buf: [u8; 3]) -> Self {
                Self::new(buf[0], buf[1], buf[2])
            }
        }
    };
}
pub(crate) use define_dice_expr;
