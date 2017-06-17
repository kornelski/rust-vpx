pub const VPX_IMAGE_ABI_VERSION: ::std::os::raw::c_uint = 3;
pub const VPX_IMG_FMT_PLANAR: ::std::os::raw::c_uint = 256;
pub const VPX_IMG_FMT_UV_FLIP: ::std::os::raw::c_uint = 512;
pub const VPX_IMG_FMT_HAS_ALPHA: ::std::os::raw::c_uint = 1024;
pub const VPX_IMG_FMT_HIGHBITDEPTH: ::std::os::raw::c_uint = 2048;
pub const VPX_PLANE_PACKED: ::std::os::raw::c_uint = 0;
pub const VPX_PLANE_Y: ::std::os::raw::c_uint = 0;
pub const VPX_PLANE_U: ::std::os::raw::c_uint = 1;
pub const VPX_PLANE_V: ::std::os::raw::c_uint = 2;
pub const VPX_PLANE_ALPHA: ::std::os::raw::c_uint = 3;
pub const VPX_CODEC_ABI_VERSION: ::std::os::raw::c_uint = 6;
pub const VPX_CODEC_CAP_DECODER: ::std::os::raw::c_uint = 1;
pub const VPX_CODEC_CAP_ENCODER: ::std::os::raw::c_uint = 2;
pub const VPX_TS_MAX_PERIODICITY: ::std::os::raw::c_uint = 16;
pub const VPX_TS_MAX_LAYERS: ::std::os::raw::c_uint = 5;
pub const MAX_PERIODICITY: ::std::os::raw::c_uint = 16;
pub const MAX_LAYERS: ::std::os::raw::c_uint = 5;
pub const VPX_SS_MAX_LAYERS: ::std::os::raw::c_uint = 5;
pub const VPX_SS_DEFAULT_LAYERS: ::std::os::raw::c_uint = 1;
pub const VPX_ENCODER_ABI_VERSION: ::std::os::raw::c_uint = 10;
pub const VPX_CODEC_CAP_PSNR: ::std::os::raw::c_uint = 65536;
pub const VPX_CODEC_CAP_OUTPUT_PARTITION: ::std::os::raw::c_uint = 131072;
pub const VPX_CODEC_CAP_HIGHBITDEPTH: ::std::os::raw::c_uint = 262144;
pub const VPX_CODEC_USE_PSNR: ::std::os::raw::c_uint = 65536;
pub const VPX_CODEC_USE_OUTPUT_PARTITION: ::std::os::raw::c_uint = 131072;
pub const VPX_CODEC_USE_HIGHBITDEPTH: ::std::os::raw::c_uint = 262144;
pub const VPX_FRAME_IS_KEY: ::std::os::raw::c_uint = 1;
pub const VPX_FRAME_IS_DROPPABLE: ::std::os::raw::c_uint = 2;
pub const VPX_FRAME_IS_INVISIBLE: ::std::os::raw::c_uint = 4;
pub const VPX_FRAME_IS_FRAGMENT: ::std::os::raw::c_uint = 8;
pub const VPX_ERROR_RESILIENT_DEFAULT: ::std::os::raw::c_uint = 1;
pub const VPX_ERROR_RESILIENT_PARTITIONS: ::std::os::raw::c_uint = 2;
pub const VPX_EFLAG_FORCE_KF: ::std::os::raw::c_uint = 1;
pub const VPX_DL_REALTIME: ::std::os::raw::c_uint = 1;
pub const VPX_DL_GOOD_QUALITY: ::std::os::raw::c_uint = 1000000;
pub const VPX_DL_BEST_QUALITY: ::std::os::raw::c_uint = 0;
pub const VP8_EFLAG_NO_REF_LAST: ::std::os::raw::c_uint = 65536;
pub const VP8_EFLAG_NO_REF_GF: ::std::os::raw::c_uint = 131072;
pub const VP8_EFLAG_NO_REF_ARF: ::std::os::raw::c_uint = 2097152;
pub const VP8_EFLAG_NO_UPD_LAST: ::std::os::raw::c_uint = 262144;
pub const VP8_EFLAG_NO_UPD_GF: ::std::os::raw::c_uint = 4194304;
pub const VP8_EFLAG_NO_UPD_ARF: ::std::os::raw::c_uint = 8388608;
pub const VP8_EFLAG_FORCE_GF: ::std::os::raw::c_uint = 524288;
pub const VP8_EFLAG_FORCE_ARF: ::std::os::raw::c_uint = 16777216;
pub const VP8_EFLAG_NO_UPD_ENTROPY: ::std::os::raw::c_uint = 1048576;
pub const VPX_MAXIMUM_WORK_BUFFERS: ::std::os::raw::c_uint = 8;
pub const VP9_MAXIMUM_REF_BUFFERS: ::std::os::raw::c_uint = 8;
pub const VPX_DECODER_ABI_VERSION: ::std::os::raw::c_uint = 9;
pub const VPX_CODEC_CAP_PUT_SLICE: ::std::os::raw::c_uint = 65536;
pub const VPX_CODEC_CAP_PUT_FRAME: ::std::os::raw::c_uint = 131072;
pub const VPX_CODEC_CAP_POSTPROC: ::std::os::raw::c_uint = 262144;
pub const VPX_CODEC_CAP_ERROR_CONCEALMENT: ::std::os::raw::c_uint = 524288;
pub const VPX_CODEC_CAP_INPUT_FRAGMENTS: ::std::os::raw::c_uint = 1048576;
pub const VPX_CODEC_CAP_FRAME_THREADING: ::std::os::raw::c_uint = 2097152;
pub const VPX_CODEC_CAP_EXTERNAL_FRAME_BUFFER: ::std::os::raw::c_uint =
    4194304;
