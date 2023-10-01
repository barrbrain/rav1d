use std::ffi::c_int;
use std::ffi::c_uint;

/// This is so we can store both `*mut D` and `*mut R`
/// for maintaining `dav1d` ABI compatibility,
/// where `D` is the `Dav1d*` type and `R` is the `Rav1d` type.
pub(crate) struct DRav1d<R, D> {
    pub rav1d: R,
    pub dav1d: D,
}

impl<R, D> DRav1d<R, D>
where
    R: Clone + Into<D>,
    D: Clone + Into<R>,
{
    pub fn update_rav1d(&mut self) {
        self.rav1d = self.dav1d.clone().into();
    }

    pub fn update_dav1d(&mut self) {
        self.dav1d = self.rav1d.clone().into();
    }
}

pub type Dav1dObuType = c_uint;
pub const DAV1D_OBU_PADDING: Dav1dObuType = 15;
pub const DAV1D_OBU_REDUNDANT_FRAME_HDR: Dav1dObuType = 7;
pub const DAV1D_OBU_FRAME: Dav1dObuType = 6;
pub const DAV1D_OBU_METADATA: Dav1dObuType = 5;
pub const DAV1D_OBU_TILE_GRP: Dav1dObuType = 4;
pub const DAV1D_OBU_FRAME_HDR: Dav1dObuType = 3;
pub const DAV1D_OBU_TD: Dav1dObuType = 2;
pub const DAV1D_OBU_SEQ_HDR: Dav1dObuType = 1;

pub(crate) type Rav1dObuType = c_uint;
pub(crate) const RAV1D_OBU_PADDING: Rav1dObuType = DAV1D_OBU_PADDING;
pub(crate) const RAV1D_OBU_REDUNDANT_FRAME_HDR: Rav1dObuType = DAV1D_OBU_REDUNDANT_FRAME_HDR;
pub(crate) const RAV1D_OBU_FRAME: Rav1dObuType = DAV1D_OBU_FRAME;
pub(crate) const RAV1D_OBU_METADATA: Rav1dObuType = DAV1D_OBU_METADATA;
pub(crate) const RAV1D_OBU_TILE_GRP: Rav1dObuType = DAV1D_OBU_TILE_GRP;
pub(crate) const RAV1D_OBU_FRAME_HDR: Rav1dObuType = DAV1D_OBU_FRAME_HDR;
pub(crate) const RAV1D_OBU_TD: Rav1dObuType = DAV1D_OBU_TD;
pub(crate) const RAV1D_OBU_SEQ_HDR: Rav1dObuType = DAV1D_OBU_SEQ_HDR;

pub type Dav1dTxfmMode = c_uint;
pub const DAV1D_N_TX_MODES: usize = 3;
pub const DAV1D_TX_SWITCHABLE: Dav1dTxfmMode = 2;
pub const DAV1D_TX_LARGEST: Dav1dTxfmMode = 1;
pub const DAV1D_TX_4X4_ONLY: Dav1dTxfmMode = 0;

pub(crate) type Rav1dTxfmMode = c_uint;
pub(crate) const _RAV1D_N_TX_MODES: usize = DAV1D_N_TX_MODES;
pub(crate) const RAV1D_TX_SWITCHABLE: Rav1dTxfmMode = DAV1D_TX_SWITCHABLE;
pub(crate) const RAV1D_TX_LARGEST: Rav1dTxfmMode = DAV1D_TX_LARGEST;
pub(crate) const RAV1D_TX_4X4_ONLY: Rav1dTxfmMode = DAV1D_TX_4X4_ONLY;

pub type Dav1dFilterMode = u8;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;
pub const DAV1D_N_FILTERS: usize = 4;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const DAV1D_N_SWITCHABLE_FILTERS: usize = 3;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;

pub type Rav1dFilterMode = u8;
pub const RAV1D_FILTER_SWITCHABLE: Rav1dFilterMode = DAV1D_FILTER_SWITCHABLE;
pub const RAV1D_N_FILTERS: usize = DAV1D_N_FILTERS;
pub const RAV1D_FILTER_BILINEAR: Rav1dFilterMode = DAV1D_FILTER_BILINEAR;
pub const RAV1D_N_SWITCHABLE_FILTERS: usize = DAV1D_N_SWITCHABLE_FILTERS;
pub const RAV1D_FILTER_8TAP_SHARP: Rav1dFilterMode = DAV1D_FILTER_8TAP_SHARP;
pub const RAV1D_FILTER_8TAP_SMOOTH: Rav1dFilterMode = DAV1D_FILTER_8TAP_SMOOTH;
pub const RAV1D_FILTER_8TAP_REGULAR: Rav1dFilterMode = DAV1D_FILTER_8TAP_REGULAR;

pub type Dav1dAdaptiveBoolean = c_uint;
pub const DAV1D_ADAPTIVE: Dav1dAdaptiveBoolean = 2;
pub const DAV1D_ON: Dav1dAdaptiveBoolean = 1;
pub const DAV1D_OFF: Dav1dAdaptiveBoolean = 0;

pub(crate) type Rav1dAdaptiveBoolean = c_uint;
pub(crate) const RAV1D_ADAPTIVE: Rav1dAdaptiveBoolean = DAV1D_ADAPTIVE;
pub(crate) const _RAV1D_ON: Rav1dAdaptiveBoolean = DAV1D_ON;
pub(crate) const _RAV1D_OFF: Rav1dAdaptiveBoolean = DAV1D_OFF;

pub type Dav1dRestorationType = u8;
pub const DAV1D_RESTORATION_SGRPROJ: Dav1dRestorationType = 3;
pub const DAV1D_RESTORATION_WIENER: Dav1dRestorationType = 2;
pub const DAV1D_RESTORATION_SWITCHABLE: Dav1dRestorationType = 1;
pub const DAV1D_RESTORATION_NONE: Dav1dRestorationType = 0;

pub type Rav1dRestorationType = u8;
pub const RAV1D_RESTORATION_SGRPROJ: Rav1dRestorationType = DAV1D_RESTORATION_SGRPROJ;
pub const RAV1D_RESTORATION_WIENER: Rav1dRestorationType = DAV1D_RESTORATION_WIENER;
pub const RAV1D_RESTORATION_SWITCHABLE: Rav1dRestorationType = DAV1D_RESTORATION_SWITCHABLE;
pub const RAV1D_RESTORATION_NONE: Rav1dRestorationType = DAV1D_RESTORATION_NONE;

pub type Dav1dWarpedMotionType = c_uint;
pub const DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType = 3;
pub const DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType = 2;
pub const DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType = 1;
pub const DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType = 0;

pub type Rav1dWarpedMotionType = c_uint;
pub const RAV1D_WM_TYPE_AFFINE: Rav1dWarpedMotionType = DAV1D_WM_TYPE_AFFINE;
pub const RAV1D_WM_TYPE_ROT_ZOOM: Rav1dWarpedMotionType = DAV1D_WM_TYPE_ROT_ZOOM;
pub const RAV1D_WM_TYPE_TRANSLATION: Rav1dWarpedMotionType = DAV1D_WM_TYPE_TRANSLATION;
pub const RAV1D_WM_TYPE_IDENTITY: Rav1dWarpedMotionType = DAV1D_WM_TYPE_IDENTITY;

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [i32; 6],
    pub abcd: [i16; 4],
}

impl Dav1dWarpedMotionParams {
    pub const fn alpha(&self) -> i16 {
        self.abcd[0]
    }

    pub const fn beta(&self) -> i16 {
        self.abcd[1]
    }

    pub const fn gamma(&self) -> i16 {
        self.abcd[2]
    }

    pub const fn delta(&self) -> i16 {
        self.abcd[3]
    }
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dWarpedMotionParams {
    pub type_0: Rav1dWarpedMotionType,
    pub matrix: [i32; 6],
    pub abcd: [i16; 4],
}

impl Rav1dWarpedMotionParams {
    #[allow(dead_code)] // TODO(kkysen) remove when we use this
    pub const fn alpha(&self) -> i16 {
        self.abcd[0]
    }

    #[allow(dead_code)] // TODO(kkysen) remove when we use this
    pub const fn beta(&self) -> i16 {
        self.abcd[1]
    }

    #[allow(dead_code)] // TODO(kkysen) remove when we use this
    pub const fn gamma(&self) -> i16 {
        self.abcd[2]
    }

