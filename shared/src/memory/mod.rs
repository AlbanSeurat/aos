mod mair;
mod pages;
mod translate;

pub mod mmu;
pub mod mapping;

pub use self::pages::{PageTable, TranslationTable, NUM_ENTRIES_4KIB};

use core::ops::RangeInclusive;