pub const VPX_CODEC_USE_POSTPROC: ::std::os::raw::c_uint = 65536;
pub const VPX_CODEC_USE_ERROR_CONCEALMENT: ::std::os::raw::c_uint = 131072;
pub const VPX_CODEC_USE_INPUT_FRAGMENTS: ::std::os::raw::c_uint = 262144;
pub const VPX_CODEC_USE_FRAME_THREADING: ::std::os::raw::c_uint = 524288;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_img_fmt {
    VPX_IMG_FMT_NONE = 0,
    VPX_IMG_FMT_RGB24 = 1,
    VPX_IMG_FMT_RGB32 = 2,
    VPX_IMG_FMT_RGB565 = 3,
    VPX_IMG_FMT_RGB555 = 4,
    VPX_IMG_FMT_UYVY = 5,
    VPX_IMG_FMT_YUY2 = 6,
    VPX_IMG_FMT_YVYU = 7,
    VPX_IMG_FMT_BGR24 = 8,
    VPX_IMG_FMT_RGB32_LE = 9,
    VPX_IMG_FMT_ARGB = 10,
    VPX_IMG_FMT_ARGB_LE = 11,
    VPX_IMG_FMT_RGB565_LE = 12,
    VPX_IMG_FMT_RGB555_LE = 13,
    VPX_IMG_FMT_YV12 = 769,
    VPX_IMG_FMT_I420 = 258,
    VPX_IMG_FMT_VPXYV12 = 771,
    VPX_IMG_FMT_VPXI420 = 260,
    VPX_IMG_FMT_I422 = 261,
    VPX_IMG_FMT_I444 = 262,
    VPX_IMG_FMT_I440 = 263,
    VPX_IMG_FMT_444A = 1286,
    VPX_IMG_FMT_I42016 = 2306,
    VPX_IMG_FMT_I42216 = 2309,
    VPX_IMG_FMT_I44416 = 2310,
    VPX_IMG_FMT_I44016 = 2311,
}
pub use self::vpx_img_fmt as vpx_img_fmt_t;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_color_space {
    VPX_CS_UNKNOWN = 0,
    VPX_CS_BT_601 = 1,
    VPX_CS_BT_709 = 2,
    VPX_CS_SMPTE_170 = 3,
    VPX_CS_SMPTE_240 = 4,
    VPX_CS_BT_2020 = 5,
    VPX_CS_RESERVED = 6,
    VPX_CS_SRGB = 7,
}
pub use self::vpx_color_space as vpx_color_space_t;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_image {
        pub fmt: vpx_img_fmt_t,
        pub cs: vpx_color_space_t,
        pub w: ::std::os::raw::c_uint,
        pub h: ::std::os::raw::c_uint,
        pub bit_depth: ::std::os::raw::c_uint,
        pub d_w: ::std::os::raw::c_uint,
        pub d_h: ::std::os::raw::c_uint,
        pub x_chroma_shift: ::std::os::raw::c_uint,
        pub y_chroma_shift: ::std::os::raw::c_uint,
        pub planes: [*mut ::std::os::raw::c_uchar; 4usize],
        pub stride: [::std::os::raw::c_int; 4usize],
        pub bps: ::std::os::raw::c_int,
        pub user_priv: *mut ::std::os::raw::c_void,
        pub img_data: *mut ::std::os::raw::c_uchar,
        pub img_data_owner: ::std::os::raw::c_int,
        pub self_allocd: ::std::os::raw::c_int,
        pub fb_priv: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_vpx_image() {
    assert_eq!(::std::mem::size_of::<vpx_image>() , 128usize , concat ! (
               "Size of: " , stringify ! ( vpx_image ) ));
    assert_eq! (::std::mem::align_of::<vpx_image>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( vpx_image ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . fmt as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( fmt ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . cs as * const _ as usize
                } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( cs ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . w as * const _ as usize }
                , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( w ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . h as * const _ as usize }
                , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( h ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . bit_depth as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( bit_depth ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . d_w as * const _ as usize
                } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( d_w ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . d_h as * const _ as usize
                } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( d_h ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . x_chroma_shift as * const
                _ as usize } , 28usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( x_chroma_shift ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . y_chroma_shift as * const
                _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( y_chroma_shift ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . planes as * const _ as
                usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( planes ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . stride as * const _ as
                usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( stride ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . bps as * const _ as usize
                } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( bps ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . user_priv as * const _ as
                usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( user_priv ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . img_data as * const _ as
                usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( img_data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . img_data_owner as * const
                _ as usize } , 112usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( img_data_owner ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . self_allocd as * const _
                as usize } , 116usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( self_allocd ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image ) ) . fb_priv as * const _ as
                usize } , 120usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image ) , "::" ,
                stringify ! ( fb_priv ) ));
}
impl Clone for vpx_image {
    fn clone(&self) -> Self { *self }
}
pub type vpx_image_t = vpx_image;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_image_rect {
        pub x: ::std::os::raw::c_uint,
        pub y: ::std::os::raw::c_uint,
        pub w: ::std::os::raw::c_uint,
        pub h: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_vpx_image_rect() {
    assert_eq!(::std::mem::size_of::<vpx_image_rect>() , 16usize , concat ! (
               "Size of: " , stringify ! ( vpx_image_rect ) ));
    assert_eq! (::std::mem::align_of::<vpx_image_rect>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( vpx_image_rect ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image_rect ) ) . x as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image_rect ) , "::"
                , stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image_rect ) ) . y as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image_rect ) , "::"
                , stringify ! ( y ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image_rect ) ) . w as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image_rect ) , "::"
                , stringify ! ( w ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_image_rect ) ) . h as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_image_rect ) , "::"
                , stringify ! ( h ) ));
}
impl Clone for vpx_image_rect {
    fn clone(&self) -> Self { *self }
}
pub type vpx_image_rect_t = vpx_image_rect;
extern "C" {
    pub fn vpx_img_alloc(img: *mut vpx_image_t, fmt: vpx_img_fmt_t,
                         d_w: ::std::os::raw::c_uint,
                         d_h: ::std::os::raw::c_uint,
                         align: ::std::os::raw::c_uint) -> *mut vpx_image_t;
}
extern "C" {
    pub fn vpx_img_wrap(img: *mut vpx_image_t, fmt: vpx_img_fmt_t,
                        d_w: ::std::os::raw::c_uint,
                        d_h: ::std::os::raw::c_uint,
                        align: ::std::os::raw::c_uint,
                        img_data: *mut ::std::os::raw::c_uchar)
     -> *mut vpx_image_t;
}
extern "C" {
    pub fn vpx_img_set_rect(img: *mut vpx_image_t, x: ::std::os::raw::c_uint,
                            y: ::std::os::raw::c_uint,
                            w: ::std::os::raw::c_uint,
                            h: ::std::os::raw::c_uint)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vpx_img_flip(img: *mut vpx_image_t);
}
extern "C" {
    pub fn vpx_img_free(img: *mut vpx_image_t);
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_codec_err_t {
    VPX_CODEC_OK = 0,
    VPX_CODEC_ERROR = 1,
    VPX_CODEC_MEM_ERROR = 2,
    VPX_CODEC_ABI_MISMATCH = 3,
    VPX_CODEC_INCAPABLE = 4,
    VPX_CODEC_UNSUP_BITSTREAM = 5,
    VPX_CODEC_UNSUP_FEATURE = 6,
    VPX_CODEC_CORRUPT_FRAME = 7,
    VPX_CODEC_INVALID_PARAM = 8,
    VPX_CODEC_LIST_END = 9,
}
pub type vpx_codec_caps_t = ::std::os::raw::c_long;
pub type vpx_codec_flags_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_iface {
    _unused: [u8; 0],
}
pub type vpx_codec_iface_t = vpx_codec_iface;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vpx_codec_priv {
    _unused: [u8; 0],
}
pub type vpx_codec_priv_t = vpx_codec_priv;
pub type vpx_codec_iter_t = *const ::std::os::raw::c_void;
#[repr(C)]
#[derive(Copy)]
pub struct vpx_codec_ctx {
        pub name: *const ::std::os::raw::c_char,
        pub iface: *mut vpx_codec_iface_t,
        pub err: vpx_codec_err_t,
        pub err_detail: *const ::std::os::raw::c_char,
        pub init_flags: vpx_codec_flags_t,
        pub config: vpx_codec_ctx__bindgen_ty_1,
        pub priv_: *mut vpx_codec_priv_t,
}
#[repr(C)]
#[derive(Copy)]
pub union vpx_codec_ctx__bindgen_ty_1 {
    pub dec: *const vpx_codec_dec_cfg,
    pub enc: *const vpx_codec_enc_cfg,
    pub raw: *const ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_vpx_codec_ctx__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<vpx_codec_ctx__bindgen_ty_1>() , 8usize ,
               concat ! (
               "Size of: " , stringify ! ( vpx_codec_ctx__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<vpx_codec_ctx__bindgen_ty_1>() , 8usize
                , concat ! (
                "Alignment of " , stringify ! ( vpx_codec_ctx__bindgen_ty_1 )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_ctx__bindgen_ty_1 ) ) . dec as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_ctx__bindgen_ty_1 ) , "::" , stringify ! ( dec ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_ctx__bindgen_ty_1 ) ) . enc as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_ctx__bindgen_ty_1 ) , "::" , stringify ! ( enc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_ctx__bindgen_ty_1 ) ) . raw as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_ctx__bindgen_ty_1 ) , "::" , stringify ! ( raw ) ));
}
impl Clone for vpx_codec_ctx__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_vpx_codec_ctx() {
    assert_eq!(::std::mem::size_of::<vpx_codec_ctx>() , 56usize , concat ! (
               "Size of: " , stringify ! ( vpx_codec_ctx ) ));
    assert_eq! (::std::mem::align_of::<vpx_codec_ctx>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( vpx_codec_ctx ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_ctx ) ) . name as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_ctx ) , "::"
                , stringify ! ( name ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_ctx ) ) . iface as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_ctx ) , "::"
                , stringify ! ( iface ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_ctx ) ) . err as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_ctx ) , "::"
                , stringify ! ( err ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_ctx ) ) . err_detail as * const
                _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_ctx ) , "::"
                , stringify ! ( err_detail ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_ctx ) ) . init_flags as * const
                _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_ctx ) , "::"
                , stringify ! ( init_flags ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_ctx ) ) . config as * const _
                as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_ctx ) , "::"
                , stringify ! ( config ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_ctx ) ) . priv_ as * const _ as
                usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_ctx ) , "::"
                , stringify ! ( priv_ ) ));
}
impl Clone for vpx_codec_ctx {
    fn clone(&self) -> Self { *self }
}
pub type vpx_codec_ctx_t = vpx_codec_ctx;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_bit_depth { VPX_BITS_8 = 8, VPX_BITS_10 = 10, VPX_BITS_12 = 12, }
pub use self::vpx_bit_depth as vpx_bit_depth_t;
extern "C" {
    pub fn vpx_codec_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vpx_codec_version_str() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_version_extra_str() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_build_config() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_iface_name(iface: *mut vpx_codec_iface_t)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_err_to_string(err: vpx_codec_err_t)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_error(ctx: *mut vpx_codec_ctx_t)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_error_detail(ctx: *mut vpx_codec_ctx_t)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vpx_codec_destroy(ctx: *mut vpx_codec_ctx_t) -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_get_caps(iface: *mut vpx_codec_iface_t)
     -> vpx_codec_caps_t;
}
extern "C" {
    pub fn vpx_codec_control_(ctx: *mut vpx_codec_ctx_t,
                              ctrl_id: ::std::os::raw::c_int, ...)
     -> vpx_codec_err_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8_com_control_id {
    VP8_SET_REFERENCE = 1,
    VP8_COPY_REFERENCE = 2,
    VP8_SET_POSTPROC = 3,
    VP8_SET_DBG_COLOR_REF_FRAME = 4,
    VP8_SET_DBG_COLOR_MB_MODES = 5,
    VP8_SET_DBG_COLOR_B_MODES = 6,
    VP8_SET_DBG_DISPLAY_MV = 7,
    VP9_GET_REFERENCE = 128,
    VP8_COMMON_CTRL_ID_MAX = 129,
    VP8_DECODER_CTRL_ID_START = 256,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8_postproc_level {
    VP8_NOFILTERING = 0,
    VP8_DEBLOCK = 1,
    VP8_DEMACROBLOCK = 2,
    VP8_ADDNOISE = 4,
    VP8_DEBUG_TXT_FRAME_INFO = 8,
    VP8_DEBUG_TXT_MBLK_MODES = 16,
    VP8_DEBUG_TXT_DC_DIFF = 32,
    VP8_DEBUG_TXT_RATE_INFO = 64,
    VP8_MFQE = 1024,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vp8_postproc_cfg {
        pub post_proc_flag: ::std::os::raw::c_int,
        pub deblocking_level: ::std::os::raw::c_int,
        pub noise_level: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_vp8_postproc_cfg() {
    assert_eq!(::std::mem::size_of::<vp8_postproc_cfg>() , 12usize , concat !
               ( "Size of: " , stringify ! ( vp8_postproc_cfg ) ));
    assert_eq! (::std::mem::align_of::<vp8_postproc_cfg>() , 4usize , concat !
                ( "Alignment of " , stringify ! ( vp8_postproc_cfg ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vp8_postproc_cfg ) ) . post_proc_flag as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vp8_postproc_cfg ) ,
                "::" , stringify ! ( post_proc_flag ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vp8_postproc_cfg ) ) . deblocking_level
                as * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( vp8_postproc_cfg ) ,
                "::" , stringify ! ( deblocking_level ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vp8_postproc_cfg ) ) . noise_level as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vp8_postproc_cfg ) ,
                "::" , stringify ! ( noise_level ) ));
}
impl Clone for vp8_postproc_cfg {
    fn clone(&self) -> Self { *self }
}
pub type vp8_postproc_cfg_t = vp8_postproc_cfg;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_ref_frame_type {
    VP8_LAST_FRAME = 1,
    VP8_GOLD_FRAME = 2,
    VP8_ALTR_FRAME = 4,
}
pub use self::vpx_ref_frame_type as vpx_ref_frame_type_t;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_ref_frame {
        pub frame_type: vpx_ref_frame_type_t,
        pub img: vpx_image_t,
}
#[test]
fn bindgen_test_layout_vpx_ref_frame() {
    assert_eq!(::std::mem::size_of::<vpx_ref_frame>() , 136usize , concat ! (
               "Size of: " , stringify ! ( vpx_ref_frame ) ));
    assert_eq! (::std::mem::align_of::<vpx_ref_frame>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( vpx_ref_frame ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_ref_frame ) ) . frame_type as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_ref_frame ) , "::"
                , stringify ! ( frame_type ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_ref_frame ) ) . img as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_ref_frame ) , "::"
                , stringify ! ( img ) ));
}
impl Clone for vpx_ref_frame {
    fn clone(&self) -> Self { *self }
}
pub type vpx_ref_frame_t = vpx_ref_frame;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vp9_ref_frame {
        pub idx: ::std::os::raw::c_int,
        pub img: vpx_image_t,
}
#[test]
fn bindgen_test_layout_vp9_ref_frame() {
    assert_eq!(::std::mem::size_of::<vp9_ref_frame>() , 136usize , concat ! (
               "Size of: " , stringify ! ( vp9_ref_frame ) ));
    assert_eq! (::std::mem::align_of::<vp9_ref_frame>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( vp9_ref_frame ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vp9_ref_frame ) ) . idx as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vp9_ref_frame ) , "::"
                , stringify ! ( idx ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vp9_ref_frame ) ) . img as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vp9_ref_frame ) , "::"
                , stringify ! ( img ) ));
}
impl Clone for vp9_ref_frame {
    fn clone(&self) -> Self { *self }
}
pub type vp9_ref_frame_t = vp9_ref_frame;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_fixed_buf {
        pub buf: *mut ::std::os::raw::c_void,
        pub sz: usize,
}
#[test]
fn bindgen_test_layout_vpx_fixed_buf() {
    assert_eq!(::std::mem::size_of::<vpx_fixed_buf>() , 16usize , concat ! (
               "Size of: " , stringify ! ( vpx_fixed_buf ) ));
    assert_eq! (::std::mem::align_of::<vpx_fixed_buf>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( vpx_fixed_buf ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_fixed_buf ) ) . buf as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_fixed_buf ) , "::"
                , stringify ! ( buf ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_fixed_buf ) ) . sz as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_fixed_buf ) , "::"
                , stringify ! ( sz ) ));
}
impl Clone for vpx_fixed_buf {
    fn clone(&self) -> Self { *self }
}
pub type vpx_fixed_buf_t = vpx_fixed_buf;
pub type vpx_codec_pts_t = i64;
pub type vpx_codec_frame_flags_t = u32;
pub type vpx_codec_er_flags_t = u32;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_codec_cx_pkt_kind {
    VPX_CODEC_CX_FRAME_PKT = 0,
    VPX_CODEC_STATS_PKT = 1,
    VPX_CODEC_FPMB_STATS_PKT = 2,
    VPX_CODEC_PSNR_PKT = 3,
    VPX_CODEC_CUSTOM_PKT = 256,
}
#[repr(C)]
pub struct vpx_codec_cx_pkt {
        pub kind: vpx_codec_cx_pkt_kind,
        pub data: vpx_codec_cx_pkt__bindgen_ty_1,
}
#[repr(C)]
pub union vpx_codec_cx_pkt__bindgen_ty_1 {
        pub frame: vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1,
        pub twopass_stats: vpx_fixed_buf_t,
        pub firstpass_mb_stats: vpx_fixed_buf_t,
        pub psnr: vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt,
        pub raw: vpx_fixed_buf_t,
        pub pad: [::std::os::raw::c_char; 124usize],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 {
        pub buf: *mut ::std::os::raw::c_void,
        pub sz: usize,
        pub pts: vpx_codec_pts_t,
        pub duration: ::std::os::raw::c_ulong,
        pub flags: vpx_codec_frame_flags_t,
        pub partition_id: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1>()
               , 40usize , concat ! (
               "Size of: " , stringify ! (
               vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 ) ));
    assert_eq! (::std::mem::align_of::<vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1>()
                , 8usize , concat ! (
                "Alignment of " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1
                ) ) . buf as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( buf ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1
                ) ) . sz as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( sz ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1
                ) ) . pts as * const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( pts ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1
                ) ) . duration as * const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( duration ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1
                ) ) . flags as * const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( flags ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1
                ) ) . partition_id as * const _ as usize } , 36usize , concat
                ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 ) , "::" ,
                stringify ! ( partition_id ) ));
}
impl Clone for vpx_codec_cx_pkt__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt {
        pub samples: [::std::os::raw::c_uint; 4usize],
        pub sse: [u64; 4usize],
        pub psnr: [f64; 4usize],
}
#[test]
fn bindgen_test_layout_vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt() {
    assert_eq!(::std::mem::size_of::<vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt>()
               , 80usize , concat ! (
               "Size of: " , stringify ! (
               vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt ) ));
    assert_eq! (::std::mem::align_of::<vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt>()
                , 8usize , concat ! (
                "Alignment of " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt )
                ) . samples as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt ) , "::" ,
                stringify ! ( samples ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt )
                ) . sse as * const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt ) , "::" ,
                stringify ! ( sse ) ));
    assert_eq! (unsafe {
                & (
                * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt )
                ) . psnr as * const _ as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt ) , "::" ,
                stringify ! ( psnr ) ));
}
impl Clone for vpx_codec_cx_pkt__bindgen_ty_1_vpx_psnr_pkt {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_vpx_codec_cx_pkt__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<vpx_codec_cx_pkt__bindgen_ty_1>() ,
               128usize , concat ! (
               "Size of: " , stringify ! ( vpx_codec_cx_pkt__bindgen_ty_1 )
               ));
    assert_eq! (::std::mem::align_of::<vpx_codec_cx_pkt__bindgen_ty_1>() ,
                8usize , concat ! (
                "Alignment of " , stringify ! ( vpx_codec_cx_pkt__bindgen_ty_1
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1 ) ) .
                frame as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1 ) , "::" , stringify ! ( frame
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1 ) ) .
                twopass_stats as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1 ) , "::" , stringify ! (
                twopass_stats ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1 ) ) .
                firstpass_mb_stats as * const _ as usize } , 0usize , concat !
                (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1 ) , "::" , stringify ! (
                firstpass_mb_stats ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1 ) ) . psnr
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1 ) , "::" , stringify ! ( psnr )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1 ) ) . raw
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1 ) , "::" , stringify ! ( raw )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_cx_pkt__bindgen_ty_1 ) ) . pad
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_cx_pkt__bindgen_ty_1 ) , "::" , stringify ! ( pad )
                ));
}
#[test]
fn bindgen_test_layout_vpx_codec_cx_pkt() {
    assert_eq!(::std::mem::size_of::<vpx_codec_cx_pkt>() , 136usize , concat !
               ( "Size of: " , stringify ! ( vpx_codec_cx_pkt ) ));
    assert_eq! (::std::mem::align_of::<vpx_codec_cx_pkt>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( vpx_codec_cx_pkt ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_cx_pkt ) ) . kind as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_cx_pkt ) ,
                "::" , stringify ! ( kind ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_cx_pkt ) ) . data as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_cx_pkt ) ,
                "::" , stringify ! ( data ) ));
}
pub type vpx_codec_cx_pkt_t = vpx_codec_cx_pkt;
pub type vpx_codec_enc_output_cx_pkt_cb_fn_t =
    ::std::option::Option<unsafe extern "C" fn(pkt: *mut vpx_codec_cx_pkt_t,
                                               user_data:
                                                   *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_codec_enc_output_cx_cb_pair {
        pub output_cx_pkt: vpx_codec_enc_output_cx_pkt_cb_fn_t,
        pub user_priv: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_vpx_codec_enc_output_cx_cb_pair() {
    assert_eq!(::std::mem::size_of::<vpx_codec_enc_output_cx_cb_pair>() ,
               16usize , concat ! (
               "Size of: " , stringify ! ( vpx_codec_enc_output_cx_cb_pair )
               ));
    assert_eq! (::std::mem::align_of::<vpx_codec_enc_output_cx_cb_pair>() ,
                8usize , concat ! (
                "Alignment of " , stringify ! (
                vpx_codec_enc_output_cx_cb_pair ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_output_cx_cb_pair ) ) .
                output_cx_pkt as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_enc_output_cx_cb_pair ) , "::" , stringify ! (
                output_cx_pkt ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_output_cx_cb_pair ) ) .
                user_priv as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                vpx_codec_enc_output_cx_cb_pair ) , "::" , stringify ! (
                user_priv ) ));
}
impl Clone for vpx_codec_enc_output_cx_cb_pair {
    fn clone(&self) -> Self { *self }
}
pub type vpx_codec_priv_output_cx_pkt_cb_pair_t =
    vpx_codec_enc_output_cx_cb_pair;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_rational {
        pub num: ::std::os::raw::c_int,
        pub den: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_vpx_rational() {
    assert_eq!(::std::mem::size_of::<vpx_rational>() , 8usize , concat ! (
               "Size of: " , stringify ! ( vpx_rational ) ));
    assert_eq! (::std::mem::align_of::<vpx_rational>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( vpx_rational ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_rational ) ) . num as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_rational ) , "::" ,
                stringify ! ( num ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_rational ) ) . den as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_rational ) , "::" ,
                stringify ! ( den ) ));
}
impl Clone for vpx_rational {
    fn clone(&self) -> Self { *self }
}
pub type vpx_rational_t = vpx_rational;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_enc_pass {
    VPX_RC_ONE_PASS = 0,
    VPX_RC_FIRST_PASS = 1,
    VPX_RC_LAST_PASS = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_rc_mode { VPX_VBR = 0, VPX_CBR = 1, VPX_CQ = 2, VPX_Q = 3, }
pub const vpx_kf_mode_VPX_KF_DISABLED: vpx_kf_mode =
    vpx_kf_mode::VPX_KF_FIXED;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_kf_mode { VPX_KF_FIXED = 0, VPX_KF_AUTO = 1, }
pub type vpx_enc_frame_flags_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_codec_enc_cfg {
        pub g_usage: ::std::os::raw::c_uint,
        pub g_threads: ::std::os::raw::c_uint,
        pub g_profile: ::std::os::raw::c_uint,
        pub g_w: ::std::os::raw::c_uint,
        pub g_h: ::std::os::raw::c_uint,
        pub g_bit_depth: vpx_bit_depth_t,
        pub g_input_bit_depth: ::std::os::raw::c_uint,
        pub g_timebase: vpx_rational,
        pub g_error_resilient: vpx_codec_er_flags_t,
        pub g_pass: vpx_enc_pass,
        pub g_lag_in_frames: ::std::os::raw::c_uint,
        pub rc_dropframe_thresh: ::std::os::raw::c_uint,
        pub rc_resize_allowed: ::std::os::raw::c_uint,
        pub rc_scaled_width: ::std::os::raw::c_uint,
        pub rc_scaled_height: ::std::os::raw::c_uint,
        pub rc_resize_up_thresh: ::std::os::raw::c_uint,
        pub rc_resize_down_thresh: ::std::os::raw::c_uint,
        pub rc_end_usage: vpx_rc_mode,
        pub rc_twopass_stats_in: vpx_fixed_buf_t,
        pub rc_firstpass_mb_stats_in: vpx_fixed_buf_t,
        pub rc_target_bitrate: ::std::os::raw::c_uint,
        pub rc_min_quantizer: ::std::os::raw::c_uint,
        pub rc_max_quantizer: ::std::os::raw::c_uint,
        pub rc_undershoot_pct: ::std::os::raw::c_uint,
        pub rc_overshoot_pct: ::std::os::raw::c_uint,
        pub rc_buf_sz: ::std::os::raw::c_uint,
        pub rc_buf_initial_sz: ::std::os::raw::c_uint,
        pub rc_buf_optimal_sz: ::std::os::raw::c_uint,
        pub rc_2pass_vbr_bias_pct: ::std::os::raw::c_uint,
        pub rc_2pass_vbr_minsection_pct: ::std::os::raw::c_uint,
        pub rc_2pass_vbr_maxsection_pct: ::std::os::raw::c_uint,
        pub kf_mode: vpx_kf_mode,
        pub kf_min_dist: ::std::os::raw::c_uint,
        pub kf_max_dist: ::std::os::raw::c_uint,
        pub ss_number_layers: ::std::os::raw::c_uint,
        pub ss_enable_auto_alt_ref: [::std::os::raw::c_int; 5usize],
        pub ss_target_bitrate: [::std::os::raw::c_uint; 5usize],
        pub ts_number_layers: ::std::os::raw::c_uint,
        pub ts_target_bitrate: [::std::os::raw::c_uint; 5usize],
        pub ts_rate_decimator: [::std::os::raw::c_uint; 5usize],
        pub ts_periodicity: ::std::os::raw::c_uint,
        pub ts_layer_id: [::std::os::raw::c_uint; 16usize],
}
#[test]
fn bindgen_test_layout_vpx_codec_enc_cfg() {
    assert_eq!(::std::mem::size_of::<vpx_codec_enc_cfg>() , 328usize , concat
               ! ( "Size of: " , stringify ! ( vpx_codec_enc_cfg ) ));
    assert_eq! (::std::mem::align_of::<vpx_codec_enc_cfg>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( vpx_codec_enc_cfg ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_usage as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_usage ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_threads as *
                const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_threads ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_profile as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_profile ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_w as * const _
                as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_w ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_h as * const _
                as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_h ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_bit_depth as *
                const _ as usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_bit_depth ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_input_bit_depth
                as * const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_input_bit_depth ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_timebase as *
                const _ as usize } , 28usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_timebase ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_error_resilient
                as * const _ as usize } , 36usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_error_resilient ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_pass as * const
                _ as usize } , 40usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_pass ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . g_lag_in_frames
                as * const _ as usize } , 44usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( g_lag_in_frames ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) .
                rc_dropframe_thresh as * const _ as usize } , 48usize , concat
                ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_dropframe_thresh ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_resize_allowed
                as * const _ as usize } , 52usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_resize_allowed ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_scaled_width
                as * const _ as usize } , 56usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_scaled_width ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_scaled_height
                as * const _ as usize } , 60usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_scaled_height ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) .
                rc_resize_up_thresh as * const _ as usize } , 64usize , concat
                ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_resize_up_thresh ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) .
                rc_resize_down_thresh as * const _ as usize } , 68usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_resize_down_thresh ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_end_usage as *
                const _ as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_end_usage ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) .
                rc_twopass_stats_in as * const _ as usize } , 80usize , concat
                ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_twopass_stats_in ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) .
                rc_firstpass_mb_stats_in as * const _ as usize } , 96usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_firstpass_mb_stats_in ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_target_bitrate
                as * const _ as usize } , 112usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_target_bitrate ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_min_quantizer
                as * const _ as usize } , 116usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_min_quantizer ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_max_quantizer
                as * const _ as usize } , 120usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_max_quantizer ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_undershoot_pct
                as * const _ as usize } , 124usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_undershoot_pct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_overshoot_pct
                as * const _ as usize } , 128usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_overshoot_pct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_buf_sz as *
                const _ as usize } , 132usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_buf_sz ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_buf_initial_sz
                as * const _ as usize } , 136usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_buf_initial_sz ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . rc_buf_optimal_sz
                as * const _ as usize } , 140usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_buf_optimal_sz ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) .
                rc_2pass_vbr_bias_pct as * const _ as usize } , 144usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_2pass_vbr_bias_pct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) .
                rc_2pass_vbr_minsection_pct as * const _ as usize } , 148usize
                , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_2pass_vbr_minsection_pct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) .
                rc_2pass_vbr_maxsection_pct as * const _ as usize } , 152usize
                , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( rc_2pass_vbr_maxsection_pct ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . kf_mode as *
                const _ as usize } , 156usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( kf_mode ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . kf_min_dist as *
                const _ as usize } , 160usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( kf_min_dist ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . kf_max_dist as *
                const _ as usize } , 164usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( kf_max_dist ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . ss_number_layers
                as * const _ as usize } , 168usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( ss_number_layers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) .
                ss_enable_auto_alt_ref as * const _ as usize } , 172usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( ss_enable_auto_alt_ref ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . ss_target_bitrate
                as * const _ as usize } , 192usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( ss_target_bitrate ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . ts_number_layers
                as * const _ as usize } , 212usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( ts_number_layers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . ts_target_bitrate
                as * const _ as usize } , 216usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( ts_target_bitrate ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . ts_rate_decimator
                as * const _ as usize } , 236usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( ts_rate_decimator ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . ts_periodicity as
                * const _ as usize } , 256usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( ts_periodicity ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_enc_cfg ) ) . ts_layer_id as *
                const _ as usize } , 260usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_enc_cfg ) ,
                "::" , stringify ! ( ts_layer_id ) ));
}
impl Clone for vpx_codec_enc_cfg {
    fn clone(&self) -> Self { *self }
}
pub type vpx_codec_enc_cfg_t = vpx_codec_enc_cfg;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_svc_parameters {
        pub max_quantizers: [::std::os::raw::c_int; 5usize],
        pub min_quantizers: [::std::os::raw::c_int; 5usize],
        pub scaling_factor_num: [::std::os::raw::c_int; 5usize],
        pub scaling_factor_den: [::std::os::raw::c_int; 5usize],
}
#[test]
fn bindgen_test_layout_vpx_svc_parameters() {
    assert_eq!(::std::mem::size_of::<vpx_svc_parameters>() , 80usize , concat
               ! ( "Size of: " , stringify ! ( vpx_svc_parameters ) ));
    assert_eq! (::std::mem::align_of::<vpx_svc_parameters>() , 4usize , concat
                ! ( "Alignment of " , stringify ! ( vpx_svc_parameters ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_svc_parameters ) ) . max_quantizers
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_svc_parameters ) ,
                "::" , stringify ! ( max_quantizers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_svc_parameters ) ) . min_quantizers
                as * const _ as usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_svc_parameters ) ,
                "::" , stringify ! ( min_quantizers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_svc_parameters ) ) .
                scaling_factor_num as * const _ as usize } , 40usize , concat
                ! (
                "Alignment of field: " , stringify ! ( vpx_svc_parameters ) ,
                "::" , stringify ! ( scaling_factor_num ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_svc_parameters ) ) .
                scaling_factor_den as * const _ as usize } , 60usize , concat
                ! (
                "Alignment of field: " , stringify ! ( vpx_svc_parameters ) ,
                "::" , stringify ! ( scaling_factor_den ) ));
}
impl Clone for vpx_svc_parameters {
    fn clone(&self) -> Self { *self }
}
pub type vpx_svc_extra_cfg_t = vpx_svc_parameters;
extern "C" {
    pub fn vpx_codec_enc_init_ver(ctx: *mut vpx_codec_ctx_t,
                                  iface: *mut vpx_codec_iface_t,
                                  cfg: *const vpx_codec_enc_cfg_t,
                                  flags: vpx_codec_flags_t,
                                  ver: ::std::os::raw::c_int)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_enc_init_multi_ver(ctx: *mut vpx_codec_ctx_t,
                                        iface: *mut vpx_codec_iface_t,
                                        cfg: *mut vpx_codec_enc_cfg_t,
                                        num_enc: ::std::os::raw::c_int,
                                        flags: vpx_codec_flags_t,
                                        dsf: *mut vpx_rational_t,
                                        ver: ::std::os::raw::c_int)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_enc_config_default(iface: *mut vpx_codec_iface_t,
                                        cfg: *mut vpx_codec_enc_cfg_t,
                                        reserved: ::std::os::raw::c_uint)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_enc_config_set(ctx: *mut vpx_codec_ctx_t,
                                    cfg: *const vpx_codec_enc_cfg_t)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_get_global_headers(ctx: *mut vpx_codec_ctx_t)
     -> *mut vpx_fixed_buf_t;
}
extern "C" {
    pub fn vpx_codec_encode(ctx: *mut vpx_codec_ctx_t,
                            img: *const vpx_image_t, pts: vpx_codec_pts_t,
                            duration: ::std::os::raw::c_ulong,
                            flags: vpx_enc_frame_flags_t,
                            deadline: ::std::os::raw::c_ulong)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_set_cx_data_buf(ctx: *mut vpx_codec_ctx_t,
                                     buf: *const vpx_fixed_buf_t,
                                     pad_before: ::std::os::raw::c_uint,
                                     pad_after: ::std::os::raw::c_uint)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_get_cx_data(ctx: *mut vpx_codec_ctx_t,
                                 iter: *mut vpx_codec_iter_t)
     -> *const vpx_codec_cx_pkt_t;
}
extern "C" {
    pub fn vpx_codec_get_preview_frame(ctx: *mut vpx_codec_ctx_t)
     -> *const vpx_image_t;
}
extern "C" {
    pub static mut vpx_codec_vp8_cx_algo: vpx_codec_iface_t;
}
extern "C" {
    pub fn vpx_codec_vp8_cx() -> *mut vpx_codec_iface_t;
}
extern "C" {
    pub static mut vpx_codec_vp9_cx_algo: vpx_codec_iface_t;
}
extern "C" {
    pub fn vpx_codec_vp9_cx() -> *mut vpx_codec_iface_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8e_enc_control_id {
    VP8E_UPD_ENTROPY = 5,
    VP8E_UPD_REFERENCE = 6,
    VP8E_USE_REFERENCE = 7,
    VP8E_SET_ROI_MAP = 8,
    VP8E_SET_ACTIVEMAP = 9,
    VP8E_SET_SCALEMODE = 11,
    VP8E_SET_CPUUSED = 13,
    VP8E_SET_ENABLEAUTOALTREF = 14,
    VP8E_SET_NOISE_SENSITIVITY = 15,
    VP8E_SET_SHARPNESS = 16,
    VP8E_SET_STATIC_THRESHOLD = 17,
    VP8E_SET_TOKEN_PARTITIONS = 18,
    VP8E_GET_LAST_QUANTIZER = 19,
    VP8E_GET_LAST_QUANTIZER_64 = 20,
    VP8E_SET_ARNR_MAXFRAMES = 21,
    VP8E_SET_ARNR_STRENGTH = 22,
    VP8E_SET_ARNR_TYPE = 23,
    VP8E_SET_TUNING = 24,
    VP8E_SET_CQ_LEVEL = 25,
    VP8E_SET_MAX_INTRA_BITRATE_PCT = 26,
    VP8E_SET_FRAME_FLAGS = 27,
    VP9E_SET_MAX_INTER_BITRATE_PCT = 28,
    VP9E_SET_GF_CBR_BOOST_PCT = 29,
    VP8E_SET_TEMPORAL_LAYER_ID = 30,
    VP8E_SET_SCREEN_CONTENT_MODE = 31,
    VP9E_SET_LOSSLESS = 32,
    VP9E_SET_TILE_COLUMNS = 33,
    VP9E_SET_TILE_ROWS = 34,
    VP9E_SET_FRAME_PARALLEL_DECODING = 35,
    VP9E_SET_AQ_MODE = 36,
    VP9E_SET_FRAME_PERIODIC_BOOST = 37,
    VP9E_SET_NOISE_SENSITIVITY = 38,
    VP9E_SET_SVC = 39,
    VP9E_SET_SVC_LAYER_ID = 40,
    VP9E_SET_TUNE_CONTENT = 41,
    VP9E_SET_COLOR_SPACE = 42,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vpx_scaling_mode_1d {
    VP8E_NORMAL = 0,
    VP8E_FOURFIVE = 1,
    VP8E_THREEFIVE = 2,
    VP8E_ONETWO = 3,
}
pub use self::vpx_scaling_mode_1d as VPX_SCALING_MODE;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_roi_map {
        pub roi_map: *mut ::std::os::raw::c_uchar,
        pub rows: ::std::os::raw::c_uint,
        pub cols: ::std::os::raw::c_uint,
        pub delta_q: [::std::os::raw::c_int; 4usize],
        pub delta_lf: [::std::os::raw::c_int; 4usize],
        pub static_threshold: [::std::os::raw::c_uint; 4usize],
}
#[test]
fn bindgen_test_layout_vpx_roi_map() {
    assert_eq!(::std::mem::size_of::<vpx_roi_map>() , 64usize , concat ! (
               "Size of: " , stringify ! ( vpx_roi_map ) ));
    assert_eq! (::std::mem::align_of::<vpx_roi_map>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( vpx_roi_map ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_roi_map ) ) . roi_map as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_roi_map ) , "::" ,
                stringify ! ( roi_map ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_roi_map ) ) . rows as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_roi_map ) , "::" ,
                stringify ! ( rows ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_roi_map ) ) . cols as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_roi_map ) , "::" ,
                stringify ! ( cols ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_roi_map ) ) . delta_q as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_roi_map ) , "::" ,
                stringify ! ( delta_q ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_roi_map ) ) . delta_lf as * const _
                as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_roi_map ) , "::" ,
                stringify ! ( delta_lf ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_roi_map ) ) . static_threshold as *
                const _ as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_roi_map ) , "::" ,
                stringify ! ( static_threshold ) ));
}
impl Clone for vpx_roi_map {
    fn clone(&self) -> Self { *self }
}
pub type vpx_roi_map_t = vpx_roi_map;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_active_map {
        pub active_map: *mut ::std::os::raw::c_uchar,
        pub rows: ::std::os::raw::c_uint,
        pub cols: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_vpx_active_map() {
    assert_eq!(::std::mem::size_of::<vpx_active_map>() , 16usize , concat ! (
               "Size of: " , stringify ! ( vpx_active_map ) ));
    assert_eq! (::std::mem::align_of::<vpx_active_map>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( vpx_active_map ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_active_map ) ) . active_map as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_active_map ) , "::"
                , stringify ! ( active_map ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_active_map ) ) . rows as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_active_map ) , "::"
                , stringify ! ( rows ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_active_map ) ) . cols as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_active_map ) , "::"
                , stringify ! ( cols ) ));
}
impl Clone for vpx_active_map {
    fn clone(&self) -> Self { *self }
}
pub type vpx_active_map_t = vpx_active_map;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_scaling_mode {
        pub h_scaling_mode: VPX_SCALING_MODE,
        pub v_scaling_mode: VPX_SCALING_MODE,
}
#[test]
fn bindgen_test_layout_vpx_scaling_mode() {
    assert_eq!(::std::mem::size_of::<vpx_scaling_mode>() , 8usize , concat ! (
               "Size of: " , stringify ! ( vpx_scaling_mode ) ));
    assert_eq! (::std::mem::align_of::<vpx_scaling_mode>() , 4usize , concat !
                ( "Alignment of " , stringify ! ( vpx_scaling_mode ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_scaling_mode ) ) . h_scaling_mode as
                * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_scaling_mode ) ,
                "::" , stringify ! ( h_scaling_mode ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_scaling_mode ) ) . v_scaling_mode as
                * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_scaling_mode ) ,
                "::" , stringify ! ( v_scaling_mode ) ));
}
impl Clone for vpx_scaling_mode {
    fn clone(&self) -> Self { *self }
}
pub type vpx_scaling_mode_t = vpx_scaling_mode;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8e_token_partitions {
    VP8_ONE_TOKENPARTITION = 0,
    VP8_TWO_TOKENPARTITION = 1,
    VP8_FOUR_TOKENPARTITION = 2,
    VP8_EIGHT_TOKENPARTITION = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp9e_tune_content {
    VP9E_CONTENT_DEFAULT = 0,
    VP9E_CONTENT_SCREEN = 1,
    VP9E_CONTENT_INVALID = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8e_tuning { VP8_TUNE_PSNR = 0, VP8_TUNE_SSIM = 1, }
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_svc_layer_id {
        pub temporal_layer_id: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_vpx_svc_layer_id() {
    assert_eq!(::std::mem::size_of::<vpx_svc_layer_id>() , 4usize , concat ! (
               "Size of: " , stringify ! ( vpx_svc_layer_id ) ));
    assert_eq! (::std::mem::align_of::<vpx_svc_layer_id>() , 4usize , concat !
                ( "Alignment of " , stringify ! ( vpx_svc_layer_id ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_svc_layer_id ) ) . temporal_layer_id
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_svc_layer_id ) ,
                "::" , stringify ! ( temporal_layer_id ) ));
}
impl Clone for vpx_svc_layer_id {
    fn clone(&self) -> Self { *self }
}
pub type vpx_svc_layer_id_t = vpx_svc_layer_id;
extern "C" {
    pub static mut vpx_codec_vp8_dx_algo: vpx_codec_iface_t;
}
extern "C" {
    pub fn vpx_codec_vp8_dx() -> *mut vpx_codec_iface_t;
}
extern "C" {
    pub static mut vpx_codec_vp9_dx_algo: vpx_codec_iface_t;
}
extern "C" {
    pub fn vpx_codec_vp9_dx() -> *mut vpx_codec_iface_t;
}
pub const vp8_dec_control_id_VP8D_SET_DECRYPTOR: vp8_dec_control_id =
    vp8_dec_control_id::VPXD_SET_DECRYPTOR;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum vp8_dec_control_id {
    VP8D_GET_LAST_REF_UPDATES = 256,
    VP8D_GET_FRAME_CORRUPTED = 257,
    VP8D_GET_LAST_REF_USED = 258,
    VPXD_SET_DECRYPTOR = 259,
    VP9D_GET_FRAME_SIZE = 260,
    VP9D_GET_DISPLAY_SIZE = 261,
    VP9D_GET_BIT_DEPTH = 262,
    VP9_SET_BYTE_ALIGNMENT = 263,
    VP9_INVERT_TILE_DECODE_ORDER = 264,
    VP8_DECODER_CTRL_ID_MAX = 265,
}
pub type vpx_decrypt_cb =
    ::std::option::Option<unsafe extern "C" fn(decrypt_state:
                                                   *mut ::std::os::raw::c_void,
                                               input:
                                                   *const ::std::os::raw::c_uchar,
                                               output:
                                                   *mut ::std::os::raw::c_uchar,
                                               count: ::std::os::raw::c_int)>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_decrypt_init {
        pub decrypt_cb: vpx_decrypt_cb,
        pub decrypt_state: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_vpx_decrypt_init() {
    assert_eq!(::std::mem::size_of::<vpx_decrypt_init>() , 16usize , concat !
               ( "Size of: " , stringify ! ( vpx_decrypt_init ) ));
    assert_eq! (::std::mem::align_of::<vpx_decrypt_init>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( vpx_decrypt_init ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_decrypt_init ) ) . decrypt_cb as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_decrypt_init ) ,
                "::" , stringify ! ( decrypt_cb ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_decrypt_init ) ) . decrypt_state as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_decrypt_init ) ,
                "::" , stringify ! ( decrypt_state ) ));
}
impl Clone for vpx_decrypt_init {
    fn clone(&self) -> Self { *self }
}
pub type vp8_decrypt_init = vpx_decrypt_init;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_codec_frame_buffer {
        pub data: *mut u8,
        pub size: usize,
        pub priv_: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_vpx_codec_frame_buffer() {
    assert_eq!(::std::mem::size_of::<vpx_codec_frame_buffer>() , 24usize ,
               concat ! ( "Size of: " , stringify ! ( vpx_codec_frame_buffer )
               ));
    assert_eq! (::std::mem::align_of::<vpx_codec_frame_buffer>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( vpx_codec_frame_buffer ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_frame_buffer ) ) . data as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_frame_buffer
                ) , "::" , stringify ! ( data ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_frame_buffer ) ) . size as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_frame_buffer
                ) , "::" , stringify ! ( size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_frame_buffer ) ) . priv_ as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_frame_buffer
                ) , "::" , stringify ! ( priv_ ) ));
}
impl Clone for vpx_codec_frame_buffer {
    fn clone(&self) -> Self { *self }
}
pub type vpx_codec_frame_buffer_t = vpx_codec_frame_buffer;
pub type vpx_get_frame_buffer_cb_fn_t =
    ::std::option::Option<unsafe extern "C" fn(priv_:
                                                   *mut ::std::os::raw::c_void,
                                               min_size: usize,
                                               fb:
                                                   *mut vpx_codec_frame_buffer_t)
                              -> ::std::os::raw::c_int>;
