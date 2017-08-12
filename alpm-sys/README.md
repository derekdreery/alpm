# Alpm-sys

Thin wrapper around "alpm.h"

> NOTE the enums are wrong at the moment. You can't just use a constant because
> how the C compiler chooses a size for an enum is undefined. This is UB.
