# Alpm-sys

Thin wrapper around "alpm.h"

> NOTE the enums are wrong at the moment. You can't just use a constant because
> how the C compiler chooses a size for an enum is undefined. This is UB.
>
> **EDIT** after reading the spec, it seems that enums do not have a fixed ABI 
> representation, so it is not possible to use them safely. In this case,
> constants are the best approach (most of the time they are either u32, or i32 
> if there are negative values).
