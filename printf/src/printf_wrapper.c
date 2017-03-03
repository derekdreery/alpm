
#include <stdlib.h>
#include <stdio.h>
#include <stdarg.h>

char* printf_wrapper(char* format, va_list args) {
    va_list cpy;
    va_copy(cpy, args);
    int size = vsnprintf(0, 0, format, cpy);
    char* out = malloc(size + 1);
    vsprintf(out, format, args);
    va_end(cpy);
    return out;
}
