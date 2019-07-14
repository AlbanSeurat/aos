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