pub type vpx_release_frame_buffer_cb_fn_t =
    ::std::option::Option<unsafe extern "C" fn(priv_:
                                                   *mut ::std::os::raw::c_void,
                                               fb:
                                                   *mut vpx_codec_frame_buffer_t)
                              -> ::std::os::raw::c_int>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_codec_stream_info {
        pub sz: ::std::os::raw::c_uint,
        pub w: ::std::os::raw::c_uint,
        pub h: ::std::os::raw::c_uint,
        pub is_kf: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_vpx_codec_stream_info() {
    assert_eq!(::std::mem::size_of::<vpx_codec_stream_info>() , 16usize ,
               concat ! ( "Size of: " , stringify ! ( vpx_codec_stream_info )
               ));
    assert_eq! (::std::mem::align_of::<vpx_codec_stream_info>() , 4usize ,
                concat ! (
                "Alignment of " , stringify ! ( vpx_codec_stream_info ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_stream_info ) ) . sz as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_stream_info )
                , "::" , stringify ! ( sz ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_stream_info ) ) . w as * const
                _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_stream_info )
                , "::" , stringify ! ( w ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_stream_info ) ) . h as * const
                _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_stream_info )
                , "::" , stringify ! ( h ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_stream_info ) ) . is_kf as *
                const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_stream_info )
                , "::" , stringify ! ( is_kf ) ));
}
impl Clone for vpx_codec_stream_info {
    fn clone(&self) -> Self { *self }
}
pub type vpx_codec_stream_info_t = vpx_codec_stream_info;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct vpx_codec_dec_cfg {
        pub threads: ::std::os::raw::c_uint,
        pub w: ::std::os::raw::c_uint,
        pub h: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_vpx_codec_dec_cfg() {
    assert_eq!(::std::mem::size_of::<vpx_codec_dec_cfg>() , 12usize , concat !
               ( "Size of: " , stringify ! ( vpx_codec_dec_cfg ) ));
    assert_eq! (::std::mem::align_of::<vpx_codec_dec_cfg>() , 4usize , concat
                ! ( "Alignment of " , stringify ! ( vpx_codec_dec_cfg ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_dec_cfg ) ) . threads as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_dec_cfg ) ,
                "::" , stringify ! ( threads ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_dec_cfg ) ) . w as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_dec_cfg ) ,
                "::" , stringify ! ( w ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const vpx_codec_dec_cfg ) ) . h as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( vpx_codec_dec_cfg ) ,
                "::" , stringify ! ( h ) ));
}
impl Clone for vpx_codec_dec_cfg {
    fn clone(&self) -> Self { *self }
}
pub type vpx_codec_dec_cfg_t = vpx_codec_dec_cfg;
extern "C" {
    pub fn vpx_codec_dec_init_ver(ctx: *mut vpx_codec_ctx_t,
                                  iface: *mut vpx_codec_iface_t,
                                  cfg: *const vpx_codec_dec_cfg_t,
                                  flags: vpx_codec_flags_t,
                                  ver: ::std::os::raw::c_int)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_peek_stream_info(iface: *mut vpx_codec_iface_t,
                                      data: *const u8,
                                      data_sz: ::std::os::raw::c_uint,
                                      si: *mut vpx_codec_stream_info_t)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_get_stream_info(ctx: *mut vpx_codec_ctx_t,
                                     si: *mut vpx_codec_stream_info_t)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_decode(ctx: *mut vpx_codec_ctx_t, data: *const u8,
                            data_sz: ::std::os::raw::c_uint,
                            user_priv: *mut ::std::os::raw::c_void,
                            deadline: ::std::os::raw::c_long)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_get_frame(ctx: *mut vpx_codec_ctx_t,
                               iter: *mut vpx_codec_iter_t)
     -> *mut vpx_image_t;
}
pub type vpx_codec_put_frame_cb_fn_t =
    ::std::option::Option<unsafe extern "C" fn(user_priv:
                                                   *mut ::std::os::raw::c_void,
                                               img: *const vpx_image_t)>;
