use crate::include::stddef::*;
use crate::include::stdint::*;
use crate::src::align::{Align16, Align64};
use ::libc;
extern "C" {
    pub type Dav1dRef;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}


use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::headers::Dav1dTxfmMode;




use crate::include::dav1d::headers::Dav1dFilterMode;











use crate::include::dav1d::headers::Dav1dRestorationType;









use crate::include::dav1d::headers::Dav1dWarpedMotionParams;



use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::Dav1dFrameType;




















































use crate::include::dav1d::headers::DAV1D_MC_IDENTITY;




use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dSequenceHeader;





use crate::include::dav1d::headers::Dav1dFilmGrainData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader {
    pub film_grain: Dav1dFrameHeader_film_grain,
    pub frame_type: Dav1dFrameType,
    pub width: [libc::c_int; 2],
    pub height: libc::c_int,
    pub frame_offset: libc::c_int,
    pub temporal_id: libc::c_int,
    pub spatial_id: libc::c_int,
    pub show_existing_frame: libc::c_int,
    pub existing_frame_idx: libc::c_int,
    pub frame_id: libc::c_int,
    pub frame_presentation_delay: libc::c_int,
    pub show_frame: libc::c_int,
    pub showable_frame: libc::c_int,
    pub error_resilient_mode: libc::c_int,
    pub disable_cdf_update: libc::c_int,
    pub allow_screen_content_tools: libc::c_int,
    pub force_integer_mv: libc::c_int,
    pub frame_size_override: libc::c_int,
    pub primary_ref_frame: libc::c_int,
    pub buffer_removal_time_present: libc::c_int,
    pub operating_points: [Dav1dFrameHeaderOperatingPoint; 32],
    pub refresh_frame_flags: libc::c_int,
    pub render_width: libc::c_int,
    pub render_height: libc::c_int,
    pub super_res: Dav1dFrameHeader_super_res,
    pub have_render_size: libc::c_int,
    pub allow_intrabc: libc::c_int,
    pub frame_ref_short_signaling: libc::c_int,
    pub refidx: [libc::c_int; 7],
    pub hp: libc::c_int,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: libc::c_int,
    pub use_ref_frame_mvs: libc::c_int,
    pub refresh_context: libc::c_int,
    pub tiling: Dav1dFrameHeader_tiling,
    pub quant: Dav1dFrameHeader_quant,
    pub segmentation: Dav1dFrameHeader_segmentation,
    pub delta: Dav1dFrameHeader_delta,
    pub all_lossless: libc::c_int,
    pub loopfilter: Dav1dFrameHeader_loopfilter,
    pub cdef: Dav1dFrameHeader_cdef,
    pub restoration: Dav1dFrameHeader_restoration,
    pub txfm_mode: Dav1dTxfmMode,
    pub switchable_comp_refs: libc::c_int,
    pub skip_mode_allowed: libc::c_int,
    pub skip_mode_enabled: libc::c_int,
    pub skip_mode_refs: [libc::c_int; 2],
    pub warp_motion: libc::c_int,
    pub reduced_txtp_set: libc::c_int,
    pub gmv: [Dav1dWarpedMotionParams; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_restoration {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [libc::c_int; 2],
}
use crate::include::dav1d::headers::Dav1dFrameHeader_cdef;
use crate::include::dav1d::headers::Dav1dFrameHeader_loopfilter;
use crate::include::dav1d::headers::Dav1dFrameHeader_delta;


use crate::include::dav1d::headers::Dav1dFrameHeader_segmentation;
use crate::include::dav1d::headers::Dav1dFrameHeader_quant;
use crate::include::dav1d::headers::Dav1dFrameHeader_tiling;
use crate::include::dav1d::headers::Dav1dFrameHeader_super_res;
use crate::include::dav1d::headers::Dav1dFrameHeaderOperatingPoint;
use crate::include::dav1d::headers::Dav1dFrameHeader_film_grain;
use crate::include::dav1d::picture::Dav1dPictureParameters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPicture {
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub data: [*mut libc::c_void; 3],
    pub stride: [ptrdiff_t; 2],
    pub p: Dav1dPictureParameters,
    pub m: Dav1dDataProps,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35: *mut Dav1dITUTT35,
    pub reserved: [uintptr_t; 4],
    pub frame_hdr_ref: *mut Dav1dRef,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub content_light_ref: *mut Dav1dRef,
    pub mastering_display_ref: *mut Dav1dRef,
    pub itut_t35_ref: *mut Dav1dRef,
    pub reserved_ref: [uintptr_t; 4],
    pub ref_0: *mut Dav1dRef,
    pub allocator_data: *mut libc::c_void,
}
pub type pixel = uint16_t;
pub type entry = int16_t;
pub type generate_grain_y_fn = Option::<
    unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData, libc::c_int) -> (),
>;
pub type generate_grain_uv_fn = Option::<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
        libc::c_int,
    ) -> (),
