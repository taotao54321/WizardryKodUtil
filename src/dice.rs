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
    };
}
pub(crate) use define_dice_expr;
