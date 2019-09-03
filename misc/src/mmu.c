#include <raspi.h>
#include <kprintf.h>
#include <stdint.h>

#define VA_START 			0xffff000000000000LL

// descriptor flags 0 et 1 : access and type of descriptor
#define MM_TYPE_PAGE_TABLE		0x3ULL // 1 has access + b10 means that it points to a table descriptor
#define MM_TYPE_BLOCK			0x1ULL // 1 has access + b01 means that it points to real physical address

#define PGD_SHIFT			39ULL
#define PUD_SHIFT			30ULL
#define PMD_SHIFT			21ULL

#define PAGE_SIZE   		4096
#define PTRS_PER_TABLE		512ULL // nb adress per MMU tables (PGD, PUD, PMD, ...)

#define MT_NORMAL_NC			0x1 // index to

#define MM_ACCESS			(0x1 << 10)
#define MT_DEVICE_nGnRnE 	0x0ULL
#define MMU_FLAGS	 		(MM_TYPE_BLOCK | (MT_NORMAL_NC << 2) | MM_ACCESS)
#define MMU_DEVICE_FLAGS	(MM_TYPE_BLOCK | (MT_DEVICE_nGnRnE << 2) | MM_ACCESS)

/*
 * Memory region attributes:
 *
 *   n = AttrIndx[2:0]
 *			n	MAIR
 *   DEVICE_nGnRnE	000	00000000
 *   NORMAL_NC		001	01000100
 */
#define MT_DEVICE_nGnRnE 		0x0
#define MT_NORMAL_NC			0x1
#define MT_DEVICE_nGnRnE_FLAGS		0x00
#define MT_NORMAL_NC_FLAGS  		0x44
#define MAIR_VALUE			(MT_DEVICE_nGnRnE_FLAGS << (8 * MT_DEVICE_nGnRnE)) | (MT_NORMAL_NC_FLAGS << (8 * MT_NORMAL_NC))

#define TCR_T0SZ			(64 - 48)
#define TCR_T1SZ			((64 - 48) << 16)
#define TCR_TG0_4K			(0 << 14)
#define TCR_TG1_4K			(2 << 30)
#define TCR_VALUE			(TCR_T0SZ | TCR_T1SZ | TCR_TG0_4K | TCR_TG1_4K)

// only one descriptor necessary for PGD and PUD
uint64_t PGD_TABLE[PTRS_PER_TABLE] = {0}; // must be 4k aligned
uint64_t PUD_TABLE[PTRS_PER_TABLE] = {0};
uint64_t PMD_TABLE[PTRS_PER_TABLE] = {0};

static inline uint64_t va_to_offset(uint64_t vaddress, uint64_t shift) {
    return vaddress >> shift & (PTRS_PER_TABLE - 1);
}

static inline void create_table_entry(void * table, void * next_table, uint64_t vaddress, uint64_t shift) {
    register uint64_t offset, next;
    offset = va_to_offset(vaddress, shift);
    next = ((uint64_t)next_table) | MM_TYPE_PAGE_TABLE;
    asm("str %0, [%1, %2, lsl #3]" :: "r" (next), "r" (table), "r" (offset));
}

static inline void create_block_map(void * table, uint64_t phys, uint64_t flags,
        uint64_t vaddress_start, uint64_t vaddress_end, uint64_t shift) {
    register uint64_t offset, offset_end, final;
    offset = va_to_offset(vaddress_start, shift);
    offset_end = va_to_offset(vaddress_end, shift);

    final = phys >> shift << shift | flags;
    do {
        asm("str %0, [%1, %2, lsl #3]" :: "r" (final), "r" (table), "r" (offset));
        offset++;
        final+= SECTION_SIZE;
        // section is 2Mb (to ease things for now, non kernel memory will be probably map differently)
    } while(offset <= offset_end);
}

// will break at wfi (ttbr0_el1 is not set and therefore the whole code become accessible)
void init_mmu() {

    uint64_t sctlr;

    create_table_entry(PGD_TABLE, PUD_TABLE, VA_START, PGD_SHIFT);
    create_table_entry(PUD_TABLE, PMD_TABLE, VA_START, PUD_SHIFT);
    create_block_map(PMD_TABLE, 0ULL, MMU_FLAGS, VA_START, VA_START + DEVICE_BASE - SECTION_SIZE, PMD_SHIFT);
    create_block_map(PMD_TABLE, DEVICE_BASE, MMU_DEVICE_FLAGS, VA_START + DEVICE_BASE,
                     VA_START + PHYS_MEMORY_SIZE - SECTION_SIZE, PMD_SHIFT);

    asm volatile ("msr ttbr1_el1, %0" :: "r"(&PGD_TABLE));
    asm volatile ("msr tcr_el1, %0"  :: "r"(TCR_VALUE));
    asm volatile ("msr mair_el1, %0" :: "r"(MAIR_VALUE));

    asm volatile ("mrs %0, sctlr_el1" : "=r" (sctlr));
    asm volatile ("msr sctlr_el1, x0" :: "r"(sctlr | 1)); // activate MMU

    asm volatile ("wfi");
}