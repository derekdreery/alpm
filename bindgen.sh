#!/usr/bin/env bash
bindgen /usr/include/alpm.h \
    --whitelist-type '_*(alpm|ALPM).*' \
    --whitelist-function '_*(alpm|ALPM).*' \
    --rustified-enum '_alpm_[a-z]+_t' \
    --no-layout-tests \
    --opaque-type alpm_handle_t \
    --opaque-type alpm_db_t \
    --opaque-type alpm_pkg_t \
    --opaque-type alpm_trans_t \
    --time-phases
