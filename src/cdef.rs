use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow2px;
use crate::include::common::intops::apply_sign;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type CdefEdgeFlags = c_uint;
pub const CDEF_HAVE_BOTTOM: CdefEdgeFlags = 8;
pub const CDEF_HAVE_TOP: CdefEdgeFlags = 4;
pub const CDEF_HAVE_RIGHT: CdefEdgeFlags = 2;
pub const CDEF_HAVE_LEFT: CdefEdgeFlags = 1;

pub type cdef_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const LeftPixelRow2px<DynPixel>,
    *const DynPixel,
    *const DynPixel,
    c_int,
    c_int,
    c_int,
    c_int,
    CdefEdgeFlags,
    c_int,
) -> ();

pub type cdef_dir_fn =
    unsafe extern "C" fn(*const DynPixel, ptrdiff_t, *mut c_uint, c_int) -> c_int;

#[repr(C)]
pub struct Rav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_sse4(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_ssse3(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(feature = "asm", feature = "bitdepth_8", target_arch = "x86_64",))]
extern "C" {
    pub(crate) fn dav1d_cdef_dir_8bpc_avx2(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_find_dir_8bpc_neon(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_padding4_8bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_padding8_8bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter4_8bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
    );
    pub(crate) fn dav1d_cdef_filter8_8bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_dir_16bpc_sse4(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_16bpc_ssse3(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(feature = "asm", feature = "bitdepth_16", target_arch = "x86_64",))]
extern "C" {
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_16bpc_avx2(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_find_dir_16bpc_neon(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_padding4_16bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_padding8_16bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter4_16bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter8_16bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
        bitdepth_max: c_int,
    );
}

#[inline]
pub unsafe fn constrain(diff: c_int, threshold: c_int, shift: c_int) -> c_int {
    let adiff = diff.abs();
    return apply_sign(
        cmp::min(adiff, cmp::max(0 as c_int, threshold - (adiff >> shift))),
        diff,
    );
}

#[inline]
pub unsafe fn fill(mut tmp: *mut i16, stride: ptrdiff_t, w: c_int, h: c_int) {
    let mut y = 0;
    while y < h {
        let mut x = 0;
        while x < w {
            *tmp.offset(x as isize) = i16::MIN;
            x += 1;
        }
        tmp = tmp.offset(stride as isize);
        y += 1;
    }
}
