mod mair;
mod pages;
mod translate;

pub mod mmu;
pub mod mapping;

pub use self::pages::{PageTable, NUM_ENTRIES_4KIB};
pub use self::mmu::TranslationTable;

use core::ops::RangeInclusive;