    #[allow(dead_code)] // TODO(kkysen) remove when we use this
    pub const fn delta(&self) -> i16 {
        self.abcd[3]
    }
}

impl From<Dav1dWarpedMotionParams> for Rav1dWarpedMotionParams {
    fn from(value: Dav1dWarpedMotionParams) -> Self {
        let Dav1dWarpedMotionParams {
            type_0,
            matrix,
            abcd,
        } = value;
        Self {
            type_0,
            matrix,
            abcd,
        }
    }
}

impl From<Rav1dWarpedMotionParams> for Dav1dWarpedMotionParams {
    fn from(value: Rav1dWarpedMotionParams) -> Self {
        let Rav1dWarpedMotionParams {
            type_0,
            matrix,
            abcd,
        } = value;
        Self {
            type_0,
            matrix,
            abcd,
        }
    }
}

pub type Dav1dPixelLayout = c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;

pub(crate) type Rav1dPixelLayout = c_uint;
pub(crate) const RAV1D_PIXEL_LAYOUT_I444: Rav1dPixelLayout = DAV1D_PIXEL_LAYOUT_I444;
pub(crate) const RAV1D_PIXEL_LAYOUT_I422: Rav1dPixelLayout = DAV1D_PIXEL_LAYOUT_I422;
pub(crate) const RAV1D_PIXEL_LAYOUT_I420: Rav1dPixelLayout = DAV1D_PIXEL_LAYOUT_I420;
pub(crate) const RAV1D_PIXEL_LAYOUT_I400: Rav1dPixelLayout = DAV1D_PIXEL_LAYOUT_I400;

pub type Dav1dFrameType = c_uint;
pub const DAV1D_FRAME_TYPE_SWITCH: Dav1dFrameType = 3;
pub const DAV1D_FRAME_TYPE_INTRA: Dav1dFrameType = 2;
pub const DAV1D_FRAME_TYPE_INTER: Dav1dFrameType = 1;
pub const DAV1D_FRAME_TYPE_KEY: Dav1dFrameType = 0;

pub(crate) type Rav1dFrameType = c_uint;
pub(crate) const RAV1D_FRAME_TYPE_SWITCH: Rav1dFrameType = DAV1D_FRAME_TYPE_SWITCH;
pub(crate) const RAV1D_FRAME_TYPE_INTRA: Rav1dFrameType = DAV1D_FRAME_TYPE_INTRA;
pub(crate) const RAV1D_FRAME_TYPE_INTER: Rav1dFrameType = DAV1D_FRAME_TYPE_INTER;
pub(crate) const RAV1D_FRAME_TYPE_KEY: Rav1dFrameType = DAV1D_FRAME_TYPE_KEY;

pub type Dav1dColorPrimaries = c_uint;
pub const DAV1D_COLOR_PRI_RESERVED: Dav1dColorPrimaries = 255;
pub const DAV1D_COLOR_PRI_EBU3213: Dav1dColorPrimaries = 22;
pub const DAV1D_COLOR_PRI_SMPTE432: Dav1dColorPrimaries = 12;
pub const DAV1D_COLOR_PRI_SMPTE431: Dav1dColorPrimaries = 11;
pub const DAV1D_COLOR_PRI_XYZ: Dav1dColorPrimaries = 10;
pub const DAV1D_COLOR_PRI_BT2020: Dav1dColorPrimaries = 9;
pub const DAV1D_COLOR_PRI_FILM: Dav1dColorPrimaries = 8;
pub const DAV1D_COLOR_PRI_SMPTE240: Dav1dColorPrimaries = 7;
pub const DAV1D_COLOR_PRI_BT601: Dav1dColorPrimaries = 6;
pub const DAV1D_COLOR_PRI_BT470BG: Dav1dColorPrimaries = 5;
pub const DAV1D_COLOR_PRI_BT470M: Dav1dColorPrimaries = 4;
pub const DAV1D_COLOR_PRI_UNKNOWN: Dav1dColorPrimaries = 2;
pub const DAV1D_COLOR_PRI_BT709: Dav1dColorPrimaries = 1;

pub(crate) type Rav1dColorPrimaries = c_uint;
pub(crate) const _RAV1D_COLOR_PRI_RESERVED: Rav1dColorPrimaries = DAV1D_COLOR_PRI_RESERVED;
pub(crate) const _RAV1D_COLOR_PRI_EBU3213: Rav1dColorPrimaries = DAV1D_COLOR_PRI_EBU3213;
pub(crate) const _RAV1D_COLOR_PRI_SMPTE432: Rav1dColorPrimaries = DAV1D_COLOR_PRI_SMPTE432;
pub(crate) const _RAV1D_COLOR_PRI_SMPTE431: Rav1dColorPrimaries = DAV1D_COLOR_PRI_SMPTE431;
pub(crate) const _RAV1D_COLOR_PRI_XYZ: Rav1dColorPrimaries = DAV1D_COLOR_PRI_XYZ;
pub(crate) const _RAV1D_COLOR_PRI_BT2020: Rav1dColorPrimaries = DAV1D_COLOR_PRI_BT2020;
pub(crate) const _RAV1D_COLOR_PRI_FILM: Rav1dColorPrimaries = DAV1D_COLOR_PRI_FILM;
pub(crate) const _RAV1D_COLOR_PRI_SMPTE240: Rav1dColorPrimaries = DAV1D_COLOR_PRI_SMPTE240;
pub(crate) const _RAV1D_COLOR_PRI_BT601: Rav1dColorPrimaries = DAV1D_COLOR_PRI_BT601;
pub(crate) const _RAV1D_COLOR_PRI_BT470BG: Rav1dColorPrimaries = DAV1D_COLOR_PRI_BT470BG;
pub(crate) const _RAV1D_COLOR_PRI_BT470M: Rav1dColorPrimaries = DAV1D_COLOR_PRI_BT470M;
pub(crate) const RAV1D_COLOR_PRI_UNKNOWN: Rav1dColorPrimaries = DAV1D_COLOR_PRI_UNKNOWN;
pub(crate) const RAV1D_COLOR_PRI_BT709: Rav1dColorPrimaries = DAV1D_COLOR_PRI_BT709;

pub type Dav1dTransferCharacteristics = c_uint;
pub const DAV1D_TRC_RESERVED: Dav1dTransferCharacteristics = 255;
pub const DAV1D_TRC_HLG: Dav1dTransferCharacteristics = 18;
pub const DAV1D_TRC_SMPTE428: Dav1dTransferCharacteristics = 17;
pub const DAV1D_TRC_SMPTE2084: Dav1dTransferCharacteristics = 16;
pub const DAV1D_TRC_BT2020_12BIT: Dav1dTransferCharacteristics = 15;
pub const DAV1D_TRC_BT2020_10BIT: Dav1dTransferCharacteristics = 14;
pub const DAV1D_TRC_SRGB: Dav1dTransferCharacteristics = 13;
pub const DAV1D_TRC_BT1361: Dav1dTransferCharacteristics = 12;
pub const DAV1D_TRC_IEC61966: Dav1dTransferCharacteristics = 11;
pub const DAV1D_TRC_LOG100_SQRT10: Dav1dTransferCharacteristics = 10;
pub const DAV1D_TRC_LOG100: Dav1dTransferCharacteristics = 9;
pub const DAV1D_TRC_LINEAR: Dav1dTransferCharacteristics = 8;
pub const DAV1D_TRC_SMPTE240: Dav1dTransferCharacteristics = 7;
pub const DAV1D_TRC_BT601: Dav1dTransferCharacteristics = 6;
pub const DAV1D_TRC_BT470BG: Dav1dTransferCharacteristics = 5;
pub const DAV1D_TRC_BT470M: Dav1dTransferCharacteristics = 4;
pub const DAV1D_TRC_UNKNOWN: Dav1dTransferCharacteristics = 2;
pub const DAV1D_TRC_BT709: Dav1dTransferCharacteristics = 1;

pub(crate) type Rav1dTransferCharacteristics = c_uint;
pub(crate) const _RAV1D_TRC_RESERVED: Rav1dTransferCharacteristics = DAV1D_TRC_RESERVED;
pub(crate) const _RAV1D_TRC_HLG: Rav1dTransferCharacteristics = DAV1D_TRC_HLG;
pub(crate) const _RAV1D_TRC_SMPTE428: Rav1dTransferCharacteristics = DAV1D_TRC_SMPTE428;
pub(crate) const _RAV1D_TRC_SMPTE2084: Rav1dTransferCharacteristics = DAV1D_TRC_SMPTE2084;
pub(crate) const _RAV1D_TRC_BT2020_12BIT: Rav1dTransferCharacteristics = DAV1D_TRC_BT2020_12BIT;
pub(crate) const _RAV1D_TRC_BT2020_10BIT: Rav1dTransferCharacteristics = DAV1D_TRC_BT2020_10BIT;
pub(crate) const RAV1D_TRC_SRGB: Rav1dTransferCharacteristics = DAV1D_TRC_SRGB;
pub(crate) const _RAV1D_TRC_BT1361: Rav1dTransferCharacteristics = DAV1D_TRC_BT1361;
pub(crate) const _RAV1D_TRC_IEC61966: Rav1dTransferCharacteristics = DAV1D_TRC_IEC61966;
pub(crate) const _RAV1D_TRC_LOG100_SQRT10: Rav1dTransferCharacteristics = DAV1D_TRC_LOG100_SQRT10;
pub(crate) const _RAV1D_TRC_LOG100: Rav1dTransferCharacteristics = DAV1D_TRC_LOG100;
pub(crate) const _RAV1D_TRC_LINEAR: Rav1dTransferCharacteristics = DAV1D_TRC_LINEAR;
pub(crate) const _RAV1D_TRC_SMPTE240: Rav1dTransferCharacteristics = DAV1D_TRC_SMPTE240;
pub(crate) const _RAV1D_TRC_BT601: Rav1dTransferCharacteristics = DAV1D_TRC_BT601;
pub(crate) const _RAV1D_TRC_BT470BG: Rav1dTransferCharacteristics = DAV1D_TRC_BT470BG;
pub(crate) const _RAV1D_TRC_BT470M: Rav1dTransferCharacteristics = DAV1D_TRC_BT470M;
pub(crate) const RAV1D_TRC_UNKNOWN: Rav1dTransferCharacteristics = DAV1D_TRC_UNKNOWN;
pub(crate) const _RAV1D_TRC_BT709: Rav1dTransferCharacteristics = DAV1D_TRC_BT709;

pub type Dav1dMatrixCoefficients = c_uint;
pub const DAV1D_MC_RESERVED: Dav1dMatrixCoefficients = 255;
pub const DAV1D_MC_ICTCP: Dav1dMatrixCoefficients = 14;
pub const DAV1D_MC_CHROMAT_CL: Dav1dMatrixCoefficients = 13;
pub const DAV1D_MC_CHROMAT_NCL: Dav1dMatrixCoefficients = 12;
pub const DAV1D_MC_SMPTE2085: Dav1dMatrixCoefficients = 11;
pub const DAV1D_MC_BT2020_CL: Dav1dMatrixCoefficients = 10;
pub const DAV1D_MC_BT2020_NCL: Dav1dMatrixCoefficients = 9;
pub const DAV1D_MC_SMPTE_YCGCO: Dav1dMatrixCoefficients = 8;
pub const DAV1D_MC_SMPTE240: Dav1dMatrixCoefficients = 7;
pub const DAV1D_MC_BT601: Dav1dMatrixCoefficients = 6;
pub const DAV1D_MC_BT470BG: Dav1dMatrixCoefficients = 5;
pub const DAV1D_MC_FCC: Dav1dMatrixCoefficients = 4;
pub const DAV1D_MC_UNKNOWN: Dav1dMatrixCoefficients = 2;
pub const DAV1D_MC_BT709: Dav1dMatrixCoefficients = 1;
pub const DAV1D_MC_IDENTITY: Dav1dMatrixCoefficients = 0;

pub(crate) type Rav1dMatrixCoefficients = c_uint;
pub(crate) const _RAV1D_MC_RESERVED: Rav1dMatrixCoefficients = DAV1D_MC_RESERVED;
pub(crate) const _RAV1D_MC_ICTCP: Rav1dMatrixCoefficients = DAV1D_MC_ICTCP;
pub(crate) const _RAV1D_MC_CHROMAT_CL: Rav1dMatrixCoefficients = DAV1D_MC_CHROMAT_CL;
pub(crate) const _RAV1D_MC_CHROMAT_NCL: Rav1dMatrixCoefficients = DAV1D_MC_CHROMAT_NCL;
pub(crate) const _RAV1D_MC_SMPTE2085: Rav1dMatrixCoefficients = DAV1D_MC_SMPTE2085;
pub(crate) const _RAV1D_MC_BT2020_CL: Rav1dMatrixCoefficients = DAV1D_MC_BT2020_CL;
pub(crate) const _RAV1D_MC_BT2020_NCL: Rav1dMatrixCoefficients = DAV1D_MC_BT2020_NCL;
pub(crate) const _RAV1D_MC_SMPTE_YCGCO: Rav1dMatrixCoefficients = DAV1D_MC_SMPTE_YCGCO;
pub(crate) const _RAV1D_MC_SMPTE240: Rav1dMatrixCoefficients = DAV1D_MC_SMPTE240;
pub(crate) const _RAV1D_MC_BT601: Rav1dMatrixCoefficients = DAV1D_MC_BT601;
pub(crate) const _RAV1D_MC_BT470BG: Rav1dMatrixCoefficients = DAV1D_MC_BT470BG;
pub(crate) const _RAV1D_MC_FCC: Rav1dMatrixCoefficients = DAV1D_MC_FCC;
pub(crate) const RAV1D_MC_UNKNOWN: Rav1dMatrixCoefficients = DAV1D_MC_UNKNOWN;
pub(crate) const _RAV1D_MC_BT709: Rav1dMatrixCoefficients = DAV1D_MC_BT709;
pub(crate) const RAV1D_MC_IDENTITY: Rav1dMatrixCoefficients = DAV1D_MC_IDENTITY;

pub type Dav1dChromaSamplePosition = c_uint;
pub const DAV1D_CHR_COLOCATED: Dav1dChromaSamplePosition = 2;
pub const DAV1D_CHR_VERTICAL: Dav1dChromaSamplePosition = 1;
pub const DAV1D_CHR_UNKNOWN: Dav1dChromaSamplePosition = 0;

pub(crate) type Rav1dChromaSamplePosition = c_uint;
pub(crate) const _RAV1D_CHR_COLOCATED: Rav1dChromaSamplePosition = DAV1D_CHR_COLOCATED;
pub(crate) const _RAV1D_CHR_VERTICAL: Rav1dChromaSamplePosition = DAV1D_CHR_VERTICAL;
pub(crate) const RAV1D_CHR_UNKNOWN: Rav1dChromaSamplePosition = DAV1D_CHR_UNKNOWN;

// Constants from Section 3. "Symbols and abbreviated terms"
pub const DAV1D_MAX_SEGMENTS: u8 = 8;

pub(crate) const RAV1D_MAX_SEGMENTS: u8 = DAV1D_MAX_SEGMENTS;

#[repr(C)]
pub struct Rav1dContentLightLevel {
    pub max_content_light_level: c_int,
    pub max_frame_average_light_level: c_int,
}

pub type Dav1dContentLightLevel = Rav1dContentLightLevel;

#[repr(C)]
pub struct Dav1dMasteringDisplay {
    pub primaries: [[u16; 2]; 3],
    pub white_point: [u16; 2],
    pub max_luminance: u32,
    pub min_luminance: u32,
}

#[repr(C)]
pub(crate) struct Rav1dMasteringDisplay {
    pub primaries: [[u16; 2]; 3],
    pub white_point: [u16; 2],
    pub max_luminance: u32,
    pub min_luminance: u32,
}

impl From<Dav1dMasteringDisplay> for Rav1dMasteringDisplay {
    fn from(value: Dav1dMasteringDisplay) -> Self {
        let Dav1dMasteringDisplay {
            primaries,
            white_point,
            max_luminance,
            min_luminance,
        } = value;
        Self {
            primaries,
            white_point,
            max_luminance,
            min_luminance,
        }
    }
}

impl From<Rav1dMasteringDisplay> for Dav1dMasteringDisplay {
    fn from(value: Rav1dMasteringDisplay) -> Self {
        let Rav1dMasteringDisplay {
            primaries,
            white_point,
            max_luminance,
            min_luminance,
        } = value;
        Self {
            primaries,
            white_point,
            max_luminance,
            min_luminance,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dITUTT35 {
    pub country_code: u8,
    pub country_code_extension_byte: u8,
    pub payload_size: usize,
    pub payload: *mut u8,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dITUTT35 {
    pub country_code: u8,
    pub country_code_extension_byte: u8,
    pub payload_size: usize,
    pub payload: *mut u8,
}

impl From<Dav1dITUTT35> for Rav1dITUTT35 {
    fn from(value: Dav1dITUTT35) -> Self {
        let Dav1dITUTT35 {
            country_code,
            country_code_extension_byte,
            payload_size,
            payload,
        } = value;
        Self {
            country_code,
            country_code_extension_byte,
            payload_size,
            payload,
        }
    }
}

impl From<Rav1dITUTT35> for Dav1dITUTT35 {
    fn from(value: Rav1dITUTT35) -> Self {
        let Rav1dITUTT35 {
            country_code,
            country_code_extension_byte,
            payload_size,
            payload,
        } = value;
        Self {
            country_code,
            country_code_extension_byte,
            payload_size,
            payload,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingPoint {
    pub major_level: c_int,
    pub minor_level: c_int,
    pub initial_display_delay: c_int,
    pub idc: c_int,
    pub tier: c_int,
    pub decoder_model_param_present: c_int,
    pub display_model_param_present: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct Rav1dSequenceHeaderOperatingPoint {
    pub major_level: c_int,
    pub minor_level: c_int,
    pub initial_display_delay: c_int,
    pub idc: c_int,
    pub tier: c_int,
    pub decoder_model_param_present: c_int,
    pub display_model_param_present: c_int,
}

impl From<Dav1dSequenceHeaderOperatingPoint> for Rav1dSequenceHeaderOperatingPoint {
    fn from(value: Dav1dSequenceHeaderOperatingPoint) -> Self {
        let Dav1dSequenceHeaderOperatingPoint {
            major_level,
            minor_level,
            initial_display_delay,
            idc,
            tier,
            decoder_model_param_present,
            display_model_param_present,
        } = value;
        Self {
            major_level,
            minor_level,
            initial_display_delay,
            idc,
            tier,
            decoder_model_param_present,
            display_model_param_present,
        }
    }
}

impl From<Rav1dSequenceHeaderOperatingPoint> for Dav1dSequenceHeaderOperatingPoint {
    fn from(value: Rav1dSequenceHeaderOperatingPoint) -> Self {
        let Rav1dSequenceHeaderOperatingPoint {
            major_level,
            minor_level,
            initial_display_delay,
            idc,
            tier,
            decoder_model_param_present,
            display_model_param_present,
        } = value;
        Self {
            major_level,
            minor_level,
            initial_display_delay,
            idc,
            tier,
            decoder_model_param_present,
            display_model_param_present,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: c_int,
    pub encoder_buffer_delay: c_int,
    pub low_delay_mode: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub(crate) struct Rav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: c_int,
    pub encoder_buffer_delay: c_int,
    pub low_delay_mode: c_int,
}

impl From<Dav1dSequenceHeaderOperatingParameterInfo> for Rav1dSequenceHeaderOperatingParameterInfo {
    fn from(value: Dav1dSequenceHeaderOperatingParameterInfo) -> Self {
        let Dav1dSequenceHeaderOperatingParameterInfo {
            decoder_buffer_delay,
            encoder_buffer_delay,
            low_delay_mode,
        } = value;
        Self {
            decoder_buffer_delay,
            encoder_buffer_delay,
            low_delay_mode,
        }
    }
}

impl From<Rav1dSequenceHeaderOperatingParameterInfo> for Dav1dSequenceHeaderOperatingParameterInfo {
    fn from(value: Rav1dSequenceHeaderOperatingParameterInfo) -> Self {
        let Rav1dSequenceHeaderOperatingParameterInfo {
            decoder_buffer_delay,
            encoder_buffer_delay,
            low_delay_mode,
        } = value;
        Self {
            decoder_buffer_delay,
            encoder_buffer_delay,
            low_delay_mode,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeader {
    pub profile: c_int,
    pub max_width: c_int,
    pub max_height: c_int,
    pub layout: Dav1dPixelLayout,
    pub pri: Dav1dColorPrimaries,
    pub trc: Dav1dTransferCharacteristics,
    pub mtrx: Dav1dMatrixCoefficients,
    pub chr: Dav1dChromaSamplePosition,
    pub hbd: c_int,
    pub color_range: c_int,
    pub num_operating_points: c_int,
    pub operating_points: [Dav1dSequenceHeaderOperatingPoint; 32],
    pub still_picture: c_int,
    pub reduced_still_picture_header: c_int,
    pub timing_info_present: c_int,
    pub num_units_in_tick: c_int,
    pub time_scale: c_int,
    pub equal_picture_interval: c_int,
    pub num_ticks_per_picture: c_uint,
    pub decoder_model_info_present: c_int,
    pub encoder_decoder_buffer_delay_length: c_int,
    pub num_units_in_decoding_tick: c_int,
    pub buffer_removal_delay_length: c_int,
    pub frame_presentation_delay_length: c_int,
    pub display_model_info_present: c_int,
    pub width_n_bits: c_int,
    pub height_n_bits: c_int,
    pub frame_id_numbers_present: c_int,
    pub delta_frame_id_n_bits: c_int,
    pub frame_id_n_bits: c_int,
    pub sb128: c_int,
    pub filter_intra: c_int,
    pub intra_edge_filter: c_int,
    pub inter_intra: c_int,
    pub masked_compound: c_int,
    pub warped_motion: c_int,
    pub dual_filter: c_int,
    pub order_hint: c_int,
    pub jnt_comp: c_int,
    pub ref_frame_mvs: c_int,
    pub screen_content_tools: Dav1dAdaptiveBoolean,
    pub force_integer_mv: Dav1dAdaptiveBoolean,
    pub order_hint_n_bits: c_int,
    pub super_res: c_int,
    pub cdef: c_int,
    pub restoration: c_int,
    pub ss_hor: c_int,
    pub ss_ver: c_int,
    pub monochrome: c_int,
    pub color_description_present: c_int,
    pub separate_uv_delta_q: c_int,
    pub film_grain_present: c_int,
    pub operating_parameter_info: [Dav1dSequenceHeaderOperatingParameterInfo; 32],
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dSequenceHeader {
    pub profile: c_int,
    pub max_width: c_int,
    pub max_height: c_int,
    pub layout: Rav1dPixelLayout,
    pub pri: Rav1dColorPrimaries,
    pub trc: Rav1dTransferCharacteristics,
    pub mtrx: Rav1dMatrixCoefficients,
    pub chr: Rav1dChromaSamplePosition,
    pub hbd: c_int,
    pub color_range: c_int,
    pub num_operating_points: c_int,
    pub operating_points: [Rav1dSequenceHeaderOperatingPoint; 32],
    pub still_picture: c_int,
    pub reduced_still_picture_header: c_int,
    pub timing_info_present: c_int,
    pub num_units_in_tick: c_int,
    pub time_scale: c_int,
    pub equal_picture_interval: c_int,
    pub num_ticks_per_picture: c_uint,
    pub decoder_model_info_present: c_int,
    pub encoder_decoder_buffer_delay_length: c_int,
    pub num_units_in_decoding_tick: c_int,
    pub buffer_removal_delay_length: c_int,
    pub frame_presentation_delay_length: c_int,
    pub display_model_info_present: c_int,
    pub width_n_bits: c_int,
    pub height_n_bits: c_int,
    pub frame_id_numbers_present: c_int,
    pub delta_frame_id_n_bits: c_int,
    pub frame_id_n_bits: c_int,
    pub sb128: c_int,
    pub filter_intra: c_int,
    pub intra_edge_filter: c_int,
    pub inter_intra: c_int,
    pub masked_compound: c_int,
    pub warped_motion: c_int,
    pub dual_filter: c_int,
    pub order_hint: c_int,
    pub jnt_comp: c_int,
    pub ref_frame_mvs: c_int,
    pub screen_content_tools: Rav1dAdaptiveBoolean,
    pub force_integer_mv: Rav1dAdaptiveBoolean,
    pub order_hint_n_bits: c_int,
    pub super_res: c_int,
    pub cdef: c_int,
    pub restoration: c_int,
    pub ss_hor: c_int,
    pub ss_ver: c_int,
    pub monochrome: c_int,
    pub color_description_present: c_int,
    pub separate_uv_delta_q: c_int,
    pub film_grain_present: c_int,
    pub operating_parameter_info: [Rav1dSequenceHeaderOperatingParameterInfo; 32],
}

impl From<Dav1dSequenceHeader> for Rav1dSequenceHeader {
    fn from(value: Dav1dSequenceHeader) -> Self {
        let Dav1dSequenceHeader {
            profile,
            max_width,
            max_height,
            layout,
            pri,
            trc,
            mtrx,
            chr,
            hbd,
            color_range,
            num_operating_points,
            operating_points,
            still_picture,
            reduced_still_picture_header,
            timing_info_present,
            num_units_in_tick,
            time_scale,
            equal_picture_interval,
            num_ticks_per_picture,
            decoder_model_info_present,
            encoder_decoder_buffer_delay_length,
            num_units_in_decoding_tick,
            buffer_removal_delay_length,
            frame_presentation_delay_length,
            display_model_info_present,
            width_n_bits,
            height_n_bits,
            frame_id_numbers_present,
            delta_frame_id_n_bits,
            frame_id_n_bits,
            sb128,
            filter_intra,
            intra_edge_filter,
            inter_intra,
            masked_compound,
            warped_motion,
            dual_filter,
            order_hint,
            jnt_comp,
            ref_frame_mvs,
            screen_content_tools,
            force_integer_mv,
            order_hint_n_bits,
            super_res,
            cdef,
            restoration,
            ss_hor,
            ss_ver,
            monochrome,
            color_description_present,
            separate_uv_delta_q,
            film_grain_present,
            operating_parameter_info,
        } = value;
        Self {
            profile,
            max_width,
            max_height,
            layout,
            pri,
            trc,
            mtrx,
            chr,
            hbd,
            color_range,
            num_operating_points,
            operating_points: operating_points.map(|c| c.into()),
            still_picture,
            reduced_still_picture_header,
            timing_info_present,
            num_units_in_tick,
            time_scale,
            equal_picture_interval,
            num_ticks_per_picture,
            decoder_model_info_present,
            encoder_decoder_buffer_delay_length,
            num_units_in_decoding_tick,
            buffer_removal_delay_length,
            frame_presentation_delay_length,
            display_model_info_present,
            width_n_bits,
            height_n_bits,
            frame_id_numbers_present,
            delta_frame_id_n_bits,
            frame_id_n_bits,
            sb128,
            filter_intra,
            intra_edge_filter,
            inter_intra,
            masked_compound,
            warped_motion,
            dual_filter,
            order_hint,
            jnt_comp,
            ref_frame_mvs,
            screen_content_tools,
            force_integer_mv,
            order_hint_n_bits,
            super_res,
            cdef,
            restoration,
            ss_hor,
            ss_ver,
            monochrome,
            color_description_present,
            separate_uv_delta_q,
            film_grain_present,
            operating_parameter_info: operating_parameter_info.map(|c| c.into()),
        }
    }
}

impl From<Rav1dSequenceHeader> for Dav1dSequenceHeader {
    fn from(value: Rav1dSequenceHeader) -> Self {
        let Rav1dSequenceHeader {
            profile,
            max_width,
            max_height,
            layout,
            pri,
            trc,
            mtrx,
            chr,
            hbd,
            color_range,
            num_operating_points,
            operating_points,
            still_picture,
            reduced_still_picture_header,
            timing_info_present,
            num_units_in_tick,
            time_scale,
            equal_picture_interval,
            num_ticks_per_picture,
            decoder_model_info_present,
            encoder_decoder_buffer_delay_length,
            num_units_in_decoding_tick,
            buffer_removal_delay_length,
            frame_presentation_delay_length,
            display_model_info_present,
            width_n_bits,
            height_n_bits,
            frame_id_numbers_present,
            delta_frame_id_n_bits,
            frame_id_n_bits,
            sb128,
            filter_intra,
            intra_edge_filter,
            inter_intra,
            masked_compound,
            warped_motion,
            dual_filter,
            order_hint,
            jnt_comp,
            ref_frame_mvs,
            screen_content_tools,
            force_integer_mv,
            order_hint_n_bits,
            super_res,
            cdef,
            restoration,
            ss_hor,
            ss_ver,
            monochrome,
            color_description_present,
            separate_uv_delta_q,
            film_grain_present,
            operating_parameter_info,
        } = value;
        Self {
            profile,
            max_width,
            max_height,
            layout,
            pri,
            trc,
            mtrx,
            chr,
            hbd,
            color_range,
            num_operating_points,
            operating_points: operating_points.map(|rust| rust.into()),
            still_picture,
            reduced_still_picture_header,
            timing_info_present,
            num_units_in_tick,
            time_scale,
            equal_picture_interval,
            num_ticks_per_picture,
            decoder_model_info_present,
            encoder_decoder_buffer_delay_length,
            num_units_in_decoding_tick,
            buffer_removal_delay_length,
            frame_presentation_delay_length,
            display_model_info_present,
            width_n_bits,
            height_n_bits,
            frame_id_numbers_present,
            delta_frame_id_n_bits,
            frame_id_n_bits,
            sb128,
            filter_intra,
            intra_edge_filter,
            inter_intra,
            masked_compound,
            warped_motion,
            dual_filter,
            order_hint,
            jnt_comp,
            ref_frame_mvs,
            screen_content_tools,
            force_integer_mv,
            order_hint_n_bits,
            super_res,
            cdef,
            restoration,
            ss_hor,
            ss_ver,
            monochrome,
            color_description_present,
            separate_uv_delta_q,
            film_grain_present,
            operating_parameter_info: operating_parameter_info.map(|rust| rust.into()),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dSegmentationData {
    pub delta_q: c_int,
    pub delta_lf_y_v: c_int,
    pub delta_lf_y_h: c_int,
    pub delta_lf_u: c_int,
    pub delta_lf_v: c_int,
    pub r#ref: c_int,
    pub skip: c_int,
    pub globalmv: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dSegmentationData {
    pub delta_q: c_int,
    pub delta_lf_y_v: c_int,
    pub delta_lf_y_h: c_int,
    pub delta_lf_u: c_int,
    pub delta_lf_v: c_int,
    pub r#ref: c_int,
    pub skip: c_int,
    pub globalmv: c_int,
}

impl From<Dav1dSegmentationData> for Rav1dSegmentationData {
    fn from(value: Dav1dSegmentationData) -> Self {
        let Dav1dSegmentationData {
            delta_q,
            delta_lf_y_v,
            delta_lf_y_h,
            delta_lf_u,
            delta_lf_v,
            r#ref,
            skip,
            globalmv,
        } = value;
        Self {
            delta_q,
            delta_lf_y_v,
            delta_lf_y_h,
            delta_lf_u,
            delta_lf_v,
            r#ref,
            skip,
            globalmv,
        }
    }
}

impl From<Rav1dSegmentationData> for Dav1dSegmentationData {
    fn from(value: Rav1dSegmentationData) -> Self {
        let Rav1dSegmentationData {
            delta_q,
            delta_lf_y_v,
            delta_lf_y_h,
            delta_lf_u,
            delta_lf_v,
            r#ref,
            skip,
            globalmv,
        } = value;
        Self {
            delta_q,
            delta_lf_y_v,
            delta_lf_y_h,
            delta_lf_u,
            delta_lf_v,
            r#ref,
            skip,
            globalmv,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; 8],
    pub preskip: c_int,
    pub last_active_segid: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dSegmentationDataSet {
    pub d: [Rav1dSegmentationData; 8],
    pub preskip: c_int,
    pub last_active_segid: c_int,
}

impl From<Dav1dSegmentationDataSet> for Rav1dSegmentationDataSet {
    fn from(value: Dav1dSegmentationDataSet) -> Self {
        let Dav1dSegmentationDataSet {
            d,
            preskip,
            last_active_segid,
        } = value;
        Self {
            d: d.map(|c| c.into()),
            preskip,
            last_active_segid,
        }
    }
}

impl From<Rav1dSegmentationDataSet> for Dav1dSegmentationDataSet {
    fn from(value: Rav1dSegmentationDataSet) -> Self {
        let Rav1dSegmentationDataSet {
            d,
            preskip,
            last_active_segid,
        } = value;
        Self {
            d: d.map(|rust| rust.into()),
            preskip,
            last_active_segid,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [c_int; 2],
    pub ref_delta: [c_int; 8],
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dLoopfilterModeRefDeltas {
    pub mode_delta: [c_int; 2],
    pub ref_delta: [c_int; 8],
}

impl From<Dav1dLoopfilterModeRefDeltas> for Rav1dLoopfilterModeRefDeltas {
    fn from(value: Dav1dLoopfilterModeRefDeltas) -> Self {
        let Dav1dLoopfilterModeRefDeltas {
            mode_delta,
            ref_delta,
        } = value;
        Self {
            mode_delta,
            ref_delta,
        }
    }
}

impl From<Rav1dLoopfilterModeRefDeltas> for Dav1dLoopfilterModeRefDeltas {
    fn from(value: Rav1dLoopfilterModeRefDeltas) -> Self {
        let Rav1dLoopfilterModeRefDeltas {
            mode_delta,
            ref_delta,
        } = value;
        Self {
            mode_delta,
            ref_delta,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFilmGrainData {
    pub seed: c_uint,
    pub num_y_points: c_int,
    pub y_points: [[u8; 2]; 14],
    pub chroma_scaling_from_luma: c_int,
    pub num_uv_points: [c_int; 2],
    pub uv_points: [[[u8; 2]; 10]; 2],
    pub scaling_shift: c_int,
    pub ar_coeff_lag: c_int,
    pub ar_coeffs_y: [i8; 24],
    pub ar_coeffs_uv: [[i8; 28]; 2],
    pub ar_coeff_shift: u64,
    pub grain_scale_shift: c_int,
    pub uv_mult: [c_int; 2],
    pub uv_luma_mult: [c_int; 2],
    pub uv_offset: [c_int; 2],
    pub overlap_flag: c_int,
    pub clip_to_restricted_range: c_int,
}

pub type Dav1dFilmGrainData = Rav1dFilmGrainData;

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_film_grain {
    pub data: Dav1dFilmGrainData,
    pub present: c_int,
    pub update: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_film_grain {
    pub data: Rav1dFilmGrainData,
    pub present: c_int,
    pub update: c_int,
}

impl From<Dav1dFrameHeader_film_grain> for Rav1dFrameHeader_film_grain {
    fn from(value: Dav1dFrameHeader_film_grain) -> Self {
        let Dav1dFrameHeader_film_grain {
            data,
            present,
            update,
        } = value;
        Self {
            data,
            present,
            update,
        }
    }
}

impl From<Rav1dFrameHeader_film_grain> for Dav1dFrameHeader_film_grain {
    fn from(value: Rav1dFrameHeader_film_grain) -> Self {
        let Rav1dFrameHeader_film_grain {
            data,
            present,
            update,
        } = value;
        Self {
            data,
            present,
            update,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: c_int,
}

impl From<Dav1dFrameHeaderOperatingPoint> for Rav1dFrameHeaderOperatingPoint {
    fn from(value: Dav1dFrameHeaderOperatingPoint) -> Self {
        let Dav1dFrameHeaderOperatingPoint {
            buffer_removal_time,
        } = value;
        Self {
            buffer_removal_time,
        }
    }
}

impl From<Rav1dFrameHeaderOperatingPoint> for Dav1dFrameHeaderOperatingPoint {
    fn from(value: Rav1dFrameHeaderOperatingPoint) -> Self {
        let Rav1dFrameHeaderOperatingPoint {
            buffer_removal_time,
        } = value;
        Self {
            buffer_removal_time,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_super_res {
    pub width_scale_denominator: c_int,
    pub enabled: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_super_res {
    pub width_scale_denominator: c_int,
    pub enabled: c_int,
}

impl From<Dav1dFrameHeader_super_res> for Rav1dFrameHeader_super_res {
    fn from(value: Dav1dFrameHeader_super_res) -> Self {
        let Dav1dFrameHeader_super_res {
            width_scale_denominator,
            enabled,
        } = value;
        Self {
            width_scale_denominator,
            enabled,
        }
    }
}

impl From<Rav1dFrameHeader_super_res> for Dav1dFrameHeader_super_res {
    fn from(value: Rav1dFrameHeader_super_res) -> Self {
        let Rav1dFrameHeader_super_res {
            width_scale_denominator,
            enabled,
        } = value;
        Self {
            width_scale_denominator,
            enabled,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_tiling {
    pub uniform: c_int,
    pub n_bytes: c_uint,
    pub min_log2_cols: c_int,
    pub max_log2_cols: c_int,
    pub log2_cols: c_int,
    pub cols: c_int,
    pub min_log2_rows: c_int,
    pub max_log2_rows: c_int,
    pub log2_rows: c_int,
    pub rows: c_int,
    pub col_start_sb: [u16; 65],
    pub row_start_sb: [u16; 65],
    pub update: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_tiling {
    pub uniform: c_int,
    pub n_bytes: c_uint,
    pub min_log2_cols: c_int,
    pub max_log2_cols: c_int,
    pub log2_cols: c_int,
    pub cols: c_int,
    pub min_log2_rows: c_int,
    pub max_log2_rows: c_int,
    pub log2_rows: c_int,
    pub rows: c_int,
    pub col_start_sb: [u16; 65],
    pub row_start_sb: [u16; 65],
    pub update: c_int,
}

impl From<Dav1dFrameHeader_tiling> for Rav1dFrameHeader_tiling {
    fn from(value: Dav1dFrameHeader_tiling) -> Self {
        let Dav1dFrameHeader_tiling {
            uniform,
            n_bytes,
            min_log2_cols,
            max_log2_cols,
            log2_cols,
            cols,
            min_log2_rows,
            max_log2_rows,
            log2_rows,
            rows,
            col_start_sb,
            row_start_sb,
            update,
        } = value;
        Self {
            uniform,
            n_bytes,
            min_log2_cols,
            max_log2_cols,
            log2_cols,
            cols,
            min_log2_rows,
            max_log2_rows,
            log2_rows,
            rows,
            col_start_sb,
            row_start_sb,
            update,
        }
    }
}

impl From<Rav1dFrameHeader_tiling> for Dav1dFrameHeader_tiling {
    fn from(value: Rav1dFrameHeader_tiling) -> Self {
        let Rav1dFrameHeader_tiling {
            uniform,
            n_bytes,
            min_log2_cols,
            max_log2_cols,
            log2_cols,
            cols,
            min_log2_rows,
            max_log2_rows,
            log2_rows,
            rows,
            col_start_sb,
            row_start_sb,
            update,
        } = value;
        Self {
            uniform,
            n_bytes,
            min_log2_cols,
            max_log2_cols,
            log2_cols,
            cols,
            min_log2_rows,
            max_log2_rows,
            log2_rows,
            rows,
            col_start_sb,
            row_start_sb,
            update,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_quant {
    pub yac: c_int,
    pub ydc_delta: c_int,
    pub udc_delta: c_int,
    pub uac_delta: c_int,
    pub vdc_delta: c_int,
    pub vac_delta: c_int,
    pub qm: c_int,
    pub qm_y: c_int,
    pub qm_u: c_int,
    pub qm_v: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_quant {
    pub yac: c_int,
    pub ydc_delta: c_int,
    pub udc_delta: c_int,
    pub uac_delta: c_int,
    pub vdc_delta: c_int,
    pub vac_delta: c_int,
    pub qm: c_int,
    pub qm_y: c_int,
    pub qm_u: c_int,
    pub qm_v: c_int,
}

impl From<Dav1dFrameHeader_quant> for Rav1dFrameHeader_quant {
    fn from(value: Dav1dFrameHeader_quant) -> Self {
        let Dav1dFrameHeader_quant {
            yac,
            ydc_delta,
            udc_delta,
            uac_delta,
            vdc_delta,
            vac_delta,
            qm,
            qm_y,
            qm_u,
            qm_v,
        } = value;
        Self {
            yac,
            ydc_delta,
            udc_delta,
            uac_delta,
            vdc_delta,
            vac_delta,
            qm,
            qm_y,
            qm_u,
            qm_v,
        }
    }
}

impl From<Rav1dFrameHeader_quant> for Dav1dFrameHeader_quant {
    fn from(value: Rav1dFrameHeader_quant) -> Self {
        let Rav1dFrameHeader_quant {
            yac,
            ydc_delta,
            udc_delta,
            uac_delta,
            vdc_delta,
            vac_delta,
            qm,
            qm_y,
            qm_u,
            qm_v,
        } = value;
        Self {
            yac,
            ydc_delta,
            udc_delta,
            uac_delta,
            vdc_delta,
            vac_delta,
            qm,
            qm_y,
            qm_u,
            qm_v,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_segmentation {
    pub enabled: c_int,
    pub update_map: c_int,
    pub temporal: c_int,
    pub update_data: c_int,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [c_int; 8],
    pub qidx: [c_int; 8],
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_segmentation {
    pub enabled: c_int,
    pub update_map: c_int,
    pub temporal: c_int,
    pub update_data: c_int,
    pub seg_data: Rav1dSegmentationDataSet,
    pub lossless: [c_int; 8],
    pub qidx: [c_int; 8],
}

impl From<Dav1dFrameHeader_segmentation> for Rav1dFrameHeader_segmentation {
    fn from(value: Dav1dFrameHeader_segmentation) -> Self {
        let Dav1dFrameHeader_segmentation {
            enabled,
            update_map,
            temporal,
            update_data,
            seg_data,
            lossless,
            qidx,
        } = value;
        Self {
            enabled,
            update_map,
            temporal,
            update_data,
            seg_data: seg_data.into(),
            lossless,
            qidx,
        }
    }
}

impl From<Rav1dFrameHeader_segmentation> for Dav1dFrameHeader_segmentation {
    fn from(value: Rav1dFrameHeader_segmentation) -> Self {
        let Rav1dFrameHeader_segmentation {
            enabled,
            update_map,
            temporal,
            update_data,
            seg_data,
            lossless,
            qidx,
        } = value;
        Self {
            enabled,
            update_map,
            temporal,
            update_data,
            seg_data: seg_data.into(),
            lossless,
            qidx,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_delta_q {
    pub present: c_int,
    pub res_log2: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_delta_q {
    pub present: c_int,
    pub res_log2: c_int,
}

impl From<Dav1dFrameHeader_delta_q> for Rav1dFrameHeader_delta_q {
    fn from(value: Dav1dFrameHeader_delta_q) -> Self {
        let Dav1dFrameHeader_delta_q { present, res_log2 } = value;
        Self { present, res_log2 }
    }
}

impl From<Rav1dFrameHeader_delta_q> for Dav1dFrameHeader_delta_q {
    fn from(value: Rav1dFrameHeader_delta_q) -> Self {
        let Rav1dFrameHeader_delta_q { present, res_log2 } = value;
        Self { present, res_log2 }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_delta_lf {
    pub present: c_int,
    pub res_log2: c_int,
    pub multi: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_delta_lf {
    pub present: c_int,
    pub res_log2: c_int,
    pub multi: c_int,
}

impl From<Dav1dFrameHeader_delta_lf> for Rav1dFrameHeader_delta_lf {
    fn from(value: Dav1dFrameHeader_delta_lf) -> Self {
        let Dav1dFrameHeader_delta_lf {
            present,
            res_log2,
            multi,
        } = value;
        Self {
            present,
            res_log2,
            multi,
        }
    }
}

impl From<Rav1dFrameHeader_delta_lf> for Dav1dFrameHeader_delta_lf {
    fn from(value: Rav1dFrameHeader_delta_lf) -> Self {
        let Rav1dFrameHeader_delta_lf {
            present,
            res_log2,
            multi,
        } = value;
        Self {
            present,
            res_log2,
            multi,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_delta {
    pub q: Dav1dFrameHeader_delta_q,
    pub lf: Dav1dFrameHeader_delta_lf,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_delta {
    pub q: Rav1dFrameHeader_delta_q,
    pub lf: Rav1dFrameHeader_delta_lf,
}

impl From<Dav1dFrameHeader_delta> for Rav1dFrameHeader_delta {
    fn from(value: Dav1dFrameHeader_delta) -> Self {
        let Dav1dFrameHeader_delta { q, lf } = value;
        Self {
            q: q.into(),
            lf: lf.into(),
        }
    }
}

impl From<Rav1dFrameHeader_delta> for Dav1dFrameHeader_delta {
    fn from(value: Rav1dFrameHeader_delta) -> Self {
        let Rav1dFrameHeader_delta { q, lf } = value;
        Self {
            q: q.into(),
            lf: lf.into(),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_loopfilter {
    pub level_y: [c_int; 2],
    pub level_u: c_int,
    pub level_v: c_int,
    pub mode_ref_delta_enabled: c_int,
    pub mode_ref_delta_update: c_int,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_loopfilter {
    pub level_y: [c_int; 2],
    pub level_u: c_int,
    pub level_v: c_int,
    pub mode_ref_delta_enabled: c_int,
    pub mode_ref_delta_update: c_int,
    pub mode_ref_deltas: Rav1dLoopfilterModeRefDeltas,
    pub sharpness: c_int,
}

impl From<Dav1dFrameHeader_loopfilter> for Rav1dFrameHeader_loopfilter {
    fn from(value: Dav1dFrameHeader_loopfilter) -> Self {
        let Dav1dFrameHeader_loopfilter {
            level_y,
            level_u,
            level_v,
            mode_ref_delta_enabled,
            mode_ref_delta_update,
            mode_ref_deltas,
            sharpness,
        } = value;
        Self {
            level_y,
            level_u,
            level_v,
            mode_ref_delta_enabled,
            mode_ref_delta_update,
            mode_ref_deltas: mode_ref_deltas.into(),
            sharpness,
        }
    }
}

impl From<Rav1dFrameHeader_loopfilter> for Dav1dFrameHeader_loopfilter {
    fn from(value: Rav1dFrameHeader_loopfilter) -> Self {
        let Rav1dFrameHeader_loopfilter {
            level_y,
            level_u,
            level_v,
            mode_ref_delta_enabled,
            mode_ref_delta_update,
            mode_ref_deltas,
            sharpness,
        } = value;
        Self {
            level_y,
            level_u,
            level_v,
            mode_ref_delta_enabled,
            mode_ref_delta_update,
            mode_ref_deltas: mode_ref_deltas.into(),
            sharpness,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_cdef {
    pub damping: c_int,
    pub n_bits: c_int,
    pub y_strength: [c_int; 8],
    pub uv_strength: [c_int; 8],
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_cdef {
    pub damping: c_int,
    pub n_bits: c_int,
    pub y_strength: [c_int; 8],
    pub uv_strength: [c_int; 8],
}

impl From<Dav1dFrameHeader_cdef> for Rav1dFrameHeader_cdef {
    fn from(value: Dav1dFrameHeader_cdef) -> Self {
        let Dav1dFrameHeader_cdef {
            damping,
            n_bits,
            y_strength,
            uv_strength,
        } = value;
        Self {
            damping,
            n_bits,
            y_strength,
            uv_strength,
        }
    }
}

impl From<Rav1dFrameHeader_cdef> for Dav1dFrameHeader_cdef {
    fn from(value: Rav1dFrameHeader_cdef) -> Self {
        let Rav1dFrameHeader_cdef {
            damping,
            n_bits,
            y_strength,
            uv_strength,
        } = value;
        Self {
            damping,
            n_bits,
            y_strength,
            uv_strength,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_restoration {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [c_int; 2],
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader_restoration {
    pub type_0: [Rav1dRestorationType; 3],
    pub unit_size: [c_int; 2],
}

impl From<Dav1dFrameHeader_restoration> for Rav1dFrameHeader_restoration {
    fn from(value: Dav1dFrameHeader_restoration) -> Self {
        let Dav1dFrameHeader_restoration { type_0, unit_size } = value;
        Self { type_0, unit_size }
    }
}

impl From<Rav1dFrameHeader_restoration> for Dav1dFrameHeader_restoration {
    fn from(value: Rav1dFrameHeader_restoration) -> Self {
        let Rav1dFrameHeader_restoration { type_0, unit_size } = value;
        Self { type_0, unit_size }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader {
    pub film_grain: Dav1dFrameHeader_film_grain,
    pub frame_type: Dav1dFrameType,
    pub width: [c_int; 2],
    pub height: c_int,
    pub frame_offset: c_int,
    pub temporal_id: c_int,
    pub spatial_id: c_int,
    pub show_existing_frame: c_int,
    pub existing_frame_idx: c_int,
    pub frame_id: c_int,
    pub frame_presentation_delay: c_int,
    pub show_frame: c_int,
    pub showable_frame: c_int,
    pub error_resilient_mode: c_int,
    pub disable_cdf_update: c_int,
    pub allow_screen_content_tools: c_int,
    pub force_integer_mv: c_int,
    pub frame_size_override: c_int,
    pub primary_ref_frame: c_int,
    pub buffer_removal_time_present: c_int,
    pub operating_points: [Dav1dFrameHeaderOperatingPoint; 32],
    pub refresh_frame_flags: c_int,
    pub render_width: c_int,
    pub render_height: c_int,
    pub super_res: Dav1dFrameHeader_super_res,
    pub have_render_size: c_int,
    pub allow_intrabc: c_int,
    pub frame_ref_short_signaling: c_int,
    pub refidx: [c_int; 7],
    pub hp: c_int,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: c_int,
    pub use_ref_frame_mvs: c_int,
    pub refresh_context: c_int,
    pub tiling: Dav1dFrameHeader_tiling,
    pub quant: Dav1dFrameHeader_quant,
    pub segmentation: Dav1dFrameHeader_segmentation,
    pub delta: Dav1dFrameHeader_delta,
    pub all_lossless: c_int,
    pub loopfilter: Dav1dFrameHeader_loopfilter,
    pub cdef: Dav1dFrameHeader_cdef,
    pub restoration: Dav1dFrameHeader_restoration,
    pub txfm_mode: Dav1dTxfmMode,
    pub switchable_comp_refs: c_int,
    pub skip_mode_allowed: c_int,
    pub skip_mode_enabled: c_int,
    pub skip_mode_refs: [c_int; 2],
    pub warp_motion: c_int,
    pub reduced_txtp_set: c_int,
    pub gmv: [Dav1dWarpedMotionParams; 7],
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dFrameHeader {
    pub film_grain: Rav1dFrameHeader_film_grain,
    pub frame_type: Rav1dFrameType,
    pub width: [c_int; 2],
    pub height: c_int,
    pub frame_offset: c_int,
    pub temporal_id: c_int,
    pub spatial_id: c_int,
    pub show_existing_frame: c_int,
    pub existing_frame_idx: c_int,
    pub frame_id: c_int,
    pub frame_presentation_delay: c_int,
    pub show_frame: c_int,
    pub showable_frame: c_int,
    pub error_resilient_mode: c_int,
    pub disable_cdf_update: c_int,
    pub allow_screen_content_tools: c_int,
    pub force_integer_mv: c_int,
    pub frame_size_override: c_int,
    pub primary_ref_frame: c_int,
    pub buffer_removal_time_present: c_int,
    pub operating_points: [Rav1dFrameHeaderOperatingPoint; 32],
    pub refresh_frame_flags: c_int,
    pub render_width: c_int,
    pub render_height: c_int,
    pub super_res: Rav1dFrameHeader_super_res,
    pub have_render_size: c_int,
    pub allow_intrabc: c_int,
    pub frame_ref_short_signaling: c_int,
    pub refidx: [c_int; 7],
    pub hp: c_int,
    pub subpel_filter_mode: Rav1dFilterMode,
    pub switchable_motion_mode: c_int,
    pub use_ref_frame_mvs: c_int,
    pub refresh_context: c_int,
    pub tiling: Rav1dFrameHeader_tiling,
    pub quant: Rav1dFrameHeader_quant,
    pub segmentation: Rav1dFrameHeader_segmentation,
    pub delta: Rav1dFrameHeader_delta,
    pub all_lossless: c_int,
    pub loopfilter: Rav1dFrameHeader_loopfilter,
    pub cdef: Rav1dFrameHeader_cdef,
    pub restoration: Rav1dFrameHeader_restoration,
    pub txfm_mode: Rav1dTxfmMode,
    pub switchable_comp_refs: c_int,
    pub skip_mode_allowed: c_int,
    pub skip_mode_enabled: c_int,
    pub skip_mode_refs: [c_int; 2],
    pub warp_motion: c_int,
    pub reduced_txtp_set: c_int,
    pub gmv: [Rav1dWarpedMotionParams; 7],
}

impl From<Dav1dFrameHeader> for Rav1dFrameHeader {
    fn from(value: Dav1dFrameHeader) -> Self {
        let Dav1dFrameHeader {
            film_grain,
            frame_type,
            width,
            height,
            frame_offset,
            temporal_id,
            spatial_id,
            show_existing_frame,
            existing_frame_idx,
            frame_id,
            frame_presentation_delay,
            show_frame,
            showable_frame,
            error_resilient_mode,
            disable_cdf_update,
            allow_screen_content_tools,
            force_integer_mv,
            frame_size_override,
            primary_ref_frame,
            buffer_removal_time_present,
            operating_points,
            refresh_frame_flags,
            render_width,
            render_height,
            super_res,
            have_render_size,
            allow_intrabc,
            frame_ref_short_signaling,
            refidx,
            hp,
            subpel_filter_mode,
            switchable_motion_mode,
            use_ref_frame_mvs,
            refresh_context,
            tiling,
            quant,
            segmentation,
            delta,
            all_lossless,
            loopfilter,
            cdef,
            restoration,
            txfm_mode,
            switchable_comp_refs,
            skip_mode_allowed,
            skip_mode_enabled,
            skip_mode_refs,
            warp_motion,
            reduced_txtp_set,
            gmv,
        } = value;
        Self {
            film_grain: film_grain.into(),
            frame_type,
            width,
            height,
            frame_offset,
            temporal_id,
            spatial_id,
            show_existing_frame,
            existing_frame_idx,
            frame_id,
            frame_presentation_delay,
            show_frame,
            showable_frame,
            error_resilient_mode,
            disable_cdf_update,
            allow_screen_content_tools,
            force_integer_mv,
            frame_size_override,
            primary_ref_frame,
            buffer_removal_time_present,
            operating_points: operating_points.map(|c| c.into()),
            refresh_frame_flags,
            render_width,
            render_height,
            super_res: super_res.into(),
            have_render_size,
            allow_intrabc,
            frame_ref_short_signaling,
            refidx,
            hp,
            subpel_filter_mode,
            switchable_motion_mode,
            use_ref_frame_mvs,
            refresh_context,
            tiling: tiling.into(),
            quant: quant.into(),
            segmentation: segmentation.into(),
            delta: delta.into(),
            all_lossless,
            loopfilter: loopfilter.into(),
            cdef: cdef.into(),
            restoration: restoration.into(),
            txfm_mode,
            switchable_comp_refs,
            skip_mode_allowed,
            skip_mode_enabled,
            skip_mode_refs,
            warp_motion,
            reduced_txtp_set,
            gmv: gmv.map(|c| c.into()),
        }
    }
}

impl From<Rav1dFrameHeader> for Dav1dFrameHeader {
    fn from(value: Rav1dFrameHeader) -> Self {
        let Rav1dFrameHeader {
            film_grain,
            frame_type,
            width,
            height,
            frame_offset,
            temporal_id,
            spatial_id,
            show_existing_frame,
            existing_frame_idx,
            frame_id,
            frame_presentation_delay,
            show_frame,
            showable_frame,
            error_resilient_mode,
            disable_cdf_update,
            allow_screen_content_tools,
            force_integer_mv,
            frame_size_override,
            primary_ref_frame,
            buffer_removal_time_present,
            operating_points,
            refresh_frame_flags,
            render_width,
            render_height,
            super_res,
            have_render_size,
            allow_intrabc,
            frame_ref_short_signaling,
            refidx,
            hp,
            subpel_filter_mode,
            switchable_motion_mode,
            use_ref_frame_mvs,
            refresh_context,
            tiling,
            quant,
            segmentation,
            delta,
            all_lossless,
            loopfilter,
            cdef,
            restoration,
            txfm_mode,
            switchable_comp_refs,
            skip_mode_allowed,
            skip_mode_enabled,
            skip_mode_refs,
            warp_motion,
            reduced_txtp_set,
            gmv,
        } = value;
        Self {
            film_grain: film_grain.into(),
            frame_type,
            width,
            height,
            frame_offset,
            temporal_id,
            spatial_id,
            show_existing_frame,
            existing_frame_idx,
            frame_id,
            frame_presentation_delay,
            show_frame,
            showable_frame,
            error_resilient_mode,
            disable_cdf_update,
            allow_screen_content_tools,
            force_integer_mv,
            frame_size_override,
            primary_ref_frame,
            buffer_removal_time_present,
            operating_points: operating_points.map(|rust| rust.into()),
            refresh_frame_flags,
            render_width,
            render_height,
            super_res: super_res.into(),
            have_render_size,
            allow_intrabc,
            frame_ref_short_signaling,
            refidx,
            hp,
            subpel_filter_mode,
            switchable_motion_mode,
            use_ref_frame_mvs,
            refresh_context,
            tiling: tiling.into(),
            quant: quant.into(),
            segmentation: segmentation.into(),
            delta: delta.into(),
            all_lossless,
            loopfilter: loopfilter.into(),
            cdef: cdef.into(),
            restoration: restoration.into(),
            txfm_mode,
            switchable_comp_refs,
            skip_mode_allowed,
            skip_mode_enabled,
            skip_mode_refs,
            warp_motion,
            reduced_txtp_set,
            gmv: gmv.map(|rust| rust.into()),
        }
    }
}
