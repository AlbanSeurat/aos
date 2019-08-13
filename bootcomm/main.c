#include <stdio.h>
#include <sys/stat.h>

#ifdef __APPLE__
#include <libkern/OSByteOrder.h>
#define htole32(x) OSSwapHostToLittleInt32(x)
#else
#include <endian.h>
#endif

int main(int argc, char *argv[]) {

    struct stat buffer;
    int         status;

    status = stat(argv[1], &buffer);
    if(status == 0) {
        int size = htole32(buffer.st_size);;

        // send kernel size to RPi
        int ch;

        fwrite(&size, 4, 1, stdout);

        FILE * kernel = fopen(argv[1], "r+");

        while ((ch = fgetc(kernel)) != EOF) {
            fputc(ch, stdout);
        }

        fflush(stdout);
    }

    return 0;
}