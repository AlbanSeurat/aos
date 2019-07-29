#include <stddef.h>
#include <stdint.h>

// Memory-Mapped I/O output
static inline void mmio_write(volatile uint32_t* reg, uint32_t data)
{
    *reg = data;
}

// Memory-Mapped I/O input
static inline uint32_t mmio_read(volatile uint32_t* reg)
{
    return *reg;
}

// Loop <delay> times in a way that the compiler won't optimize away
static inline void delay(int32_t count)
{
    for(int i = 0 ; i < count ; i++) {
        asm volatile("nop");
    }
}

// naive version should be replaced by NEON optimized
static inline void* memcpy(void* destination, void* source, size_t num)
{
    size_t i;
    char* d = destination;
    char* s = source;
    for (i = 0; i < num; i++) {
        d[i] = s[i];
    }
    return destination;
}

typedef struct _file {
    void * position;
    void * start;
    long size;
} FILE;

FILE * open(void * address, long size);
long read(FILE * file, void * buf, int len);
long skip(FILE * file, int len);
void close(FILE * file);

/* POSIX ustar header format */
typedef struct {                /* byte offset */
    char name[100];               /*   0 */
    char mode[8];                 /* 100 */
    char uid[8];                  /* 108 */
    char gid[8];                  /* 116 */
    char size[12];                /* 124 */
    char mtime[12];               /* 136 */
    char chksum[8];               /* 148 */
    char typeflag;                /* 156 */
    char linkname[100];           /* 157 */
    char magic[6];                /* 257 */
    char version[2];              /* 263 */
    char uname[32];               /* 265 */
    char gname[32];               /* 297 */
    char devmajor[8];             /* 329 */
    char devminor[8];             /* 337 */
    char prefix[167];             /* 345 */
} __attribute__((packed)) tar_t;

void read_tar(void*, size_t elem);

void init_mmu();