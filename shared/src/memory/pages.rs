use crate::memory::mapping::{AttributeFields, Descriptor, Translation};
use crate::memory::translate::{PageDescriptor, TableDescriptor, Granule512MiB, TranslationGranule, Granule64KiB};
use core::ops::RangeInclusive;
use crate::memory::mmu::VIRTUAL_ADDR_START;
use core::fmt::{Display, Formatter};
use core::fmt;
use itertools::Itertools;

trait BaseAddr {
    fn phys_base_addr(&self) -> usize;
}

impl<T, const N: usize> BaseAddr for [T; N] {
    fn phys_base_addr(&self) -> usize {
        // The binary is still identity mapped, so we don't need to convert here.
        // EVEN in HIGH KERNEL MODE, WE REMOVE THE HIGH FLAGS
        self as *const _ as usize & !VIRTUAL_ADDR_START
    }
}

/// Big monolithic struct for storing the translation tables. Individual levels must be 64 KiB
/// aligned, hence the "reverse" order of appearance.
#[repr(C)]
#[repr(align(65536))]
pub struct FixedSizeTranslationTable<const NUM_TABLES: usize> {
    /// Page descriptors, covering 64 KiB windows per entry.
    lvl3: [[PageDescriptor; 8192]; NUM_TABLES],

    /// Table descriptors, covering 512 MiB windows.
    lvl2: [TableDescriptor; NUM_TABLES],
}

impl<const NUM_TABLES: usize> FixedSizeTranslationTable<{ NUM_TABLES }> {
    /// Create an instance.
    pub const fn new() -> Self {
        Self {
            lvl3: [[PageDescriptor(0); 8192]; NUM_TABLES],
            lvl2: [TableDescriptor(0); NUM_TABLES],
        }
    }

    /// Helper to calculate the lvl2 and lvl3 indices from an address.
    #[inline(always)]
    fn lvl2_lvl3_index_from(&self, addr: usize)
                            -> Result<(usize, usize), &'static str> {
        let lvl2_index = addr as usize >> Granule512MiB::SHIFT;
        let lvl3_index = (addr as usize & Granule512MiB::MASK) >> Granule64KiB::SHIFT;

        if lvl2_index > (NUM_TABLES - 1) {
            return Err("Virtual page is out of bounds of translation table");
        }

        Ok((lvl2_index, lvl3_index))
    }

    /// Returns the PageDescriptor corresponding to the supplied Page.
    #[inline(always)]
    fn page_descriptor_from(&mut self, addr: usize)
                            -> Result<&mut PageDescriptor, &'static str> {
        let (lvl2_index, lvl3_index) = self.lvl2_lvl3_index_from(addr)?;
        Ok(&mut self.lvl3[lvl2_index][lvl3_index])
    }

    unsafe fn map_pages_at(
        &mut self,
        range: RangeInclusive<usize>,
        translation: &Translation,
        attr: &AttributeFields,
    ) -> Result<(), &'static str> {
        for phys_page in range.step_by(Granule64KiB::SIZE) {
            let page_descriptor = self.page_descriptor_from(phys_page)?;
            let output_addr = match translation {
                Translation::Identity => phys_page,
                Translation::Offset(a) => a + phys_page,
            };
            *page_descriptor = PageDescriptor::new(output_addr & Granule64KiB::ALIGN, &attr);
        }

        Ok(())
    }

    pub fn phys_base_addr(&self) -> usize {
        return self.lvl2.phys_base_addr();
    }

    pub fn map_descriptors(&mut self, descriptors: &[Descriptor]) -> Result<(), &'static str> {

        // Populate the l2 entries.
        for (lvl2_nr, lvl2_entry) in self.lvl2.iter_mut().enumerate() {
            *lvl2_entry = self.lvl3[lvl2_nr].phys_base_addr().into();
        }

        for desc in descriptors.iter() {
            let range = (desc.virtual_range)();
            unsafe {
                self.map_pages_at(range, &desc.map.translation, &desc.map.attribute_fields)?;
            }
        }
        Ok(())
    }
}

impl<const NUM_TABLES: usize> Display for FixedSizeTranslationTable<{ NUM_TABLES }> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (lvl2_nr, lvl2_entry) in self.lvl2.iter().enumerate() {
            if lvl2_entry.is_valid() {
                self.lvl3[lvl2_nr].iter()
                    .enumerate()
                    .filter(|l| l.1.is_valid())
                    .map(|l| (l, l))
                    .coalesce(|x, y| {
                        if x.1.1.addr() + 1 == y.0.1.addr() {
                            Ok((x.0, y.1))
                        } else {
                            Err((x, y))
                        }
                    }
                    ).for_each(|(s, e)| {
                    f.write_fmt(format_args!("Table 0x{:08x}..0x{:08x} | Virtual 0x{:08x}..0x{:08x} => Physical 0x{:08x}..0x{:08x}\n",
                                             &self.lvl3[lvl2_nr][s.0] as *const _ as usize, &self.lvl3[lvl2_nr][e.0] as *const _ as usize,
                                             (lvl2_nr << Granule512MiB::SHIFT) + (s.0 << Granule64KiB::SHIFT),
                                             (lvl2_nr << Granule512MiB::SHIFT) + (e.0 << Granule64KiB::SHIFT),
                                             s.1.addr() << Granule64KiB::SHIFT, e.1.addr() << Granule64KiB::SHIFT)).unwrap()
                });
            }
        }
        Ok(())
    }
}
