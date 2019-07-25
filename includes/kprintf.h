/*
 * kprintf with gcc by default generate NEON / SMID instruction which are trapped in level 1
 * for now, we disable gcc option to use SMID instruction with -march=armv8-a+nofp
 */


int kprintf(char *fmt, ...);
int ksprintf(char *out, const char *format, ...);