extern "C" {
    pub fn vpx_codec_register_put_frame_cb(ctx: *mut vpx_codec_ctx_t,
                                           cb: vpx_codec_put_frame_cb_fn_t,
                                           user_priv:
                                               *mut ::std::os::raw::c_void)
     -> vpx_codec_err_t;
}
pub type vpx_codec_put_slice_cb_fn_t =
    ::std::option::Option<unsafe extern "C" fn(user_priv:
                                                   *mut ::std::os::raw::c_void,
                                               img: *const vpx_image_t,
                                               valid: *const vpx_image_rect_t,
                                               update:
                                                   *const vpx_image_rect_t)>;
extern "C" {
    pub fn vpx_codec_register_put_slice_cb(ctx: *mut vpx_codec_ctx_t,
                                           cb: vpx_codec_put_slice_cb_fn_t,
                                           user_priv:
                                               *mut ::std::os::raw::c_void)
     -> vpx_codec_err_t;
}
extern "C" {
    pub fn vpx_codec_set_frame_buffer_functions(ctx: *mut vpx_codec_ctx_t,
                                                cb_get:
                                                    vpx_get_frame_buffer_cb_fn_t,
                                                cb_release:
                                                    vpx_release_frame_buffer_cb_fn_t,
                                                cb_priv:
                                                    *mut ::std::os::raw::c_void)
     -> vpx_codec_err_t;
}

