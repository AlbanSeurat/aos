use crate::memory::mapping::{AttributeFields, Descriptor, Granule, Translation};
use crate::memory::translate::{Lvl2BlockDescriptor, TableDescriptor, PageDescriptor};

pub const NUM_ENTRIES_4KIB: usize = 512;
const ALIGNED_2M: usize = 0xFFFFFFFFFFE00000;
const ALIGNED_4K: usize = 0xFFFFFFFFFFFFF000;
const TWO_MIB_SHIFT: usize = 21;
const FOUR_KIB_SHIFT: usize = 12;


pub trait BaseAddr {
    fn base_addr_u64(&self) -> u64;
    fn base_addr_usize(&self) -> usize;
}

impl BaseAddr for [u64; 512] {
    fn base_addr_u64(&self) -> u64 {
        self as *const u64 as u64
    }
    fn base_addr_usize(&self) -> usize {
        self as *const u64 as usize
    }
}

pub struct TranslationTable {
    tables_base_addr: usize,
    tables_count: usize,
}

impl TranslationTable {
    pub fn new(tables_base_addr: usize) -> TranslationTable {
        TranslationTable {
            tables_base_addr,
            tables_count: 0
        }
    }

    pub fn alloc_table(&mut self) -> Result<*mut PageTable, &'static str> {
        if self.tables_count < 512 {
            let page_addr = self.tables_base_addr + self.tables_count * 0x1000;
            self.tables_count += 1;
            unsafe {
                let page = page_addr as *mut PageTable;
                (*page).entries = core::mem::zeroed();
                return Ok(page)
            }
        } else {
            return Err("Can not allocate mmu descriptors table");
        }
    }
}

// A wrapper struct is needed here so that the align attribute can be used.
#[repr(C)]
#[repr(align(4096))]
pub struct PageTable {
    pub entries: [u64; NUM_ENTRIES_4KIB],
}

pub fn init(tb: & mut TranslationTable, level2: &mut PageTable, descriptors: &[Descriptor]) -> Result<(), &'static str> {
    for desc in descriptors.iter() {
        match map_descriptor(tb, desc, level2) {
            Ok(_) => (),
            Err(s) => return Err(s)
        }
    }
    Ok(())
}

fn map_descriptor(tb: &mut TranslationTable, descriptor: &Descriptor, page2: &mut PageTable) -> Result<(), &'static str>{

    let range = (descriptor.virtual_range)();
    let start = range.start();
    let end = range.end();

    match descriptor.granule {
        Granule::Regular => map_4k_blocks(tb, descriptor, page2, *start, *end),
        Granule::BigPage => map_2M_blocks(descriptor, page2, *start, *end)
    }
}

fn map_4k_blocks(tb: &mut TranslationTable, desc: &Descriptor, page2: &mut PageTable, start: usize, end: usize) -> Result<(), &'static str> {
    let segment = start & ALIGNED_2M;
    let page3 = match map_2M_table(tb, desc, page2, segment) {
        Err(s) => return Err(s),
        Ok(p) => p,
    };
    let mut cur = start;
    while cur < end {
        unsafe {
            match map_4k_block(desc, &mut *page3, cur, segment) {
                Err(s) => return Err(s),
                Ok(i) => i,
            }
        }
        cur += 1 << FOUR_KIB_SHIFT;
    }
    // Map in Page 3 the correct segment describe (does not manage - yet - cross boundary 2M segment)
    Ok(())
}

fn map_2M_table(tb: &mut TranslationTable, desc: &Descriptor, page2: &mut PageTable, segment: usize) -> Result<*mut PageTable, &'static str> {
    // align to 2M the descriptor address
    let page3 = page2.entries[segment >> TWO_MIB_SHIFT];
    if page3 & 3 == 3 {
        return Ok((page3 - 3) as *mut PageTable);
    } else {
        let level3 = match tb.alloc_table() {
            Ok(table) => table,
            Err(s) => return Err(s)
        };
        unsafe {
            page2.entries[segment >> TWO_MIB_SHIFT] = match TableDescriptor::new((*level3).entries.base_addr_usize()) {
                Err(s) => return Err(s),
                Ok(table) => table.value(),
            };
        }
        Ok(level3)
    }
}

fn map_4k_block(desc: &Descriptor, page3: &mut PageTable, start: usize, segment: usize) -> Result<(), &'static str> {
    let addr_aligned = start & ALIGNED_4K;
    let output_addr = match desc.map.translation {
        Translation::Identity => addr_aligned,
        Translation::Offset(a) => a + (addr_aligned - start),
    };
    let page_desc = match PageDescriptor::new(output_addr, desc.map.attribute_fields) {
        Err(s) => return Err(s),
        Ok(desc) => desc,
    };
    page3.entries[(addr_aligned - segment) >> FOUR_KIB_SHIFT] = page_desc.value();
    Ok(())
}


fn map_2M_blocks(desc: &Descriptor, page2: &mut PageTable, start: usize, end: usize) -> Result<(), &'static str> {
    let mut cur = start;
    while cur < end {
        match map_2M_block(desc, page2, cur) {
            Err(s) => return Err(s),
            Ok(i) => i,
        }
        cur += 1 << TWO_MIB_SHIFT;
    }
    Ok(())
}

// TODO : test offset properly (not sure it works well)
fn map_2M_block(desc: &Descriptor, page2: &mut PageTable, start: usize) -> Result<(), &'static str> {
    // align to 2M the descriptor address
    let addr_aligned = start & ALIGNED_2M;
    let output_addr = match desc.map.translation {
        Translation::Identity => addr_aligned,
        Translation::Offset(a) => a + (addr_aligned - start),
    };
    let page_desc = match Lvl2BlockDescriptor::new(output_addr, desc.map.attribute_fields) {
        Err(s) => return Err(s),
        Ok(desc) => desc,
    };
    page2.entries[addr_aligned >> TWO_MIB_SHIFT] = page_desc.value();
    Ok(())
}
