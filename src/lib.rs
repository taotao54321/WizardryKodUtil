//! NES 版 Wizardry #2 (Knight of Diamonds) 関連ユーティリティ。

// NOTE: flagset クレートによって作られる enum の内部値に依存してはならない。
// 内部値はビットマスクとして指定した値とは関係なく 0, 1, 2, ... の順に振られるため。

mod alignment;
pub mod bcd;
mod class;
mod dice;
mod element;
pub mod extract;
mod item;
mod monster;
mod rng;
mod rom;
mod string;
pub mod util;

pub use self::alignment::*;
pub use self::class::*;
pub use self::element::*;
pub use self::item::*;
pub use self::monster::*;
pub use self::rng::*;
pub use self::rom::*;
pub use self::string::*;
