use libc::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct alpm_list_t {
    pub data: *mut c_void,
    pub prev: *mut alpm_list_t,
    pub next: *mut alpm_list_t,
}

pub type alpm_list_fn_free = Option<unsafe extern "C" fn(arg1: *mut c_void)>;

pub type alpm_list_fn_cmp = Option<
    unsafe extern "C" fn(arg1: *const c_void, arg2: *const c_void) -> c_int,
>;

extern "C" {
    pub fn alpm_list_free(list: *mut alpm_list_t);

    pub fn alpm_list_free_inner(list: *mut alpm_list_t, fn_: alpm_list_fn_free);

    pub fn alpm_list_add(
        list: *mut alpm_list_t,
        data: *mut c_void,
    ) -> *mut alpm_list_t;

    pub fn alpm_list_append(
        list: *mut *mut alpm_list_t,
        data: *mut c_void,
    ) -> *mut alpm_list_t;

    pub fn alpm_list_append_strdup(
        list: *mut *mut alpm_list_t,
        data: *const c_char,
    ) -> *mut alpm_list_t;

    pub fn alpm_list_add_sorted(
        list: *mut alpm_list_t,
        data: *mut c_void,
        fn_: alpm_list_fn_cmp,
    ) -> *mut alpm_list_t;

    pub fn alpm_list_join(first: *mut alpm_list_t, second: *mut alpm_list_t) -> *mut alpm_list_t;

    pub fn alpm_list_mmerge(
        left: *mut alpm_list_t,
        right: *mut alpm_list_t,
        fn_: alpm_list_fn_cmp,
    ) -> *mut alpm_list_t;

    pub fn alpm_list_msort(
        list: *mut alpm_list_t,
        n: usize,
        fn_: alpm_list_fn_cmp,
    ) -> *mut alpm_list_t;

    pub fn alpm_list_remove_item(
        haystack: *mut alpm_list_t,
        item: *mut alpm_list_t,
    ) -> *mut alpm_list_t;

    pub fn alpm_list_remove(
        haystack: *mut alpm_list_t,
        needle: *const c_void,
        fn_: alpm_list_fn_cmp,
        data: *mut *mut c_void,
    ) -> *mut alpm_list_t;

    pub fn alpm_list_remove_str(
        haystack: *mut alpm_list_t,
        needle: *const c_char,
        data: *mut *mut c_char,
    ) -> *mut alpm_list_t;

    pub fn alpm_list_remove_dupes(list: *const alpm_list_t) -> *mut alpm_list_t;

    pub fn alpm_list_strdup(list: *const alpm_list_t) -> *mut alpm_list_t;

    pub fn alpm_list_copy(list: *const alpm_list_t) -> *mut alpm_list_t;

    pub fn alpm_list_copy_data(list: *const alpm_list_t, size: usize) -> *mut alpm_list_t;

    pub fn alpm_list_reverse(list: *mut alpm_list_t) -> *mut alpm_list_t;

    pub fn alpm_list_nth(list: *const alpm_list_t, n: usize) -> *mut alpm_list_t;

    pub fn alpm_list_next(list: *const alpm_list_t) -> *mut alpm_list_t;

    pub fn alpm_list_previous(list: *const alpm_list_t) -> *mut alpm_list_t;

    pub fn alpm_list_last(list: *const alpm_list_t) -> *mut alpm_list_t;

    pub fn alpm_list_count(list: *const alpm_list_t) -> usize;

    pub fn alpm_list_find(
        haystack: *const alpm_list_t,
        needle: *const c_void,
        fn_: alpm_list_fn_cmp,
    ) -> *mut c_void;

    pub fn alpm_list_find_ptr(
        haystack: *const alpm_list_t,
        needle: *const c_void,
    ) -> *mut c_void;

    pub fn alpm_list_find_str(
        haystack: *const alpm_list_t,
        needle: *const c_char,
    ) -> *mut c_char;

    pub fn alpm_list_diff(
        lhs: *const alpm_list_t,
        rhs: *const alpm_list_t,
        fn_: alpm_list_fn_cmp,
    ) -> *mut alpm_list_t;

    pub fn alpm_list_diff_sorted(
        left: *const alpm_list_t,
        right: *const alpm_list_t,
        fn_: alpm_list_fn_cmp,
        onlyleft: *mut *mut alpm_list_t,
        onlyright: *mut *mut alpm_list_t,
    );

    pub fn alpm_list_to_array(
        list: *const alpm_list_t,
        n: usize,
        size: usize,
    ) -> *mut c_void;
}
