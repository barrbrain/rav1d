use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::stderr;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Dav1dRef;
    pub type MuxerPriv;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static null_muxer: Muxer;
    static md5_muxer: Muxer;
    static yuv_muxer: Muxer;
    static y4m2_muxer: Muxer;
}

pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
use crate::include::dav1d::common::Dav1dUserData;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::headers::Dav1dTxfmMode;
use crate::include::dav1d::headers::DAV1D_N_TX_MODES;
use crate::include::dav1d::headers::DAV1D_TX_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_TX_LARGEST;
use crate::include::dav1d::headers::DAV1D_TX_4X4_ONLY;
use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::include::dav1d::headers::DAV1D_FILTER_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_N_FILTERS;
use crate::include::dav1d::headers::DAV1D_FILTER_BILINEAR;
use crate::include::dav1d::headers::DAV1D_N_SWITCHABLE_FILTERS;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SHARP;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SMOOTH;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_REGULAR;
use crate::include::dav1d::headers::Dav1dAdaptiveBoolean;
use crate::include::dav1d::headers::DAV1D_ADAPTIVE;
use crate::include::dav1d::headers::DAV1D_ON;
use crate::include::dav1d::headers::DAV1D_OFF;
use crate::include::dav1d::headers::Dav1dRestorationType;
use crate::include::dav1d::headers::DAV1D_RESTORATION_SGRPROJ;
use crate::include::dav1d::headers::DAV1D_RESTORATION_WIENER;
use crate::include::dav1d::headers::DAV1D_RESTORATION_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_RESTORATION_NONE;
use crate::include::dav1d::headers::Dav1dWarpedMotionType;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_AFFINE;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_ROT_ZOOM;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_TRANSLATION;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_IDENTITY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub p: C2RustUnnamed_0,
    pub abcd: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
}
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::Dav1dFrameType;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_SWITCH;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_INTRA;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_INTER;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_KEY;
use crate::include::dav1d::headers::Dav1dColorPrimaries;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_RESERVED;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_EBU3213;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_SMPTE432;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_SMPTE431;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_XYZ;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT2020;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_FILM;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_SMPTE240;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT601;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT470BG;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT470M;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_UNKNOWN;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT709;
use crate::include::dav1d::headers::Dav1dTransferCharacteristics;
use crate::include::dav1d::headers::DAV1D_TRC_RESERVED;
use crate::include::dav1d::headers::DAV1D_TRC_HLG;
use crate::include::dav1d::headers::DAV1D_TRC_SMPTE428;
use crate::include::dav1d::headers::DAV1D_TRC_SMPTE2084;
use crate::include::dav1d::headers::DAV1D_TRC_BT2020_12BIT;
use crate::include::dav1d::headers::DAV1D_TRC_BT2020_10BIT;
use crate::include::dav1d::headers::DAV1D_TRC_SRGB;
use crate::include::dav1d::headers::DAV1D_TRC_BT1361;
use crate::include::dav1d::headers::DAV1D_TRC_IEC61966;
use crate::include::dav1d::headers::DAV1D_TRC_LOG100_SQRT10;
use crate::include::dav1d::headers::DAV1D_TRC_LOG100;
use crate::include::dav1d::headers::DAV1D_TRC_LINEAR;
use crate::include::dav1d::headers::DAV1D_TRC_SMPTE240;
use crate::include::dav1d::headers::DAV1D_TRC_BT601;
use crate::include::dav1d::headers::DAV1D_TRC_BT470BG;
use crate::include::dav1d::headers::DAV1D_TRC_BT470M;
use crate::include::dav1d::headers::DAV1D_TRC_UNKNOWN;
use crate::include::dav1d::headers::DAV1D_TRC_BT709;
use crate::include::dav1d::headers::Dav1dMatrixCoefficients;
use crate::include::dav1d::headers::DAV1D_MC_RESERVED;
use crate::include::dav1d::headers::DAV1D_MC_ICTCP;
use crate::include::dav1d::headers::DAV1D_MC_CHROMAT_CL;
use crate::include::dav1d::headers::DAV1D_MC_CHROMAT_NCL;
use crate::include::dav1d::headers::DAV1D_MC_SMPTE2085;
use crate::include::dav1d::headers::DAV1D_MC_BT2020_CL;
use crate::include::dav1d::headers::DAV1D_MC_BT2020_NCL;
use crate::include::dav1d::headers::DAV1D_MC_SMPTE_YCGCO;
use crate::include::dav1d::headers::DAV1D_MC_SMPTE240;
use crate::include::dav1d::headers::DAV1D_MC_BT601;
use crate::include::dav1d::headers::DAV1D_MC_BT470BG;
use crate::include::dav1d::headers::DAV1D_MC_FCC;
use crate::include::dav1d::headers::DAV1D_MC_UNKNOWN;
use crate::include::dav1d::headers::DAV1D_MC_BT709;
use crate::include::dav1d::headers::DAV1D_MC_IDENTITY;
use crate::include::dav1d::headers::Dav1dChromaSamplePosition;
use crate::include::dav1d::headers::DAV1D_CHR_COLOCATED;
use crate::include::dav1d::headers::DAV1D_CHR_VERTICAL;
use crate::include::dav1d::headers::DAV1D_CHR_UNKNOWN;
use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dITUTT35;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeader {
    pub profile: libc::c_int,
    pub max_width: libc::c_int,
    pub max_height: libc::c_int,
    pub layout: Dav1dPixelLayout,
    pub pri: Dav1dColorPrimaries,
    pub trc: Dav1dTransferCharacteristics,
    pub mtrx: Dav1dMatrixCoefficients,
    pub chr: Dav1dChromaSamplePosition,
    pub hbd: libc::c_int,
    pub color_range: libc::c_int,
    pub num_operating_points: libc::c_int,
    pub operating_points: [Dav1dSequenceHeaderOperatingPoint; 32],
    pub still_picture: libc::c_int,
    pub reduced_still_picture_header: libc::c_int,
    pub timing_info_present: libc::c_int,
    pub num_units_in_tick: libc::c_int,
    pub time_scale: libc::c_int,
    pub equal_picture_interval: libc::c_int,
    pub num_ticks_per_picture: libc::c_uint,
    pub decoder_model_info_present: libc::c_int,
    pub encoder_decoder_buffer_delay_length: libc::c_int,
    pub num_units_in_decoding_tick: libc::c_int,
    pub buffer_removal_delay_length: libc::c_int,
    pub frame_presentation_delay_length: libc::c_int,
    pub display_model_info_present: libc::c_int,
    pub width_n_bits: libc::c_int,
    pub height_n_bits: libc::c_int,
    pub frame_id_numbers_present: libc::c_int,
    pub delta_frame_id_n_bits: libc::c_int,
    pub frame_id_n_bits: libc::c_int,
    pub sb128: libc::c_int,
    pub filter_intra: libc::c_int,
    pub intra_edge_filter: libc::c_int,
    pub inter_intra: libc::c_int,
    pub masked_compound: libc::c_int,
    pub warped_motion: libc::c_int,
    pub dual_filter: libc::c_int,
    pub order_hint: libc::c_int,
    pub jnt_comp: libc::c_int,
    pub ref_frame_mvs: libc::c_int,
    pub screen_content_tools: Dav1dAdaptiveBoolean,
    pub force_integer_mv: Dav1dAdaptiveBoolean,
    pub order_hint_n_bits: libc::c_int,
    pub super_res: libc::c_int,
    pub cdef: libc::c_int,
    pub restoration: libc::c_int,
    pub ss_hor: libc::c_int,
    pub ss_ver: libc::c_int,
    pub monochrome: libc::c_int,
    pub color_description_present: libc::c_int,
    pub separate_uv_delta_q: libc::c_int,
    pub film_grain_present: libc::c_int,
    pub operating_parameter_info: [Dav1dSequenceHeaderOperatingParameterInfo; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: libc::c_int,
    pub encoder_buffer_delay: libc::c_int,
    pub low_delay_mode: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingPoint {
    pub major_level: libc::c_int,
    pub minor_level: libc::c_int,
    pub initial_display_delay: libc::c_int,
    pub idc: libc::c_int,
    pub tier: libc::c_int,
    pub decoder_model_param_present: libc::c_int,
    pub display_model_param_present: libc::c_int,
}
use crate::include::dav1d::headers::Dav1dSegmentationData;
use crate::include::dav1d::headers::Dav1dSegmentationDataSet;
use crate::include::dav1d::headers::Dav1dLoopfilterModeRefDeltas;
use crate::include::dav1d::headers::Dav1dFilmGrainData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader {
    pub film_grain: C2RustUnnamed_11,
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
    pub super_res: C2RustUnnamed_10,
    pub have_render_size: libc::c_int,
    pub allow_intrabc: libc::c_int,
    pub frame_ref_short_signaling: libc::c_int,
    pub refidx: [libc::c_int; 7],
    pub hp: libc::c_int,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: libc::c_int,
    pub use_ref_frame_mvs: libc::c_int,
    pub refresh_context: libc::c_int,
    pub tiling: C2RustUnnamed_9,
    pub quant: C2RustUnnamed_8,
    pub segmentation: C2RustUnnamed_7,
    pub delta: C2RustUnnamed_4,
    pub all_lossless: libc::c_int,
    pub loopfilter: C2RustUnnamed_3,
    pub cdef: C2RustUnnamed_2,
    pub restoration: C2RustUnnamed_1,
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
pub struct C2RustUnnamed_1 {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub damping: libc::c_int,
    pub n_bits: libc::c_int,
    pub y_strength: [libc::c_int; 8],
    pub uv_strength: [libc::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub level_y: [libc::c_int; 2],
    pub level_u: libc::c_int,
    pub level_v: libc::c_int,
    pub mode_ref_delta_enabled: libc::c_int,
    pub mode_ref_delta_update: libc::c_int,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub q: C2RustUnnamed_6,
    pub lf: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
    pub multi: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub enabled: libc::c_int,
    pub update_map: libc::c_int,
    pub temporal: libc::c_int,
    pub update_data: libc::c_int,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [libc::c_int; 8],
    pub qidx: [libc::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub yac: libc::c_int,
    pub ydc_delta: libc::c_int,
    pub udc_delta: libc::c_int,
    pub uac_delta: libc::c_int,
    pub vdc_delta: libc::c_int,
    pub vac_delta: libc::c_int,
    pub qm: libc::c_int,
    pub qm_y: libc::c_int,
    pub qm_u: libc::c_int,
    pub qm_v: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub uniform: libc::c_int,
    pub n_bytes: libc::c_uint,
    pub min_log2_cols: libc::c_int,
    pub max_log2_cols: libc::c_int,
    pub log2_cols: libc::c_int,
    pub cols: libc::c_int,
    pub min_log2_rows: libc::c_int,
    pub max_log2_rows: libc::c_int,
    pub log2_rows: libc::c_int,
    pub rows: libc::c_int,
    pub col_start_sb: [uint16_t; 65],
    pub row_start_sb: [uint16_t; 65],
    pub update: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub width_scale_denominator: libc::c_int,
    pub enabled: libc::c_int,
}
use crate::include::dav1d::headers::Dav1dFrameHeaderOperatingPoint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub data: Dav1dFilmGrainData,
    pub present: libc::c_int,
    pub update: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPictureParameters {
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MuxerContext {
    pub data: *mut MuxerPriv,
    pub impl_0: *const Muxer,
    pub one_file_per_frame: libc::c_int,
    pub fps: [libc::c_uint; 2],
    pub filename: *const libc::c_char,
    pub framenum: libc::c_int,
    pub priv_data: [uint64_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Muxer {
    pub priv_data_size: libc::c_int,
    pub name: *const libc::c_char,
    pub extension: *const libc::c_char,
    pub write_header: Option::<
        unsafe extern "C" fn(
            *mut MuxerPriv,
            *const libc::c_char,
            *const Dav1dPictureParameters,
            *const libc::c_uint,
        ) -> libc::c_int,
    >,
    pub write_picture: Option::<
        unsafe extern "C" fn(*mut MuxerPriv, *mut Dav1dPicture) -> libc::c_int,
    >,
    pub write_trailer: Option::<unsafe extern "C" fn(*mut MuxerPriv) -> ()>,
    pub verify: Option::<
        unsafe extern "C" fn(*mut MuxerPriv, *const libc::c_char) -> libc::c_int,
    >,
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
static mut muxers: [*const Muxer; 5] = unsafe {
    [
        &null_muxer as *const Muxer,
        &md5_muxer as *const Muxer,
        &yuv_muxer as *const Muxer,
        &y4m2_muxer as *const Muxer,
        0 as *const Muxer,
    ]
};
unsafe extern "C" fn find_extension(f: *const libc::c_char) -> *const libc::c_char {
    let l: size_t = strlen(f);
    if l == 0 as libc::c_int as libc::c_ulong {
        return 0 as *const libc::c_char;
    }
    let end: *const libc::c_char = &*f
        .offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as *const libc::c_char;
    let mut step: *const libc::c_char = end;
    while *step as libc::c_int >= 'a' as i32 && *step as libc::c_int <= 'z' as i32
        || *step as libc::c_int >= 'A' as i32 && *step as libc::c_int <= 'Z' as i32
        || *step as libc::c_int >= '0' as i32 && *step as libc::c_int <= '9' as i32
    {
        step = step.offset(-1);
    }
    return if step < end && step > f && *step as libc::c_int == '.' as i32
        && *step.offset(-(1 as libc::c_int) as isize) as libc::c_int != '/' as i32
    {
        &*step.offset(1 as libc::c_int as isize) as *const libc::c_char
    } else {
        0 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn output_open(
    c_out: *mut *mut MuxerContext,
    name: *const libc::c_char,
    filename: *const libc::c_char,
    p: *const Dav1dPictureParameters,
    mut fps: *const libc::c_uint,
) -> libc::c_int {
    let mut impl_0: *const Muxer = 0 as *const Muxer;
    let mut c: *mut MuxerContext = 0 as *mut MuxerContext;
    let mut i: libc::c_uint = 0;
    let mut res: libc::c_int = 0;
    let mut name_offset: libc::c_int = 0 as libc::c_int;
    if !name.is_null() {
        name_offset = 5 as libc::c_int
            * (strncmp(
                name,
                b"frame\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0) as libc::c_int;
        i = 0 as libc::c_int as libc::c_uint;
        while !(muxers[i as usize]).is_null() {
            if strcmp((*muxers[i as usize]).name, &*name.offset(name_offset as isize))
                == 0
            {
                impl_0 = muxers[i as usize];
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if (muxers[i as usize]).is_null() {
            fprintf(
                stderr,
                b"Failed to find muxer named \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                name,
            );
            return -(92 as libc::c_int);
        }
    } else if strcmp(filename, b"/dev/null\0" as *const u8 as *const libc::c_char) == 0 {
        impl_0 = muxers[0 as libc::c_int as usize];
    } else {
        let ext: *const libc::c_char = find_extension(filename);
        if ext.is_null() {
            fprintf(
                stderr,
                b"No extension found for file %s\n\0" as *const u8
                    as *const libc::c_char,
                filename,
            );
            return -(1 as libc::c_int);
        }
        i = 0 as libc::c_int as libc::c_uint;
        while !(muxers[i as usize]).is_null() {
            if strcmp((*muxers[i as usize]).extension, ext) == 0 {
                impl_0 = muxers[i as usize];
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if (muxers[i as usize]).is_null() {
            fprintf(
                stderr,
                b"Failed to find muxer for extension \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                ext,
            );
            return -(92 as libc::c_int);
        }
    }
    c = malloc(
        (48 as libc::c_ulong).wrapping_add((*impl_0).priv_data_size as libc::c_ulong),
    ) as *mut MuxerContext;
    if c.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(12 as libc::c_int);
    }
    (*c).impl_0 = impl_0;
    (*c).data = ((*c).priv_data).as_mut_ptr() as *mut MuxerPriv;
    let mut have_num_pattern: libc::c_int = 0 as libc::c_int;
    let mut ptr: *const libc::c_char = if !filename.is_null() {
        strchr(filename, '%' as i32)
    } else {
        0 as *mut libc::c_char
    };
    while have_num_pattern == 0 && !ptr.is_null() {
        ptr = ptr.offset(1);
        while *ptr as libc::c_int >= '0' as i32 && *ptr as libc::c_int <= '9' as i32 {
            ptr = ptr.offset(1);
        }
        have_num_pattern = (*ptr as libc::c_int == 'n' as i32) as libc::c_int;
        ptr = strchr(ptr, '%' as i32);
    }
    (*c)
        .one_file_per_frame = (name_offset != 0
        || name.is_null() && have_num_pattern != 0) as libc::c_int;
    if (*c).one_file_per_frame != 0 {
        (*c).fps[0 as libc::c_int as usize] = *fps.offset(0 as libc::c_int as isize);
        (*c).fps[1 as libc::c_int as usize] = *fps.offset(1 as libc::c_int as isize);
        (*c).filename = filename;
        (*c).framenum = 0 as libc::c_int;
    } else if ((*impl_0).write_header).is_some()
        && {
            res = ((*impl_0).write_header)
                .expect("non-null function pointer")((*c).data, filename, p, fps);
            res < 0 as libc::c_int
        }
    {
        free(c as *mut libc::c_void);
        return res;
    }
    *c_out = c;
    return 0 as libc::c_int;
}
unsafe extern "C" fn safe_strncat(
    dst: *mut libc::c_char,
    dst_len: libc::c_int,
    src: *const libc::c_char,
    src_len: libc::c_int,
) {
    if src_len == 0 {
        return;
    }
    let dst_fill: libc::c_int = strlen(dst) as libc::c_int;
    if !(dst_fill < dst_len) {
        unreachable!();
    }
    let to_copy: libc::c_int = imin(src_len, dst_len - dst_fill - 1 as libc::c_int);
    if to_copy == 0 {
        return;
    }
    memcpy(
        dst.offset(dst_fill as isize) as *mut libc::c_void,
        src as *const libc::c_void,
        to_copy as libc::c_ulong,
    );
    *dst.offset((dst_fill + to_copy) as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn assemble_field(
    dst: *mut libc::c_char,
    dst_len: libc::c_int,
    fmt: *const libc::c_char,
    fmt_len: libc::c_int,
    field: libc::c_int,
) {
    let mut fmt_copy: [libc::c_char; 32] = [0; 32];
    if !(*fmt.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32) {
        unreachable!();
    }
    fmt_copy[0 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    if *fmt.offset(1 as libc::c_int as isize) as libc::c_int >= '1' as i32
        && *fmt.offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32
    {
        fmt_copy[1 as libc::c_int as usize] = '0' as i32 as libc::c_char;
        fmt_copy[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        fmt_copy[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    safe_strncat(
        fmt_copy.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        &*fmt.offset(1 as libc::c_int as isize),
        fmt_len - 1 as libc::c_int,
    );
    safe_strncat(
        fmt_copy.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        b"d\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    let mut tmp: [libc::c_char; 32] = [0; 32];
    snprintf(
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        fmt_copy.as_mut_ptr(),
        field,
    );
    safe_strncat(
        dst,
        dst_len,
        tmp.as_mut_ptr(),
        strlen(tmp.as_mut_ptr()) as libc::c_int,
    );
}
unsafe extern "C" fn assemble_filename(
    ctx: *mut MuxerContext,
    filename: *mut libc::c_char,
    filename_size: libc::c_int,
    p: *const Dav1dPictureParameters,
) {
    *filename.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    let fresh0 = (*ctx).framenum;
    (*ctx).framenum = (*ctx).framenum + 1;
    let framenum: libc::c_int = fresh0;
    if ((*ctx).filename).is_null() {
        unreachable!();
    }
    let mut ptr: *const libc::c_char = (*ctx).filename;
    let mut iptr: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        iptr = strchr(ptr, '%' as i32);
        if iptr.is_null() {
            break;
        }
        safe_strncat(
            filename,
            filename_size,
            ptr,
            iptr.offset_from(ptr) as libc::c_long as libc::c_int,
        );
        ptr = iptr;
        let mut iiptr: *const libc::c_char = &*iptr.offset(1 as libc::c_int as isize)
            as *const libc::c_char;
        while *iiptr as libc::c_int >= '0' as i32 && *iiptr as libc::c_int <= '9' as i32
        {
            iiptr = iiptr.offset(1);
        }
        match *iiptr as libc::c_int {
            119 => {
                assemble_field(
                    filename,
                    filename_size,
                    ptr,
                    iiptr.offset_from(ptr) as libc::c_long as libc::c_int,
                    (*p).w,
                );
            }
            104 => {
                assemble_field(
                    filename,
                    filename_size,
                    ptr,
                    iiptr.offset_from(ptr) as libc::c_long as libc::c_int,
                    (*p).h,
                );
            }
            110 => {
                assemble_field(
                    filename,
                    filename_size,
                    ptr,
                    iiptr.offset_from(ptr) as libc::c_long as libc::c_int,
                    framenum,
                );
            }
            _ => {
                safe_strncat(
                    filename,
                    filename_size,
                    b"%\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
                ptr = &*iptr.offset(1 as libc::c_int as isize) as *const libc::c_char;
                continue;
            }
        }
        ptr = &*iiptr.offset(1 as libc::c_int as isize) as *const libc::c_char;
    }
    safe_strncat(filename, filename_size, ptr, strlen(ptr) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn output_write(
    ctx: *mut MuxerContext,
    p: *mut Dav1dPicture,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    if (*ctx).one_file_per_frame != 0 && ((*(*ctx).impl_0).write_header).is_some() {
        let mut filename: [libc::c_char; 1024] = [0; 1024];
        assemble_filename(
            ctx,
            filename.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            &mut (*p).p,
        );
        res = ((*(*ctx).impl_0).write_header)
            .expect(
                "non-null function pointer",
            )(
            (*ctx).data,
            filename.as_mut_ptr(),
            &mut (*p).p,
            ((*ctx).fps).as_mut_ptr() as *const libc::c_uint,
        );
        if res < 0 as libc::c_int {
            return res;
        }
    }
    res = ((*(*ctx).impl_0).write_picture)
        .expect("non-null function pointer")((*ctx).data, p);
    if res < 0 as libc::c_int {
        return res;
    }
    if (*ctx).one_file_per_frame != 0 && ((*(*ctx).impl_0).write_trailer).is_some() {
        ((*(*ctx).impl_0).write_trailer)
            .expect("non-null function pointer")((*ctx).data);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn output_close(ctx: *mut MuxerContext) {
    if (*ctx).one_file_per_frame == 0 && ((*(*ctx).impl_0).write_trailer).is_some() {
        ((*(*ctx).impl_0).write_trailer)
            .expect("non-null function pointer")((*ctx).data);
    }
    free(ctx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn output_verify(
    ctx: *mut MuxerContext,
    md5_str: *const libc::c_char,
) -> libc::c_int {
    let res: libc::c_int = if ((*(*ctx).impl_0).verify).is_some() {
        ((*(*ctx).impl_0).verify)
            .expect("non-null function pointer")((*ctx).data, md5_str)
    } else {
        0 as libc::c_int
    };
    free(ctx as *mut libc::c_void);
    return res;
}
