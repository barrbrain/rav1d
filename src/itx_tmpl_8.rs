use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip_u8;
use crate::src::itx::Rav1dInvTxfmDSPContext;
use crate::src::levels::ADST_ADST;
use crate::src::levels::ADST_DCT;
use crate::src::levels::ADST_FLIPADST;
use crate::src::levels::DCT_ADST;
use crate::src::levels::DCT_DCT;
use crate::src::levels::DCT_FLIPADST;
use crate::src::levels::FLIPADST_ADST;
use crate::src::levels::FLIPADST_DCT;
use crate::src::levels::FLIPADST_FLIPADST;
use crate::src::levels::H_ADST;
use crate::src::levels::H_DCT;
use crate::src::levels::H_FLIPADST;
use crate::src::levels::IDTX;
use crate::src::levels::RTX_16X32;
use crate::src::levels::RTX_16X4;
use crate::src::levels::RTX_16X64;
use crate::src::levels::RTX_16X8;
use crate::src::levels::RTX_32X16;
use crate::src::levels::RTX_32X64;
use crate::src::levels::RTX_32X8;
use crate::src::levels::RTX_4X16;
use crate::src::levels::RTX_4X8;
use crate::src::levels::RTX_64X16;
use crate::src::levels::RTX_64X32;
use crate::src::levels::RTX_8X16;
use crate::src::levels::RTX_8X32;
use crate::src::levels::RTX_8X4;
use crate::src::levels::TX_16X16;
use crate::src::levels::TX_32X32;
use crate::src::levels::TX_4X4;
use crate::src::levels::TX_64X64;
use crate::src::levels::TX_8X8;
use crate::src::levels::V_ADST;
use crate::src::levels::V_DCT;
use crate::src::levels::V_FLIPADST;
use crate::src::levels::WHT_WHT;
use libc::memset;
use libc::ptrdiff_t;
use std::ffi::c_int;
use std::ffi::c_void;

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

pub type pixel = u8;
pub type coef = i16;

unsafe extern "C" fn inv_txfm_add_wht_wht_4x4_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    coeff: *mut DynCoef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    inv_txfm_add_wht_wht_4x4_rust(dst.cast(), stride, coeff.cast(), eob);
}

unsafe fn inv_txfm_add_wht_wht_4x4_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    _eob: c_int,
) {
    use crate::src::itx_1d::dav1d_inv_wht4_1d_c;

    let mut tmp: [i32; 16] = [0; 16];
    let mut c: *mut i32 = tmp.as_mut_ptr();
    let mut y = 0;
    while y < 4 {
        let mut x = 0;
        while x < 4 {
            *c.offset(x as isize) = *coeff.offset((y + x * 4) as isize) as c_int >> 2;
            x += 1;
        }
        dav1d_inv_wht4_1d_c(c, 1 as c_int as ptrdiff_t);
        y += 1;
        c = c.offset(4);
    }
    memset(
        coeff as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<coef>()
            .wrapping_mul(4)
            .wrapping_mul(4),
    );
    let mut x_0 = 0;
    while x_0 < 4 {
        dav1d_inv_wht4_1d_c(
            &mut *tmp.as_mut_ptr().offset(x_0 as isize),
            4 as c_int as ptrdiff_t,
        );
        x_0 += 1;
    }
    c = tmp.as_mut_ptr();
    let mut y_0 = 0;
    while y_0 < 4 {
        let mut x_1 = 0;
        while x_1 < 4 {
            let fresh1 = c;
            c = c.offset(1);
            *dst.offset(x_1 as isize) =
                iclip_u8(*dst.offset(x_1 as isize) as c_int + *fresh1) as pixel;
            x_1 += 1;
        }
        y_0 += 1;
        dst = dst.offset(stride as isize);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
#[rustfmt::skip]
unsafe fn itx_dsp_init_x86(c: *mut Rav1dInvTxfmDSPContext, _bpc: c_int) {
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::itx::*;

    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSE2) {
        return;
    }

    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(dav1d_inv_txfm_add_wht_wht_4x4_8bpc_sse2);

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_8bpc_ssse3);
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_8bpc_ssse3);
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_8bpc_ssse3);
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_ssse3);
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_ssse3);
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_ssse3);
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_ssse3);
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_ssse3);

    if !flags.contains(CpuFlags::SSE41) {
        return;
    }

    #[cfg(target_arch = "x86_64")]
    {
        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(dav1d_inv_txfm_add_wht_wht_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_avx2);
        (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_8bpc_avx2);
        (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_avx2);
        (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_8bpc_avx2);
        (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_avx2);
        (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_avx2);
        (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_8bpc_avx2);
        (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_avx2);
        (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_8bpc_avx2);
        (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_avx2);
        (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_8bpc_avx2);
        (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_avx2);
        (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_avx2);
        (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_avx2);
        (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_avx2);

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_avx512icl);
        (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_avx512icl);
        (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_8bpc_avx512icl);
        (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_avx512icl);
        (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_avx512icl);
        (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_avx512icl);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
#[rustfmt::skip]
unsafe fn itx_dsp_init_arm(c: *mut Rav1dInvTxfmDSPContext, mut _bpc: c_int) {
        // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::itx::*;

    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(dav1d_inv_txfm_add_wht_wht_4x4_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_neon);
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_8bpc_neon);
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_neon);
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_8bpc_neon);
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_neon);
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_neon);
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_8bpc_neon);
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_neon);
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_8bpc_neon);
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_neon);
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_8bpc_neon);
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_neon);
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_neon);
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_neon);
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_neon);
}

#[cold]
#[rustfmt::skip]
pub unsafe fn rav1d_itx_dsp_init_8bpc(c: *mut Rav1dInvTxfmDSPContext, mut _bpc: c_int) {
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::itx::*;

    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(inv_txfm_add_wht_wht_4x4_c_erased);
    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_4x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_4x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_4x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_8x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_8x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_8x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_8x32_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_8x32_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_16x4_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_16x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_16x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_16x32_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_16x32_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_16x64_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_32x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_32x8_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_32x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_32x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_32x32_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_32x32_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_32x64_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_64x16_c_erased::<BitDepth8>);
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_64x32_c_erased::<BitDepth8>);
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_64x64_c_erased::<BitDepth8>);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            itx_dsp_init_x86(c, _bpc);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            itx_dsp_init_arm(c, _bpc);
        }
    }
}
