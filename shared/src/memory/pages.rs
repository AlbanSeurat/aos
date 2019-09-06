use crate::memory::mapping::{AttributeFields, Descriptor, Granule, Translation};
use crate::memory::translate::{Lvl2BlockDescriptor, TableDescriptor};

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

pub const NUM_ENTRIES_4KIB: usize = 512;
const ALIGNED_2M: usize = 0xFFFFFFFFFFE00000;
const ALIGNED_4K: usize = 0xFFFFFFFFFFFFF000;
const TWO_MIB_SHIFT: usize = 21;

// A wrapper struct is needed here so that the align attribute can be used.
#[repr(C)]
#[repr(align(4096))]
pub struct PageTable {
    pub entries: [u64; NUM_ENTRIES_4KIB],
}

pub fn map_descriptor(descriptor: &Descriptor, page2: &mut PageTable) -> Result<(), &'static str>{

    let range = (descriptor.virtual_range)();
    let start = range.start();
    let end = range.end();

    match descriptor.granule {
        Granule::Regular => map_4k_block(descriptor, page2, *start),
        Granule::BigPage => map_2M_blocks(descriptor, page2, *start, *end)
    }
}

fn map_4k_block(desc: &Descriptor, page2: &mut PageTable, start: usize) -> Result<(), &'static str> {
    debugln!("map 4k: {:#x}", start);
    let addr_aligned = start & ALIGNED_4K;
    debugln!("map 4k aligned : {:#x}", addr_aligned);
    /*
    let page3 = match map_2M_table(desc, page2, start) {
        Err(s) => return Err(s),
        Ok(p) => p,
    };*/
    // Map in Page 3 the correct segment describe (does not manage - yet - cross boundary 2M segment)
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
/*
fn map_2M_table(desc: &Descriptor, page2: &mut PageTable, start: usize) -> Result<PageTable, &'static str> {
    // align to 2M the descriptor address
    debugln!("map 2M: {:#x}", start);
    let addr_aligned = start & ALIGNED_2M;
    debugln!("map 2M aligned : {:#x}", addr_aligned);
    page2.entries[addr_aligned >> TWO_MIB_SHIFT] = match TableDescriptor::new(page3.entries.base_addr_usize()) {
        Err(s) => return Err(s),
        Ok(page) => page.value(),
    };
    Ok(page3)
}
*/
/*
pub fn user_2M_page_mapping(desc : &Descriptor, virt_addr: usize) -> Option<Lvl2BlockDescriptor> {

    let option= match get_user_virt_addr_properties(desc, virt_addr) {
        Err(s) => panic!(s),
        Ok(i) => i,
    };

    return mapDescriptorToBlock(option);
}


fn mapDescriptorToBlock(option: Option<(usize, AttributeFields)>) -> Option<Lvl2BlockDescriptor> {
    if option.is_some() {
        let (output_addr, attribute_fields) = option.unwrap();
        let page_desc = match Lvl2BlockDescriptor::new(output_addr, attribute_fields) {
            Err(s) => panic!(s),
            Ok(desc) => desc,
        };
        return Some(page_desc)
    } else {
        return None
    }
}


fn get_layout_properties(descriptor : &Descriptor, virt_addr : usize) -> Option<(usize, AttributeFields)> {
    if descriptor.map.is_some() {
        let mapping = descriptor.map.unwrap();
        let output_addr = match mapping.translation {
            Translation::Identity => virt_addr,
            Translation::Offset(a) => a + (virt_addr - (descriptor.virtual_range)().start()),
        };
        return Some((output_addr, mapping.attribute_fields))
    } else {
        return None
    }
}

*/
