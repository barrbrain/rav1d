pub type ObuMetaType = libc::c_uint;
pub const OBU_META_TIMECODE: ObuMetaType = 5;
pub const OBU_META_ITUT_T35: ObuMetaType = 4;
pub const OBU_META_SCALABILITY: ObuMetaType = 3;
pub const OBU_META_HDR_MDCV: ObuMetaType = 2;
pub const OBU_META_HDR_CLL: ObuMetaType = 1;
pub type TxfmSize = libc::c_uint;
pub const N_TX_SIZES: TxfmSize = 5;
pub const TX_64X64: TxfmSize = 4;
pub const TX_32X32: TxfmSize = 3;
pub const TX_16X16: TxfmSize = 2;
pub const TX_8X8: TxfmSize = 1;
pub const TX_4X4: TxfmSize = 0;
pub type BlockLevel = libc::c_uint;
pub const N_BL_LEVELS: BlockLevel = 5;
pub const BL_8X8: BlockLevel = 4;
pub const BL_16X16: BlockLevel = 3;
pub const BL_32X32: BlockLevel = 2;
pub const BL_64X64: BlockLevel = 1;
pub const BL_128X128: BlockLevel = 0;
pub type RectTxfmSize = libc::c_uint;
pub const N_RECT_TX_SIZES: RectTxfmSize = 19;
pub const RTX_64X16: RectTxfmSize = 18;
pub const RTX_16X64: RectTxfmSize = 17;
pub const RTX_32X8: RectTxfmSize = 16;
pub const RTX_8X32: RectTxfmSize = 15;
pub const RTX_16X4: RectTxfmSize = 14;
pub const RTX_4X16: RectTxfmSize = 13;
pub const RTX_64X32: RectTxfmSize = 12;
pub const RTX_32X64: RectTxfmSize = 11;
pub const RTX_32X16: RectTxfmSize = 10;
pub const RTX_16X32: RectTxfmSize = 9;
pub const RTX_16X8: RectTxfmSize = 8;
pub const RTX_8X16: RectTxfmSize = 7;
pub const RTX_8X4: RectTxfmSize = 6;
pub const RTX_4X8: RectTxfmSize = 5;
pub type TxfmType = libc::c_uint;
pub const N_TX_TYPES_PLUS_LL: TxfmType = 17;
pub const WHT_WHT: TxfmType = 16;
pub const N_TX_TYPES: TxfmType = 16;
pub const H_FLIPADST: TxfmType = 15;
pub const V_FLIPADST: TxfmType = 14;
pub const H_ADST: TxfmType = 13;
pub const V_ADST: TxfmType = 12;
pub const H_DCT: TxfmType = 11;
pub const V_DCT: TxfmType = 10;
pub const IDTX: TxfmType = 9;
pub const FLIPADST_ADST: TxfmType = 8;
pub const ADST_FLIPADST: TxfmType = 7;
pub const FLIPADST_FLIPADST: TxfmType = 6;
pub const DCT_FLIPADST: TxfmType = 5;
pub const FLIPADST_DCT: TxfmType = 4;
pub const ADST_ADST: TxfmType = 3;
pub const DCT_ADST: TxfmType = 2;
pub const ADST_DCT: TxfmType = 1;
pub const DCT_DCT: TxfmType = 0;
