use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::src::cdf::CdfContext;
use crate::src::msac::MsacContext;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut libc::c_void;
}









pub type pixel = uint16_t;
pub type coef = int32_t;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;

use crate::src::r#ref::Dav1dRef;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext {
    pub seq_hdr_ref: *mut Dav1dRef,
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr_ref: *mut Dav1dRef,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub refp: [Dav1dThreadPicture; 7],
    pub cur: Dav1dPicture,
    pub sr_cur: Dav1dThreadPicture,
    pub mvs_ref: *mut Dav1dRef,
    pub mvs: *mut refmvs_temporal_block,
    pub ref_mvs: [*mut refmvs_temporal_block; 7],
    pub ref_mvs_ref: [*mut Dav1dRef; 7],
    pub cur_segmap_ref: *mut Dav1dRef,
    pub prev_segmap_ref: *mut Dav1dRef,
    pub cur_segmap: *mut uint8_t,
    pub prev_segmap: *const uint8_t,
    pub refpoc: [libc::c_uint; 7],
    pub refrefpoc: [[libc::c_uint; 7]; 7],
    pub gmv_warp_allowed: [uint8_t; 7],
    pub in_cdf: CdfThreadContext,
    pub out_cdf: CdfThreadContext,
    pub tile: *mut Dav1dTileGroup,
    pub n_tile_data_alloc: libc::c_int,
    pub n_tile_data: libc::c_int,
    pub svc: [[ScalableMotionParams; 2]; 7],
    pub resize_step: [libc::c_int; 2],
    pub resize_start: [libc::c_int; 2],
    pub c: *const Dav1dContext,
    pub ts: *mut Dav1dTileState,
    pub n_ts: libc::c_int,
    pub dsp: *const Dav1dDSPContext,
    pub bd_fn: Dav1dFrameContext_bd_fn,
    pub ipred_edge_sz: libc::c_int,
    pub ipred_edge: [*mut pixel; 3],
    pub b4_stride: ptrdiff_t,
    pub w4: libc::c_int,
    pub h4: libc::c_int,
    pub bw: libc::c_int,
    pub bh: libc::c_int,
    pub sb128w: libc::c_int,
    pub sb128h: libc::c_int,
    pub sbh: libc::c_int,
    pub sb_shift: libc::c_int,
    pub sb_step: libc::c_int,
    pub sr_sb128w: libc::c_int,
    pub dq: [[[uint16_t; 2]; 3]; 8],
    pub qm: [[*const uint8_t; 3]; 19],
    pub a: *mut BlockContext,
    pub a_sz: libc::c_int,
    pub rf: refmvs_frame,
    pub jnt_weights: [[uint8_t; 7]; 7],
    pub bitdepth_max: libc::c_int,
    pub frame_thread: Dav1dFrameContext_frame_thread,
    pub lf: Dav1dFrameContext_lf,
    pub task_thread: Dav1dFrameContext_task_thread,
    pub tile_thread: FrameTileThreadData,
}
use crate::src::internal::FrameTileThreadData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_task_thread {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub ttd: *mut TaskThreadData,
    pub tasks: *mut Dav1dTask,
    pub tile_tasks: [*mut Dav1dTask; 2],
    pub init_task: Dav1dTask,
    pub num_tasks: libc::c_int,
    pub num_tile_tasks: libc::c_int,
    pub init_done: atomic_int,
    pub done: [atomic_int; 2],
    pub retval: libc::c_int,
    pub update_set: libc::c_int,
    pub error: atomic_int,
    pub task_counter: atomic_int,
    pub task_head: *mut Dav1dTask,
    pub task_tail: *mut Dav1dTask,
    pub task_cur_prev: *mut Dav1dTask,
    pub pending_tasks: Dav1dFrameContext_task_thread_pending_tasks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_task_thread_pending_tasks {
    pub merge: atomic_int,
    pub lock: pthread_mutex_t,
    pub head: *mut Dav1dTask,
    pub tail: *mut Dav1dTask,
}
use crate::src::internal::Dav1dTask;
use crate::src::internal::TaskType;













use crate::include::pthread::pthread_mutex_t;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskThreadData {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub first: atomic_uint,
    pub cur: libc::c_uint,
    pub reset_task_cur: atomic_uint,
    pub cond_signaled: atomic_int,
    pub delayed_fg: Dav1dContext_task_thread_delayed_fg,
    pub inited: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext_task_thread_delayed_fg {
    pub exec: libc::c_int,
    pub cond: pthread_cond_t,
    pub in_0: *const Dav1dPicture,
    pub out: *mut Dav1dPicture,
    pub type_0: TaskType,
    pub progress: [atomic_int; 2],
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub grain_lut_16bpc: [[[int16_t; 82]; 74]; 3],
    pub scaling_16bpc: [[uint8_t; 4096]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub grain_lut_8bpc: [[[int8_t; 82]; 74]; 3],
    pub scaling_8bpc: [[uint8_t; 256]; 3],
}
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dContentLightLevel;

use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dWarpedMotionParams;













































use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dSequenceHeader;



























































use crate::include::pthread::pthread_cond_t;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_lf {
    pub level: *mut [uint8_t; 4],
    pub mask: *mut Av1Filter,
    pub lr_mask: *mut Av1Restoration,
    pub mask_sz: libc::c_int,
    pub lr_mask_sz: libc::c_int,
    pub cdef_buf_plane_sz: [libc::c_int; 2],
    pub cdef_buf_sbh: libc::c_int,
    pub lr_buf_plane_sz: [libc::c_int; 2],
    pub re_sz: libc::c_int,
    pub lim_lut: Av1FilterLUT,
    pub last_sharpness: libc::c_int,
    pub lvl: [[[[uint8_t; 2]; 8]; 4]; 8],
    pub tx_lpf_right_edge: [*mut uint8_t; 2],
    pub cdef_line_buf: *mut uint8_t,
    pub lr_line_buf: *mut uint8_t,
    pub cdef_line: [[*mut pixel; 3]; 2],
    pub cdef_lpf_line: [*mut pixel; 3],
    pub lr_lpf_line: [*mut pixel; 3],
    pub start_of_tile_row: *mut uint8_t,
    pub start_of_tile_row_sz: libc::c_int,
    pub need_cdef_lpf_copy: libc::c_int,
    pub p: [*mut pixel; 3],
    pub sr_p: [*mut pixel; 3],
    pub mask_ptr: *mut Av1Filter,
    pub prev_mask_ptr: *mut Av1Filter,
    pub restore_planes: libc::c_int,
}
use crate::src::lf_mask::Av1Filter;
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::lf_mask::Av1Restoration;
use crate::src::lf_mask::Av1RestorationUnit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_frame_thread {
    pub next_tile_row: [libc::c_int; 2],
    pub entropy_progress: atomic_int,
    pub deblock_progress: atomic_int,
    pub frame_progress: *mut atomic_uint,
    pub copy_lpf_progress: *mut atomic_uint,
    pub b: *mut Av1Block,
    pub cbi: *mut CodedBlockInfo,
    pub pal: *mut [[uint16_t; 8]; 3],
    pub pal_idx: *mut uint8_t,
    pub cf: *mut coef,
    pub prog_sz: libc::c_int,
    pub pal_sz: libc::c_int,
    pub pal_idx_sz: libc::c_int,
    pub cf_sz: libc::c_int,
    pub tile_start_off: *mut libc::c_int,
}
use crate::src::internal::CodedBlockInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Block {
    pub bl: uint8_t,
    pub bs: uint8_t,
    pub bp: uint8_t,
    pub intra: uint8_t,
    pub seg_id: uint8_t,
    pub skip_mode: uint8_t,
    pub skip: uint8_t,
    pub uvtx: uint8_t,
    pub c2rust_unnamed: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub c2rust_unnamed_0: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub comp_type: uint8_t,
    pub inter_mode: uint8_t,
    pub motion_mode: uint8_t,
    pub drl_idx: uint8_t,
    pub ref_0: [int8_t; 2],
    pub max_ytx: uint8_t,
    pub filter2d: uint8_t,
    pub interintra_type: uint8_t,
    pub tx_split0: uint8_t,
    pub tx_split1: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub c2rust_unnamed_0: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub mv2d: mv,
    pub matrix: [int16_t; 4],
}
use crate::src::levels::mv;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub mv: [mv; 2],
    pub wedge_idx: uint8_t,
    pub mask_sign: uint8_t,
    pub interintra_mode: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub y_mode: uint8_t,
    pub uv_mode: uint8_t,
    pub tx: uint8_t,
    pub pal_sz: [uint8_t; 2],
    pub y_angle: int8_t,
    pub uv_angle: int8_t,
    pub cfl_alpha: [int8_t; 2],
}
use crate::src::refmvs::refmvs_frame;



use crate::src::refmvs::refmvs_temporal_block;
use crate::src::env::BlockContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_bd_fn {
    pub recon_b_intra: recon_b_intra_fn,
    pub recon_b_inter: recon_b_inter_fn,
    pub filter_sbrow: filter_sbrow_fn,
    pub filter_sbrow_deblock_cols: filter_sbrow_fn,
    pub filter_sbrow_deblock_rows: filter_sbrow_fn,
    pub filter_sbrow_cdef: Option::<
        unsafe extern "C" fn(*mut Dav1dTaskContext, libc::c_int) -> (),
    >,
    pub filter_sbrow_resize: filter_sbrow_fn,
    pub filter_sbrow_lr: filter_sbrow_fn,
    pub backup_ipred_edge: backup_ipred_edge_fn,
    pub read_coef_blocks: read_coef_blocks_fn,
}
pub type read_coef_blocks_fn = Option::<
    unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> (),
>;
use crate::src::levels::BlockSize;























#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext {
    pub c: *const Dav1dContext,
    pub f: *const Dav1dFrameContext,
    pub ts: *mut Dav1dTileState,
    pub bx: libc::c_int,
    pub by: libc::c_int,
    pub l: BlockContext,
    pub a: *mut BlockContext,
    pub rt: refmvs_tile,
    pub c2rust_unnamed: Dav1dTaskContext_cf,
    pub al_pal: [[[[uint16_t; 8]; 3]; 32]; 2],
    pub pal_sz_uv: [[uint8_t; 32]; 2],
    pub txtp_map: [uint8_t; 1024],
    pub scratch: Dav1dTaskContext_scratch,
    pub warpmv: Dav1dWarpedMotionParams,
    pub lf_mask: *mut Av1Filter,
    pub top_pre_cdef_toggle: libc::c_int,
    pub cur_sb_cdef_idx_ptr: *mut int8_t,
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: Dav1dTaskContext_frame_thread,
    pub task_thread: Dav1dTaskContext_task_thread,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_task_thread {
    pub td: thread_data,
    pub ttd: *mut TaskThreadData,
    pub fttd: *mut FrameTileThreadData,
    pub flushed: libc::c_int,
    pub die: libc::c_int,
}
use crate::src::thread_data::thread_data;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_frame_thread {
    pub pass: libc::c_int,
}
use crate::src::levels::Filter2d;











#[derive(Copy, Clone)]
#[repr(C)]
pub union Dav1dTaskContext_scratch {
    pub c2rust_unnamed: C2RustUnnamed_16,
    pub c2rust_unnamed_0: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub c2rust_unnamed: C2RustUnnamed_14,
    pub ac: [int16_t; 1024],
    pub pal_idx: [uint8_t; 8192],
    pub pal: [[uint16_t; 8]; 3],
    pub c2rust_unnamed_0: Dav1dTaskContext_scratch_interintra_edge,
}
use crate::src::internal::Dav1dTaskContext_scratch_interintra_edge;


#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub levels: [uint8_t; 1088],
    pub c2rust_unnamed: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub pal_order: [[uint8_t; 8]; 64],
    pub pal_ctx: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub c2rust_unnamed: C2RustUnnamed_18,
    pub c2rust_unnamed_0: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub emu_edge_8bpc: [uint8_t; 84160],
    pub emu_edge_16bpc: [uint16_t; 84160],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub lap_8bpc: [uint8_t; 4096],
    pub lap_16bpc: [uint16_t; 4096],
    pub c2rust_unnamed: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub compinter: [[int16_t; 16384]; 2],
    pub seg_mask: [uint8_t; 16384],
}
use crate::src::internal::Dav1dTaskContext_cf;
use crate::src::refmvs::refmvs_tile;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileState {
    pub cdf: CdfContext,
    pub msac: MsacContext,
    pub tiling: Dav1dTileState_tiling,
    pub progress: [atomic_int; 2],
    pub frame_thread: [Dav1dTileState_frame_thread; 2],
    pub lowest_pixel: *mut [[libc::c_int; 2]; 7],
    pub dqmem: [[[uint16_t; 2]; 3]; 8],
    pub dq: *const [[uint16_t; 2]; 3],
    pub last_qidx: libc::c_int,
    pub last_delta_lf: [int8_t; 4],
    pub lflvlmem: [[[[uint8_t; 2]; 8]; 4]; 8],
    pub lflvl: *const [[[uint8_t; 2]; 8]; 4],
    pub lr_ref: [*mut Av1RestorationUnit; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileState_frame_thread {
    pub pal_idx: *mut uint8_t,
    pub cf: *mut coef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileState_tiling {
    pub col_start: libc::c_int,
    pub col_end: libc::c_int,
    pub row_start: libc::c_int,
    pub row_end: libc::c_int,
    pub col: libc::c_int,
    pub row: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext {
    pub fc: *mut Dav1dFrameContext,
    pub n_fc: libc::c_uint,
    pub tc: *mut Dav1dTaskContext,
    pub n_tc: libc::c_uint,
    pub tile: *mut Dav1dTileGroup,
    pub n_tile_data_alloc: libc::c_int,
    pub n_tile_data: libc::c_int,
    pub n_tiles: libc::c_int,
    pub seq_hdr_pool: *mut Dav1dMemPool,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr_pool: *mut Dav1dMemPool,
    pub frame_hdr_ref: *mut Dav1dRef,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub content_light_ref: *mut Dav1dRef,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display_ref: *mut Dav1dRef,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35_ref: *mut Dav1dRef,
    pub itut_t35: *mut Dav1dITUTT35,
    pub in_0: Dav1dData,
    pub out: Dav1dThreadPicture,
    pub cache: Dav1dThreadPicture,
    pub flush_mem: atomic_int,
    pub flush: *mut atomic_int,
    pub frame_thread: Dav1dContext_frame_thread,
    pub task_thread: TaskThreadData,
    pub segmap_pool: *mut Dav1dMemPool,
    pub refmvs_pool: *mut Dav1dMemPool,
    pub refs: [Dav1dContext_refs; 8],
    pub cdf_pool: *mut Dav1dMemPool,
    pub cdf: [CdfThreadContext; 8],
    pub dsp: [Dav1dDSPContext; 3],
    pub refmvs_dsp: Dav1dRefmvsDSPContext,
    pub intra_edge: Dav1dContext_intra_edge,
    pub allocator: Dav1dPicAllocator,
    pub apply_grain: libc::c_int,
    pub operating_point: libc::c_int,
    pub operating_point_idc: libc::c_uint,
    pub all_layers: libc::c_int,
    pub max_spatial_id: libc::c_int,
    pub frame_size_limit: libc::c_uint,
    pub strict_std_compliance: libc::c_int,
    pub output_invisible_frames: libc::c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub decode_frame_type: Dav1dDecodeFrameType,
    pub drain: libc::c_int,
    pub frame_flags: PictureFlags,
    pub event_flags: Dav1dEventFlags,
    pub cached_error_props: Dav1dDataProps,
    pub cached_error: libc::c_int,
    pub logger: Dav1dLogger,
    pub picture_pool: *mut Dav1dMemPool,
}
use crate::src::mem::Dav1dMemPool;

use crate::include::dav1d::dav1d::Dav1dLogger;
use crate::include::dav1d::dav1d::Dav1dEventFlags;


use crate::src::picture::PictureFlags;



use crate::include::dav1d::dav1d::Dav1dDecodeFrameType;




use crate::include::dav1d::dav1d::Dav1dInloopFilterType;





use crate::include::dav1d::picture::Dav1dPicAllocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext_intra_edge {
    pub root: [*mut EdgeNode; 2],
    pub branch_sb128: [EdgeBranch; 85],
    pub branch_sb64: [EdgeBranch; 21],
    pub tip_sb128: [EdgeTip; 256],
    pub tip_sb64: [EdgeTip; 64],
}
use crate::src::intra_edge::EdgeTip;
use crate::src::intra_edge::EdgeFlags;






use crate::src::intra_edge::EdgeNode;
use crate::src::intra_edge::EdgeBranch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dRefmvsDSPContext {
    pub splat_mv: splat_mv_fn,
}
use crate::src::refmvs::splat_mv_fn;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dDSPContext {
    pub fg: Dav1dFilmGrainDSPContext,
    pub ipred: Dav1dIntraPredDSPContext,
    pub mc: Dav1dMCDSPContext,
    pub itx: Dav1dInvTxfmDSPContext,
    pub lf: Dav1dLoopFilterDSPContext,
    pub cdef: Dav1dCdefDSPContext,
    pub lr: Dav1dLoopRestorationDSPContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: [looprestorationfilter_fn; 2],
    pub sgr: [looprestorationfilter_fn; 3],
}
pub type looprestorationfilter_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const LooprestorationParams,
        LrEdgeFlags,
        libc::c_int,
    ) -> (),
>;
use crate::src::looprestoration::LrEdgeFlags;




#[derive(Copy, Clone)]
#[repr(C)]
pub union LooprestorationParams {
    pub filter: [[int16_t; 8]; 2],
    pub sgr: LooprestorationParams_sgr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LooprestorationParams_sgr {
    pub s0: uint32_t,
    pub s1: uint32_t,
    pub w0: int16_t,
    pub w1: int16_t,
}
pub type const_left_pixel_row = *const [pixel; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}
pub type cdef_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row_2px,
        *const pixel,
        *const pixel,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        CdefEdgeFlags,
        libc::c_int,
    ) -> (),
>;
use crate::src::cdef::CdefEdgeFlags;
use crate::src::cdef::CDEF_HAVE_BOTTOM;
use crate::src::cdef::CDEF_HAVE_TOP;
use crate::src::cdef::CDEF_HAVE_RIGHT;
use crate::src::cdef::CDEF_HAVE_LEFT;
pub type const_left_pixel_row_2px = *const [pixel; 2];
pub type cdef_dir_fn = Option::<
    unsafe extern "C" fn(
        *const pixel,
        ptrdiff_t,
        *mut libc::c_uint,
        libc::c_int,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
}
pub type loopfilter_sb_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint32_t,
        *const [uint8_t; 4],
        ptrdiff_t,
        *const Av1FilterLUT,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}
pub type itxfm_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *mut coef,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMCDSPContext {
    pub mc: [mc_fn; 10],
    pub mc_scaled: [mc_scaled_fn; 10],
    pub mct: [mct_fn; 10],
    pub mct_scaled: [mct_scaled_fn; 10],
    pub avg: avg_fn,
    pub w_avg: w_avg_fn,
    pub mask: mask_fn,
    pub w_mask: [w_mask_fn; 3],
    pub blend: blend_fn,
    pub blend_v: blend_dir_fn,
    pub blend_h: blend_dir_fn,
    pub warp8x8: warp8x8_fn,
    pub warp8x8t: warp8x8t_fn,
    pub emu_edge: emu_edge_fn,
    pub resize: resize_fn,
}
pub type resize_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type emu_edge_fn = Option::<
    unsafe extern "C" fn(
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
    ) -> (),
>;
pub type warp8x8t_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type warp8x8_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_dir_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_mask_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *mut uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mask_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
        libc::c_int,
    ) -> (),
>;
pub type w_avg_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type avg_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_scaled_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_scaled_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dIntraPredDSPContext {
    pub intra_pred: [angular_ipred_fn; 14],
    pub cfl_ac: [cfl_ac_fn; 3],
    pub cfl_pred: [cfl_pred_fn; 6],
    pub pal_pred: pal_pred_fn,
}
pub type pal_pred_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint16_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_ac_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type angular_ipred_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        libc::c_int,
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
pub type entry = int16_t;
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
pub type generate_grain_uv_fn = Option::<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
        libc::c_int,
    ) -> (),
>;
pub type generate_grain_y_fn = Option::<
    unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData, libc::c_int) -> (),
