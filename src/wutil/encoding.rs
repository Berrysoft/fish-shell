#[cfg(not(target_os = "cygwin"))]
extern "C" {
    pub fn wcrtomb(s: *mut libc::c_char, wc: libc::wchar_t, ps: *mut mbstate_t) -> usize;
    pub fn mbrtowc(
        pwc: *mut libc::wchar_t,
        s: *const libc::c_char,
        n: usize,
        p: *mut mbstate_t,
    ) -> usize;
}

// HACK This should be mbstate_t from libc but that's not exposed.  Since it's only written by
// libc, we define it as opaque type that should be large enough for all implementations.
pub type mbstate_t = [u64; 16];
pub fn zero_mbstate() -> mbstate_t {
    [0; 16]
}

// HACK This should be the MB_LEN_MAX macro from libc but that's not easy to get.
pub const AT_LEAST_MB_LEN_MAX: usize = 32;

#[cfg(target_os = "cygwin")]
pub unsafe fn wcrtomb(s: *mut libc::c_char, wc: char, _ps: *mut mbstate_t) -> usize {
    let slice = if s.is_null() {
        &mut [0u8; AT_LEAST_MB_LEN_MAX]
    } else {
        unsafe { core::slice::from_raw_parts_mut(s.cast(), AT_LEAST_MB_LEN_MAX) }
    };
    wc.encode_utf8(slice).len()
}

#[cfg(target_os = "cygwin")]
pub unsafe fn mbrtowc(
    pwc: *mut char,
    s: *const libc::c_char,
    n: usize,
    _p: *mut mbstate_t,
) -> usize {
    if n == 0 {
        return 0;
    }
    if s.is_null() {
        return 0;
    }
    let s = core::slice::from_raw_parts(s.cast::<u8>(), n);
    let max_len = s.len().min(4);
    for len in 1..=max_len {
        if let Ok(st) = std::str::from_utf8(&s[..len]) {
            let c = st.chars().next().unwrap();
            *pwc = c;
            if c == '\0' {
                return 0;
            } else {
                return len;
            }
        }
    }
    if max_len >= 4 {
        0_usize.wrapping_sub(1)
    } else {
        0_usize.wrapping_sub(2)
    }
}
