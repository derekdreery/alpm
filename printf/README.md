# printf

This library provides a single method that takes a format string and arg list
that would be used in functions like printf and returns a formatted string. It
is expected that the arg list be created in *C* code using `va_start`.

I created this lib specifically to help in wrapping c libraries that expect
printf-style function callbacks.