>;
use crate::src::cdf::CdfThreadContext;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext_refs {
    pub p: Dav1dThreadPicture,
    pub segmap: *mut Dav1dRef,
    pub refmvs: *mut Dav1dRef,
    pub refpoc: [libc::c_uint; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dThreadPicture {
    pub p: Dav1dPicture,
    pub visible: libc::c_int,
    pub showable: libc::c_int,
    pub flags: PictureFlags,
    pub progress: *mut atomic_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext_frame_thread {
    pub out_delayed: *mut Dav1dThreadPicture,
    pub next: libc::c_uint,
}
use crate::src::internal::Dav1dTileGroup;
pub type backup_ipred_edge_fn = Option::<
    unsafe extern "C" fn(*mut Dav1dTaskContext) -> (),
>;
pub type filter_sbrow_fn = Option::<
    unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
>;
pub type recon_b_inter_fn = Option::<
    unsafe extern "C" fn(
        *mut Dav1dTaskContext,
        BlockSize,
        *const Av1Block,
    ) -> libc::c_int,
>;
pub type recon_b_intra_fn = Option::<
    unsafe extern "C" fn(
        *mut Dav1dTaskContext,
        BlockSize,
        EdgeFlags,
        *const Av1Block,
    ) -> (),
>;
use crate::src::internal::ScalableMotionParams;
pub type Backup2x8Flags = libc::c_uint;
pub const BACKUP_2X8_UV: Backup2x8Flags = 2;
pub const BACKUP_2X8_Y: Backup2x8Flags = 1;
#[inline]
unsafe extern "C" fn clz(mask: libc::c_uint) -> libc::c_int {
    return mask.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn ulog2(v: libc::c_uint) -> libc::c_int {
    return 31 as libc::c_int - clz(v);
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1 as libc::c_int;
}
unsafe extern "C" fn backup2lines(
    mut dst: *const *mut pixel,
    mut src: *const *mut pixel,
    mut stride: *const ptrdiff_t,
    layout: Dav1dPixelLayout,
) {
    let y_stride: ptrdiff_t = PXSTRIDE(*stride.offset(0 as libc::c_int as isize));
    if y_stride < 0 {
        memcpy(
            (*dst.offset(0 as libc::c_int as isize)).offset(y_stride as isize)
                as *mut libc::c_void,
            (*src.offset(0 as libc::c_int as isize))
                .offset((7 as libc::c_int as isize * y_stride) as isize)
                as *const libc::c_void,
            (-(2 as libc::c_int) as isize * y_stride << 1 as libc::c_int)
                as size_t,
        );
    } else {
        memcpy(
            *dst.offset(0 as libc::c_int as isize) as *mut libc::c_void,
            (*src.offset(0 as libc::c_int as isize))
                .offset(6  * y_stride as isize)
                as *const libc::c_void,
            (2 * y_stride << 1 as libc::c_int)
                as size_t,
        );
    }
    if layout as libc::c_uint != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint {
        let uv_stride: ptrdiff_t = PXSTRIDE(*stride.offset(1 as libc::c_int as isize));
        if uv_stride < 0 {
            let uv_off: libc::c_int = if layout as libc::c_uint
                == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint
            {
                3 as libc::c_int
            } else {
                7 as libc::c_int
            };
            memcpy(
                (*dst.offset(1 as libc::c_int as isize)).offset(uv_stride as isize)
                    as *mut libc::c_void,
                (*src.offset(1 as libc::c_int as isize))
                    .offset((uv_off as isize * uv_stride) as isize)
                    as *const libc::c_void,
                (-(2 as libc::c_int) as isize * uv_stride << 1 as libc::c_int)
                    as size_t,
            );
            memcpy(
                (*dst.offset(2 as libc::c_int as isize)).offset(uv_stride as isize)
                    as *mut libc::c_void,
                (*src.offset(2 as libc::c_int as isize))
                    .offset((uv_off as isize * uv_stride) as isize)
                    as *const libc::c_void,
                (-(2 as libc::c_int) as isize * uv_stride << 1 as libc::c_int)
                    as size_t,
            );
        } else {
            let uv_off_0: libc::c_int = if layout as libc::c_uint
                == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint
            {
                2 as libc::c_int
            } else {
                6 as libc::c_int
            };
            memcpy(
                *dst.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                (*src.offset(1 as libc::c_int as isize))
                    .offset((uv_off_0 as isize * uv_stride) as isize)
                    as *const libc::c_void,
                (2 * uv_stride << 1 as libc::c_int)
                    as size_t,
            );
            memcpy(
                *dst.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                (*src.offset(2 as libc::c_int as isize))
                    .offset((uv_off_0 as isize * uv_stride) as isize)
                    as *const libc::c_void,
                (2 * uv_stride << 1 as libc::c_int) as size_t,
            );
        }
    }
}
unsafe extern "C" fn backup2x8(
    mut dst: *mut [[pixel; 2]; 8],
    mut src: *const *mut pixel,
    mut src_stride: *const ptrdiff_t,
    mut x_off: libc::c_int,
    layout: Dav1dPixelLayout,
    flag: Backup2x8Flags,
) {
    let mut y_off: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
    if flag as libc::c_uint & BACKUP_2X8_Y as libc::c_int as libc::c_uint != 0 {
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < 8 as libc::c_int {
            memcpy(
                ((*dst.offset(0 as libc::c_int as isize))[y as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                &mut *(*src.offset(0 as libc::c_int as isize))
                    .offset(
                        (y_off + x_off as isize
                            - 2 as libc::c_int as isize) as isize,
                    ) as *mut pixel as *const libc::c_void,
                ((2 as libc::c_int) << 1 as libc::c_int) as size_t,
            );
            y += 1;
            y_off += PXSTRIDE(*src_stride.offset(0 as libc::c_int as isize));
        }
    }
    if layout as libc::c_uint == DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        || flag as libc::c_uint & BACKUP_2X8_UV as libc::c_int as libc::c_uint == 0
    {
        return;
    }
    let ss_ver: libc::c_int = (layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = (layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    x_off >>= ss_hor;
    y_off = 0 as libc::c_int as ptrdiff_t;
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < 8 as libc::c_int >> ss_ver {
        memcpy(
            ((*dst.offset(1 as libc::c_int as isize))[y_0 as usize]).as_mut_ptr()
                as *mut libc::c_void,
            &mut *(*src.offset(1))
                .offset(
                    (y_off + x_off as isize - 2)
                        as isize,
                ) as *mut pixel as *const libc::c_void,
            ((2 as libc::c_int) << 1 as libc::c_int) as size_t,
        );
        memcpy(
            ((*dst.offset(2 as libc::c_int as isize))[y_0 as usize]).as_mut_ptr()
                as *mut libc::c_void,
            &mut *(*src.offset(2 as libc::c_int as isize))
                .offset(
                    (y_off + x_off as isize - 2)
                        as isize,
                ) as *mut pixel as *const libc::c_void,
            ((2 as libc::c_int) << 1 as libc::c_int) as size_t,
        );
        y_0 += 1;
        y_off += PXSTRIDE(*src_stride.offset(1 as libc::c_int as isize));
    }
}
unsafe extern "C" fn adjust_strength(
    strength: libc::c_int,
    var: libc::c_uint,
) -> libc::c_int {
    if var == 0 {
        return 0 as libc::c_int;
    }
    let i: libc::c_int = if var >> 6 as libc::c_int != 0 {
        imin(ulog2(var >> 6 as libc::c_int), 12 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    return strength * (4 as libc::c_int + i) + 8 as libc::c_int >> 4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdef_brow_16bpc(
    tc: *mut Dav1dTaskContext,
    mut p: *const *mut pixel,
    lflvl: *const Av1Filter,
    by_start: libc::c_int,
    by_end: libc::c_int,
    sbrow_start: libc::c_int,
    sby: libc::c_int,
) {
    let f: *mut Dav1dFrameContext = (*tc).f as *mut Dav1dFrameContext;
    let bitdepth_min_8: libc::c_int = if 16 as libc::c_int == 8 as libc::c_int {
        0 as libc::c_int
    } else {
        (*f).cur.p.bpc - 8 as libc::c_int
    };
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut edges: CdefEdgeFlags = (CDEF_HAVE_BOTTOM as libc::c_int
        | (if by_start > 0 as libc::c_int {
            CDEF_HAVE_TOP as libc::c_int
        } else {
            0 as libc::c_int
        })) as CdefEdgeFlags;
    let mut ptrs: [*mut pixel; 3] = [
        *p.offset(0 as libc::c_int as isize),
        *p.offset(1 as libc::c_int as isize),
        *p.offset(2 as libc::c_int as isize),
    ];
    let sbsz: libc::c_int = 16 as libc::c_int;
    let sb64w: libc::c_int = (*f).sb128w << 1 as libc::c_int;
    let damping: libc::c_int = (*(*f).frame_hdr).cdef.damping + bitdepth_min_8;
    let layout: Dav1dPixelLayout = (*f).cur.p.layout;
    let uv_idx: libc::c_int = (DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
        .wrapping_sub(layout as libc::c_uint) as libc::c_int;
    let ss_ver: libc::c_int = (layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = (layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    static mut uv_dirs: [[uint8_t; 8]; 2] = [
        [
            0 as libc::c_int as uint8_t,
            1 as libc::c_int as uint8_t,
            2 as libc::c_int as uint8_t,
            3 as libc::c_int as uint8_t,
            4 as libc::c_int as uint8_t,
            5 as libc::c_int as uint8_t,
            6 as libc::c_int as uint8_t,
            7 as libc::c_int as uint8_t,
        ],
        [
            7 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            2 as libc::c_int as uint8_t,
            4 as libc::c_int as uint8_t,
            5 as libc::c_int as uint8_t,
            6 as libc::c_int as uint8_t,
            6 as libc::c_int as uint8_t,
            6 as libc::c_int as uint8_t,
        ],
    ];
    let mut uv_dir: *const uint8_t = (uv_dirs[(layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I422 as libc::c_int as libc::c_uint) as libc::c_int
        as usize])
        .as_ptr();
    let have_tt: libc::c_int = ((*(*f).c).n_tc > 1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let sb128: libc::c_int = (*(*f).seq_hdr).sb128;
    let resize: libc::c_int = ((*(*f).frame_hdr).width[0 as libc::c_int as usize]
        != (*(*f).frame_hdr).width[1 as libc::c_int as usize]) as libc::c_int;
    let y_stride: ptrdiff_t = PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize]);
    let uv_stride: ptrdiff_t = PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize]);
    let mut bit: libc::c_int = 0 as libc::c_int;
    let mut by: libc::c_int = by_start;
    while by < by_end {
        let tf: libc::c_int = (*tc).top_pre_cdef_toggle;
        let by_idx: libc::c_int = (by & 30 as libc::c_int) >> 1 as libc::c_int;
        if by + 2 as libc::c_int >= (*f).bh {
            edges = ::core::mem::transmute::<
                libc::c_uint,
                CdefEdgeFlags,
            >(
                edges as libc::c_uint
                    & !(CDEF_HAVE_BOTTOM as libc::c_int) as libc::c_uint,
            );
        }
        if (have_tt == 0 || sbrow_start != 0 || (by + 2 as libc::c_int) < by_end)
            && edges as libc::c_uint & CDEF_HAVE_BOTTOM as libc::c_int as libc::c_uint
                != 0
        {
            let cdef_top_bak: [*mut pixel; 3] = [
                ((*f)
                    .lf
                    .cdef_line[(tf == 0) as libc::c_int
                    as usize][0 as libc::c_int as usize])
                    .offset(
                        ((have_tt * sby * 4 as libc::c_int) as isize * y_stride)
                            as isize,
                    ),
                ((*f)
                    .lf
                    .cdef_line[(tf == 0) as libc::c_int
                    as usize][1 as libc::c_int as usize])
                    .offset(
                        ((have_tt * sby * 8 as libc::c_int) as isize * uv_stride)
                            as isize,
                    ),
                ((*f)
                    .lf
                    .cdef_line[(tf == 0) as libc::c_int
                    as usize][2 as libc::c_int as usize])
                    .offset(
                        ((have_tt * sby * 8 as libc::c_int) as isize * uv_stride)
                            as isize,
                    ),
            ];
            backup2lines(
                cdef_top_bak.as_ptr(),
                ptrs.as_mut_ptr() as *const *mut pixel,
                ((*f).cur.stride).as_mut_ptr() as *const ptrdiff_t,
                layout,
            );
        }
        let mut lr_bak: [[[[pixel; 2]; 8]; 3]; 2] = [[[[0; 2]; 8]; 3]; 2];
        let mut iptrs: [*mut pixel; 3] = [
            ptrs[0 as libc::c_int as usize],
            ptrs[1 as libc::c_int as usize],
            ptrs[2 as libc::c_int as usize],
        ];
        edges = ::core::mem::transmute::<
            libc::c_uint,
            CdefEdgeFlags,
        >(edges as libc::c_uint & !(CDEF_HAVE_LEFT as libc::c_int) as libc::c_uint);
        edges = ::core::mem::transmute::<
            libc::c_uint,
            CdefEdgeFlags,
        >(edges as libc::c_uint | CDEF_HAVE_RIGHT as libc::c_int as libc::c_uint);
        let mut prev_flag: Backup2x8Flags = 0 as Backup2x8Flags;
        let mut sbx: libc::c_int = 0 as libc::c_int;
        let mut last_skip: libc::c_int = 1 as libc::c_int;
        while sbx < sb64w {
            let mut noskip_row: *const [uint16_t; 2] = 0 as *const [uint16_t; 2];
            let mut noskip_mask: libc::c_uint = 0;
            let mut y_lvl: libc::c_int = 0;
            let mut uv_lvl: libc::c_int = 0;
            let mut flag: Backup2x8Flags = 0 as Backup2x8Flags;
            let mut y_pri_lvl: libc::c_int = 0;
            let mut y_sec_lvl: libc::c_int = 0;
            let mut uv_pri_lvl: libc::c_int = 0;
            let mut uv_sec_lvl: libc::c_int = 0;
            let mut bptrs: [*mut pixel; 3] = [0 as *mut pixel; 3];
            let sb128x: libc::c_int = sbx >> 1 as libc::c_int;
            let sb64_idx: libc::c_int = ((by & sbsz) >> 3 as libc::c_int)
                + (sbx & 1 as libc::c_int);
            let cdef_idx: libc::c_int = (*lflvl.offset(sb128x as isize))
                .cdef_idx[sb64_idx as usize] as libc::c_int;
            if cdef_idx == -(1 as libc::c_int)
                || (*(*f).frame_hdr).cdef.y_strength[cdef_idx as usize] == 0
                    && (*(*f).frame_hdr).cdef.uv_strength[cdef_idx as usize] == 0
            {
                last_skip = 1 as libc::c_int;
            } else {
                noskip_row = &*((*lflvl.offset(sb128x as isize)).noskip_mask)
                    .as_ptr()
                    .offset(by_idx as isize) as *const [uint16_t; 2];
                noskip_mask = ((*noskip_row
                    .offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
                    as libc::c_uint) << 16 as libc::c_int
                    | (*noskip_row
                        .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                        as libc::c_uint;
                y_lvl = (*(*f).frame_hdr).cdef.y_strength[cdef_idx as usize];
                uv_lvl = (*(*f).frame_hdr).cdef.uv_strength[cdef_idx as usize];
                flag = ((y_lvl != 0) as libc::c_int
                    + (((uv_lvl != 0) as libc::c_int) << 1 as libc::c_int))
                    as Backup2x8Flags;
                y_pri_lvl = (y_lvl >> 2 as libc::c_int) << bitdepth_min_8;
                y_sec_lvl = y_lvl & 3 as libc::c_int;
                y_sec_lvl += (y_sec_lvl == 3 as libc::c_int) as libc::c_int;
                y_sec_lvl <<= bitdepth_min_8;
                uv_pri_lvl = (uv_lvl >> 2 as libc::c_int) << bitdepth_min_8;
                uv_sec_lvl = uv_lvl & 3 as libc::c_int;
                uv_sec_lvl += (uv_sec_lvl == 3 as libc::c_int) as libc::c_int;
                uv_sec_lvl <<= bitdepth_min_8;
                bptrs = [
                    iptrs[0 as libc::c_int as usize],
                    iptrs[1 as libc::c_int as usize],
                    iptrs[2 as libc::c_int as usize],
                ];
                let mut bx: libc::c_int = sbx * sbsz;
                while bx < imin((sbx + 1 as libc::c_int) * sbsz, (*f).bw) {
                    let mut uvdir: libc::c_int = 0;
                    let mut do_left: libc::c_int = 0;
                    let mut dir: libc::c_int = 0;
                    let mut variance: libc::c_uint = 0;
                    let mut top: *const pixel = 0 as *const pixel;
                    let mut bot: *const pixel = 0 as *const pixel;
                    let mut offset: ptrdiff_t = 0;
                    let mut current_block_84: u64;
                    if bx + 2 as libc::c_int >= (*f).bw {
                        edges = ::core::mem::transmute::<
                            libc::c_uint,
                            CdefEdgeFlags,
                        >(
                            edges as libc::c_uint
                                & !(CDEF_HAVE_RIGHT as libc::c_int) as libc::c_uint,
                        );
                    }
                    let bx_mask: uint32_t = (3 as libc::c_uint)
                        << (bx & 30 as libc::c_int);
                    if noskip_mask & bx_mask == 0 {
                        last_skip = 1 as libc::c_int;
                    } else {
                        do_left = (if last_skip != 0 {
                            flag as libc::c_uint
                        } else {
                            (prev_flag as libc::c_uint ^ flag as libc::c_uint)
                                & flag as libc::c_uint
                        }) as libc::c_int;
                        prev_flag = flag;
                        if do_left != 0
                            && edges as libc::c_uint
                                & CDEF_HAVE_LEFT as libc::c_int as libc::c_uint != 0
                        {
                            backup2x8(
                                (lr_bak[bit as usize]).as_mut_ptr(),
                                bptrs.as_mut_ptr() as *const *mut pixel,
                                ((*f).cur.stride).as_mut_ptr() as *const ptrdiff_t,
                                0 as libc::c_int,
                                layout,
                                do_left as Backup2x8Flags,
                            );
                        }
                        if edges as libc::c_uint
                            & CDEF_HAVE_RIGHT as libc::c_int as libc::c_uint != 0
                        {
                            backup2x8(
                                (lr_bak[(bit == 0) as libc::c_int as usize]).as_mut_ptr(),
                                bptrs.as_mut_ptr() as *const *mut pixel,
                                ((*f).cur.stride).as_mut_ptr() as *const ptrdiff_t,
                                8 as libc::c_int,
                                layout,
                                flag,
                            );
                        }
                        dir = 0;
                        variance = 0;
                        if y_pri_lvl != 0 || uv_pri_lvl != 0 {
                            dir = ((*dsp).cdef.dir)
                                .expect(
                                    "non-null function pointer",
                                )(
                                bptrs[0 as libc::c_int as usize],
                                (*f).cur.stride[0 as libc::c_int as usize],
                                &mut variance,
                                (*f).bitdepth_max,
                            );
                        }
                        top = 0 as *const pixel;
                        bot = 0 as *const pixel;
                        offset = 0;
                        if have_tt == 0 {
                            current_block_84 = 17728966195399430138;
                        } else if sbrow_start != 0 && by == by_start {
                            if resize != 0 {
                                offset = ((sby - 1 as libc::c_int) * 4 as libc::c_int)
                                    as isize * y_stride
                                    + (bx * 4 as libc::c_int) as isize;
                                top = &mut *(*((*f).lf.cdef_lpf_line)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize))
                                    .offset(offset as isize) as *mut pixel;
                            } else {
                                offset = (sby * ((4 as libc::c_int) << sb128)
                                    - 4 as libc::c_int) as isize * y_stride
                                    + (bx * 4 as libc::c_int) as isize;
                                top = &mut *(*((*f).lf.lr_lpf_line)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize))
                                    .offset(offset as isize) as *mut pixel;
                            }
                            bot = (bptrs[0 as libc::c_int as usize])
                                .offset(
                                    (8 * y_stride) as isize,
                                );
                            current_block_84 = 17075014677070940716;
                        } else if sbrow_start == 0 && by + 2 as libc::c_int >= by_end {
                            top = &mut *(*(*((*f).lf.cdef_line)
                                .as_mut_ptr()
                                .offset(tf as isize))
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize))
                                .offset(
                                    ((sby * 4 as libc::c_int) as isize * y_stride
                                        + (bx * 4 as libc::c_int) as isize) as isize,
                                ) as *mut pixel;
                            if resize != 0 {
                                offset = (sby * 4 as libc::c_int + 2 as libc::c_int)
                                    as isize * y_stride
                                    + (bx * 4 as libc::c_int) as isize;
                                bot = &mut *(*((*f).lf.cdef_lpf_line)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize))
                                    .offset(offset as isize) as *mut pixel;
                            } else {
                                let line: libc::c_int = sby * ((4 as libc::c_int) << sb128)
                                    + 4 as libc::c_int * sb128 + 2 as libc::c_int;
                                offset = line as isize * y_stride
                                    + (bx * 4 as libc::c_int) as isize;
                                bot = &mut *(*((*f).lf.lr_lpf_line)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize))
                                    .offset(offset as isize) as *mut pixel;
                            }
                            current_block_84 = 17075014677070940716;
                        } else {
                            current_block_84 = 17728966195399430138;
                        }
                        match current_block_84 {
                            17728966195399430138 => {
                                offset = (sby * 4 as libc::c_int) as isize
                                    * y_stride;
                                top = &mut *(*(*((*f).lf.cdef_line)
                                    .as_mut_ptr()
                                    .offset(tf as isize))
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize))
                                    .offset(
                                        (have_tt as isize * offset
                                            + (bx * 4 as libc::c_int) as isize) as isize,
                                    ) as *mut pixel;
                                bot = (bptrs[0 as libc::c_int as usize])
                                    .offset(
                                        (8 * y_stride) as isize,
                                    );
                            }
                            _ => {}
                        }
                        if y_pri_lvl != 0 {
                            let adj_y_pri_lvl: libc::c_int = adjust_strength(
                                y_pri_lvl,
                                variance,
                            );
                            if adj_y_pri_lvl != 0 || y_sec_lvl != 0 {
                                ((*dsp).cdef.fb[0 as libc::c_int as usize])
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    bptrs[0 as libc::c_int as usize],
                                    (*f).cur.stride[0 as libc::c_int as usize],
                                    (lr_bak[bit as usize][0 as libc::c_int as usize])
                                        .as_mut_ptr() as const_left_pixel_row_2px,
                                    top,
                                    bot,
                                    adj_y_pri_lvl,
                                    y_sec_lvl,
                                    dir,
                                    damping,
                                    edges,
                                    (*f).bitdepth_max,
                                );
                            }
                        } else if y_sec_lvl != 0 {
                            ((*dsp).cdef.fb[0 as libc::c_int as usize])
                                .expect(
                                    "non-null function pointer",
                                )(
                                bptrs[0 as libc::c_int as usize],
                                (*f).cur.stride[0 as libc::c_int as usize],
                                (lr_bak[bit as usize][0 as libc::c_int as usize])
                                    .as_mut_ptr() as const_left_pixel_row_2px,
                                top,
                                bot,
                                0 as libc::c_int,
                                y_sec_lvl,
                                0 as libc::c_int,
                                damping,
                                edges,
                                (*f).bitdepth_max,
                            );
                        }
                        if !(uv_lvl == 0) {
                            if !(layout as libc::c_uint
                                != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint)
                            {
                                unreachable!();
                            }
                            uvdir = if uv_pri_lvl != 0 {
                                *uv_dir.offset(dir as isize) as libc::c_int
                            } else {
                                0 as libc::c_int
                            };
                            let mut pl: libc::c_int = 1 as libc::c_int;
                            while pl <= 2 as libc::c_int {
                                let mut current_block_77: u64;
                                if have_tt == 0 {
                                    current_block_77 = 5687667889785024198;
                                } else if sbrow_start != 0 && by == by_start {
                                    if resize != 0 {
                                        offset = ((sby - 1 as libc::c_int) * 4 as libc::c_int)
                                            as isize * uv_stride
                                            + (bx * 4 as libc::c_int >> ss_hor) as isize;
                                        top = &mut *(*((*f).lf.cdef_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            .offset(offset as isize) as *mut pixel;
                                    } else {
                                        let line_0: libc::c_int = sby
                                            * ((4 as libc::c_int) << sb128) - 4 as libc::c_int;
                                        offset = line_0 as isize * uv_stride + (bx * 4 as libc::c_int >> ss_hor) as isize;
                                        top = &mut *(*((*f).lf.lr_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            .offset(offset as isize) as *mut pixel;
                                    }
                                    bot = (bptrs[pl as usize])
                                        .offset(
                                            ((8 as libc::c_int >> ss_ver) as isize * uv_stride)
                                                as isize,
                                        );
                                    current_block_77 = 6540614962658479183;
                                } else if sbrow_start == 0
                                    && by + 2 as libc::c_int >= by_end
                                {
                                    let top_offset: ptrdiff_t = (sby * 8 as libc::c_int)
                                        as isize * uv_stride
                                        + (bx * 4 as libc::c_int >> ss_hor) as isize;
                                    top = &mut *(*(*((*f).lf.cdef_line)
                                        .as_mut_ptr()
                                        .offset(tf as isize))
                                        .as_mut_ptr()
                                        .offset(pl as isize))
                                        .offset(top_offset as isize) as *mut pixel;
                                    if resize != 0 {
                                        offset = (sby * 4 as libc::c_int + 2 as libc::c_int)
                                            as isize * uv_stride
                                            + (bx * 4 as libc::c_int >> ss_hor) as isize;
                                        bot = &mut *(*((*f).lf.cdef_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            .offset(offset as isize) as *mut pixel;
                                    } else {
                                        let line_1: libc::c_int = sby
                                            * ((4 as libc::c_int) << sb128) + 4 as libc::c_int * sb128
                                            + 2 as libc::c_int;
                                        offset = line_1 as isize * uv_stride
                                            + (bx * 4 as libc::c_int >> ss_hor) as isize;
                                        bot = &mut *(*((*f).lf.lr_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            .offset(offset as isize) as *mut pixel;
                                    }
                                    current_block_77 = 6540614962658479183;
                                } else {
                                    current_block_77 = 5687667889785024198;
                                }
                                match current_block_77 {
                                    5687667889785024198 => {
                                        let offset_0: ptrdiff_t = (sby * 8 as libc::c_int)
                                            as isize * uv_stride;
                                        top = &mut *(*(*((*f).lf.cdef_line)
                                            .as_mut_ptr()
                                            .offset(tf as isize))
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            .offset(
                                                (have_tt as isize * offset_0
                                                    + (bx * 4 as libc::c_int >> ss_hor) as isize)
                                                    as isize,
                                            ) as *mut pixel;
                                        bot = (bptrs[pl as usize])
                                            .offset(
                                                ((8 as libc::c_int >> ss_ver) as isize * uv_stride)
                                                    as isize,
                                            );
                                    }
                                    _ => {}
                                }
                                ((*dsp).cdef.fb[uv_idx as usize])
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    bptrs[pl as usize],
                                    (*f).cur.stride[1 as libc::c_int as usize],
                                    (lr_bak[bit as usize][pl as usize]).as_mut_ptr()
                                        as const_left_pixel_row_2px,
                                    top,
                                    bot,
                                    uv_pri_lvl,
                                    uv_sec_lvl,
                                    uvdir,
                                    damping - 1 as libc::c_int,
                                    edges,
                                    (*f).bitdepth_max,
                                );
                                pl += 1;
                            }
                        }
                        bit ^= 1 as libc::c_int;
                        last_skip = 0 as libc::c_int;
                    }
                    bptrs[0 as libc::c_int
                        as usize] = (bptrs[0 as libc::c_int as usize])
                        .offset(8 as libc::c_int as isize);
                    bptrs[1 as libc::c_int
                        as usize] = (bptrs[1 as libc::c_int as usize])
                        .offset((8 as libc::c_int >> ss_hor) as isize);
                    bptrs[2 as libc::c_int
                        as usize] = (bptrs[2 as libc::c_int as usize])
                        .offset((8 as libc::c_int >> ss_hor) as isize);
                    bx += 2 as libc::c_int;
                    edges = ::core::mem::transmute::<
                        libc::c_uint,
                        CdefEdgeFlags,
                    >(
                        edges as libc::c_uint
                            | CDEF_HAVE_LEFT as libc::c_int as libc::c_uint,
                    );
                }
            }
            iptrs[0 as libc::c_int
                as usize] = (iptrs[0 as libc::c_int as usize])
                .offset((sbsz * 4 as libc::c_int) as isize);
            iptrs[1 as libc::c_int
                as usize] = (iptrs[1 as libc::c_int as usize])
                .offset((sbsz * 4 as libc::c_int >> ss_hor) as isize);
            iptrs[2 as libc::c_int
                as usize] = (iptrs[2 as libc::c_int as usize])
                .offset((sbsz * 4 as libc::c_int >> ss_hor) as isize);
            sbx += 1;
            edges = ::core::mem::transmute::<
                libc::c_uint,
                CdefEdgeFlags,
            >(edges as libc::c_uint | CDEF_HAVE_LEFT as libc::c_int as libc::c_uint);
        }
        ptrs[0 as libc::c_int
            as usize] = (ptrs[0 as libc::c_int as usize])
            .offset(
                (8
                    * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])) as isize,
            );
        ptrs[1 as libc::c_int
            as usize] = (ptrs[1 as libc::c_int as usize])
            .offset(
                (8
                    * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize]) >> ss_ver)
                    as isize,
            );
        ptrs[2 as libc::c_int
            as usize] = (ptrs[2 as libc::c_int as usize])
            .offset(
                (8
                    * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize]) >> ss_ver)
                    as isize,
            );
        (*tc).top_pre_cdef_toggle ^= 1 as libc::c_int;
        by += 2 as libc::c_int;
        edges = ::core::mem::transmute::<
            libc::c_uint,
            CdefEdgeFlags,
        >(edges as libc::c_uint | CDEF_HAVE_TOP as libc::c_int as libc::c_uint);
    }
}