>;
pub type fgy_32x32xn_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        *const pixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type fguv_32x32xn_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        *const pixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y_fn,
    pub generate_grain_uv: [generate_grain_uv_fn; 3],
    pub fgy_32x32xn: fgy_32x32xn_fn,
    pub fguv_32x32xn: [fguv_32x32xn_fn; 3],
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1 as libc::c_int;
}
unsafe extern "C" fn generate_scaling(
    bitdepth: libc::c_int,
    mut points: *const [uint8_t; 2],
    num: libc::c_int,
    mut scaling: *mut uint8_t,
) {
    if !(bitdepth > 8 as libc::c_int) {
        unreachable!();
    }
    let shift_x: libc::c_int = bitdepth - 8 as libc::c_int;
    let scaling_size: libc::c_int = (1 as libc::c_int) << bitdepth;
    if num == 0 as libc::c_int {
        memset(
            scaling as *mut libc::c_void,
            0 as libc::c_int,
            scaling_size as libc::c_ulong,
        );
        return;
    }
    memset(
        scaling as *mut libc::c_void,
        (*points.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            as libc::c_int,
        (((*points.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            as libc::c_int) << shift_x) as libc::c_ulong,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num - 1 as libc::c_int {
        let bx: libc::c_int = (*points.offset(i as isize))[0 as libc::c_int as usize]
            as libc::c_int;
        let by: libc::c_int = (*points.offset(i as isize))[1 as libc::c_int as usize]
            as libc::c_int;
        let ex: libc::c_int = (*points
            .offset((i + 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
            as libc::c_int;
        let ey: libc::c_int = (*points
            .offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
            as libc::c_int;
        let dx: libc::c_int = ex - bx;
        let dy: libc::c_int = ey - by;
        if !(dx > 0 as libc::c_int) {
            unreachable!();
        }
        let delta: libc::c_int = dy
            * ((0x10000 as libc::c_int + (dx >> 1 as libc::c_int)) / dx);
        let mut x: libc::c_int = 0 as libc::c_int;
        let mut d: libc::c_int = 0x8000 as libc::c_int;
        while x < dx {
            *scaling
                .offset(
                    (bx + x << shift_x) as isize,
                ) = (by + (d >> 16 as libc::c_int)) as uint8_t;
            d += delta;
            x += 1;
        }
        i += 1;
    }
    let n: libc::c_int = ((*points
        .offset((num - 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
        as libc::c_int) << shift_x;
    memset(
        &mut *scaling.offset(n as isize) as *mut uint8_t as *mut libc::c_void,
        (*points.offset((num - 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
            as libc::c_int,
        (scaling_size - n) as libc::c_ulong,
    );
    let pad: libc::c_int = (1 as libc::c_int) << shift_x;
    let rnd: libc::c_int = pad >> 1 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < num - 1 as libc::c_int {
        let bx_0: libc::c_int = ((*points
            .offset(i_0 as isize))[0 as libc::c_int as usize] as libc::c_int) << shift_x;
        let ex_0: libc::c_int = ((*points
            .offset((i_0 + 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
            as libc::c_int) << shift_x;
        let dx_0: libc::c_int = ex_0 - bx_0;
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < dx_0 {
            let range: libc::c_int = *scaling.offset((bx_0 + x_0 + pad) as isize)
                as libc::c_int - *scaling.offset((bx_0 + x_0) as isize) as libc::c_int;
            let mut n_0: libc::c_int = 1 as libc::c_int;
            let mut r: libc::c_int = rnd;
            while n_0 < pad {
                r += range;
                *scaling
                    .offset(
                        (bx_0 + x_0 + n_0) as isize,
                    ) = (*scaling.offset((bx_0 + x_0) as isize) as libc::c_int
                    + (r >> shift_x)) as uint8_t;
                n_0 += 1;
            }
            x_0 += pad;
        }
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_prep_grain_16bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
    mut scaling: *mut [uint8_t; 4096],
    mut grain_lut: *mut [[entry; 82]; 74],
) {
    let data: *const Dav1dFilmGrainData = &mut (*(*out).frame_hdr).film_grain.data;
    let bitdepth_max: libc::c_int = ((1 as libc::c_int) << (*out).p.bpc)
        - 1 as libc::c_int;
    ((*dsp).generate_grain_y)
        .expect(
            "non-null function pointer",
        )(
        (*grain_lut.offset(0 as libc::c_int as isize)).as_mut_ptr(),
        data,
        bitdepth_max,
    );
    if (*data).num_uv_points[0 as libc::c_int as usize] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        ((*dsp)
            .generate_grain_uv[((*in_0).p.layout as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
            .expect(
                "non-null function pointer",
            )(
            (*grain_lut.offset(1 as libc::c_int as isize)).as_mut_ptr(),
            (*grain_lut.offset(0 as libc::c_int as isize)).as_mut_ptr()
                as *const [entry; 82],
            data,
            0 as libc::c_int as intptr_t,
            bitdepth_max,
        );
    }
    if (*data).num_uv_points[1 as libc::c_int as usize] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        ((*dsp)
            .generate_grain_uv[((*in_0).p.layout as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
            .expect(
                "non-null function pointer",
            )(
            (*grain_lut.offset(2 as libc::c_int as isize)).as_mut_ptr(),
            (*grain_lut.offset(0 as libc::c_int as isize)).as_mut_ptr()
                as *const [entry; 82],
            data,
            1 as libc::c_int as intptr_t,
            bitdepth_max,
        );
    }
    if (*data).num_y_points != 0 || (*data).chroma_scaling_from_luma != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).y_points).as_ptr(),
            (*data).num_y_points,
            (*scaling.offset(0 as libc::c_int as isize)).as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[0 as libc::c_int as usize] != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).uv_points[0 as libc::c_int as usize]).as_ptr(),
            (*data).num_uv_points[0 as libc::c_int as usize],
            (*scaling.offset(1 as libc::c_int as isize)).as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[1 as libc::c_int as usize] != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).uv_points[1 as libc::c_int as usize]).as_ptr(),
            (*data).num_uv_points[1 as libc::c_int as usize],
            (*scaling.offset(2 as libc::c_int as isize)).as_mut_ptr(),
        );
    }
    if !((*out).stride[0 as libc::c_int as usize]
        == (*in_0).stride[0 as libc::c_int as usize])
    {
        unreachable!();
    }
    if (*data).num_y_points == 0 {
        let stride: ptrdiff_t = (*out).stride[0 as libc::c_int as usize];
        let sz: ptrdiff_t = (*out).p.h as isize * stride;
        if sz < 0 {
            memcpy(
                ((*out).data[0 as libc::c_int as usize] as *mut uint8_t)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *mut libc::c_void,
                ((*in_0).data[0 as libc::c_int as usize] as *mut uint8_t)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *const libc::c_void,
                -sz as libc::c_ulong,
            );
        } else {
            memcpy(
                (*out).data[0 as libc::c_int as usize],
                (*in_0).data[0 as libc::c_int as usize],
                sz as libc::c_ulong,
            );
        }
    }
    if (*in_0).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        && (*data).chroma_scaling_from_luma == 0
    {
        if !((*out).stride[1 as libc::c_int as usize]
            == (*in_0).stride[1 as libc::c_int as usize])
        {
            unreachable!();
        }
        let ss_ver: libc::c_int = ((*in_0).p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
        let stride_0: ptrdiff_t = (*out).stride[1 as libc::c_int as usize];
        let sz_0: ptrdiff_t = ((*out).p.h + ss_ver >> ss_ver) as isize * stride_0;
        if sz_0 < 0 {
            if (*data).num_uv_points[0 as libc::c_int as usize] == 0 {
                memcpy(
                    ((*out).data[1 as libc::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *mut libc::c_void,
                    ((*in_0).data[1 as libc::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *const libc::c_void,
                    -sz_0 as libc::c_ulong,
                );
            }
            if (*data).num_uv_points[1 as libc::c_int as usize] == 0 {
                memcpy(
                    ((*out).data[2 as libc::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *mut libc::c_void,
                    ((*in_0).data[2 as libc::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *const libc::c_void,
                    -sz_0 as libc::c_ulong,
                );
            }
        } else {
            if (*data).num_uv_points[0 as libc::c_int as usize] == 0 {
                memcpy(
                    (*out).data[1 as libc::c_int as usize],
                    (*in_0).data[1 as libc::c_int as usize],
                    sz_0 as libc::c_ulong,
                );
            }
            if (*data).num_uv_points[1 as libc::c_int as usize] == 0 {
                memcpy(
                    (*out).data[2 as libc::c_int as usize],
                    (*in_0).data[2 as libc::c_int as usize],
                    sz_0 as libc::c_ulong,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain_row_16bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
    mut scaling: *const [uint8_t; 4096],
    mut grain_lut: *const [[entry; 82]; 74],
    row: libc::c_int,
) {
    let data: *const Dav1dFilmGrainData = &mut (*(*out).frame_hdr).film_grain.data;
    let ss_y: libc::c_int = ((*in_0).p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_x: libc::c_int = ((*in_0).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let cpw: libc::c_int = (*out).p.w + ss_x >> ss_x;
    let is_id: libc::c_int = ((*(*out).seq_hdr).mtrx as libc::c_uint
        == DAV1D_MC_IDENTITY as libc::c_int as libc::c_uint) as libc::c_int;
    let luma_src: *mut pixel = ((*in_0).data[0 as libc::c_int as usize] as *mut pixel)
        .offset(
            ((row * 32 as libc::c_int) as isize
                * PXSTRIDE((*in_0).stride[0 as libc::c_int as usize])) as isize,
        );
    let bitdepth_max: libc::c_int = ((1 as libc::c_int) << (*out).p.bpc)
        - 1 as libc::c_int;
    if (*data).num_y_points != 0 {
        let bh: libc::c_int = imin(
            (*out).p.h - row * 32 as libc::c_int,
            32 as libc::c_int,
        );
        ((*dsp).fgy_32x32xn)
            .expect(
                "non-null function pointer",
            )(
            ((*out).data[0 as libc::c_int as usize] as *mut pixel)
                .offset(
                    ((row * 32 as libc::c_int) as isize
                        * PXSTRIDE((*out).stride[0 as libc::c_int as usize])) as isize,
                ),
            luma_src,
            (*out).stride[0 as libc::c_int as usize],
            data,
            (*out).p.w as size_t,
            (*scaling.offset(0 as libc::c_int as isize)).as_ptr(),
            (*grain_lut.offset(0 as libc::c_int as isize)).as_ptr(),
            bh,
            row,
            bitdepth_max,
        );
    }
    if (*data).num_uv_points[0 as libc::c_int as usize] == 0
        && (*data).num_uv_points[1 as libc::c_int as usize] == 0
        && (*data).chroma_scaling_from_luma == 0
    {
        return;
    }
    let bh_0: libc::c_int = imin((*out).p.h - row * 32 as libc::c_int, 32 as libc::c_int)
        + ss_y >> ss_y;
    if (*out).p.w & ss_x != 0 {
        let mut ptr: *mut pixel = luma_src;
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < bh_0 {
            *ptr
                .offset(
                    (*out).p.w as isize,
                ) = *ptr.offset(((*out).p.w - 1 as libc::c_int) as isize);
            ptr = ptr
                .offset(
                    (PXSTRIDE((*in_0).stride[0 as libc::c_int as usize]) << ss_y)
                        as isize,
                );
            y += 1;
        }
    }
    let uv_off: ptrdiff_t = (row * 32 as libc::c_int) as isize * PXSTRIDE((*out).stride[1 as libc::c_int as usize]) >> ss_y;
    if (*data).chroma_scaling_from_luma != 0 {
        let mut pl: libc::c_int = 0 as libc::c_int;
        while pl < 2 as libc::c_int {
            ((*dsp)
                .fguv_32x32xn[((*in_0).p.layout as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
                .expect(
                    "non-null function pointer",
                )(
                ((*out).data[(1 as libc::c_int + pl) as usize] as *mut pixel)
                    .offset(uv_off as isize),
                ((*in_0).data[(1 as libc::c_int + pl) as usize] as *const pixel)
                    .offset(uv_off as isize),
                (*in_0).stride[1 as libc::c_int as usize],
                data,
                cpw as size_t,
                (*scaling.offset(0 as libc::c_int as isize)).as_ptr(),
                (*grain_lut.offset((1 as libc::c_int + pl) as isize)).as_ptr(),
                bh_0,
                row,
                luma_src,
                (*in_0).stride[0 as libc::c_int as usize],
                pl,
                is_id,
                bitdepth_max,
            );
            pl += 1;
        }
    } else {
        let mut pl_0: libc::c_int = 0 as libc::c_int;
        while pl_0 < 2 as libc::c_int {
            if (*data).num_uv_points[pl_0 as usize] != 0 {
                ((*dsp)
                    .fguv_32x32xn[((*in_0).p.layout as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*out).data[(1 as libc::c_int + pl_0) as usize] as *mut pixel)
                        .offset(uv_off as isize),
                    ((*in_0).data[(1 as libc::c_int + pl_0) as usize] as *const pixel)
                        .offset(uv_off as isize),
                    (*in_0).stride[1 as libc::c_int as usize],
                    data,
                    cpw as size_t,
                    (*scaling.offset((1 as libc::c_int + pl_0) as isize)).as_ptr(),
                    (*grain_lut.offset((1 as libc::c_int + pl_0) as isize)).as_ptr(),
                    bh_0,
                    row,
                    luma_src,
                    (*in_0).stride[0 as libc::c_int as usize],
                    pl_0,
                    is_id,
                    bitdepth_max,
                );
            }
            pl_0 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain_16bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) {
    let mut grain_lut = Align16([[[0; 82]; 74]; 3]);
    let mut scaling = Align64([[0; 4096]; 3]);
    let rows: libc::c_int = (*out).p.h + 31 as libc::c_int >> 5 as libc::c_int;
    dav1d_prep_grain_16bpc(dsp, out, in_0, scaling.0.as_mut_ptr(), grain_lut.0.as_mut_ptr());
    let mut row: libc::c_int = 0 as libc::c_int;
    while row < rows {
        dav1d_apply_grain_row_16bpc(
            dsp,
            out,
            in_0,
            scaling.0.as_mut_ptr() as *const [uint8_t; 4096],
            grain_lut.0.as_mut_ptr() as *const [[entry; 82]; 74],
            row,
        );
        row += 1;
    }
}
