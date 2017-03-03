#include <stdarg.h>
#include <stdlib.h>

static char* format1 = "testing printf format: %d\n";
static char* format2 = "Characters: %c %c";
static char* format3 = "Decimals: %d %ld";
static char* format4 = "Preceding with blanks: %10d";
static char* format5 = "Preceding with zeros: %010d";
static char* format6 = "Some different radices: %d %x %o %#x %#o";
static char* format7 = "floats: %4.2f %+.0e %E";
static char* format8 = "Width trick: %*d";
static char* format9 = "%s";

/* turns a variadic set of arguments into a va_list */
__attribute__ (( format( printf, 2, 3 ) ))
void* test_printf(char* (*cb)(char*, va_list), char* format, ...)
{
    void* out;
    va_list args1;

    va_start(args1, format);
    out = cb(format, args1);
    va_end(args1);
    return out;
}

char* dispatch(int test_no, char* (*cb)(char*, va_list))
{
    switch (test_no) {
    case 1:
        return test_printf(cb, format1, 1);
    case 2:
        return test_printf(cb, format2, 'a', 65);
    case 3:
        return test_printf(cb, format3, 1977, 650000L);
    case 4:
        return test_printf(cb, format4, 1977);
    case 5:
        return test_printf(cb, format5, 1977);
    case 6:
        return test_printf(cb, format6, 100, 100, 100, 100, 100);
    case 7:
        return test_printf(cb, format7, 3.1416, 3.1416, 3.1416);
    case 8:
        return test_printf(cb, format8, 5, 10);
    case 9:
        return test_printf(cb, format9, "A string");
    default:
        return "";
    }
}
