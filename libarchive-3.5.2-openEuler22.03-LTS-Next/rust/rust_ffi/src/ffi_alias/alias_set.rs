pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub type iconv_t = *mut libc::c_void;
pub type wchar_t = wchar::wchar_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type intmax_t = __intmax_t;
pub type ssize_t = __ssize_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type mode_t = __mode_t;
pub type time_t = __time_t;
pub type la_int64_t = int64_t;
pub type la_ssize_t = ssize_t;
pub type z_stream = libz_sys::z_stream;
pub type z_streamp = *mut z_stream;
pub type Byte = libc::c_uchar;
pub type Bytef = libz_sys::Bytef;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
pub type lzma_reserved_enum = libc::c_uint;
pub type lzma_ret = libc::c_uint;
pub type lzma_action = libc::c_uint;


pub type UInt16 = libc::c_ushort;
pub type UInt32 = libc::c_uint;
pub type CPpmd_Void_Ref = UInt32;
pub type Int32 = libc::c_int;
pub type CPpmd8_Context_Ref = UInt32;
pub type CPpmd_State_Ref = UInt32;
pub type Bool = libc::c_int;


pub type __syscall_slong_t = libc::c_long;
pub type __ino_t = libc::c_ulong;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub const _ISprint_m: libc::c_uint = 16384;

pub type UInt64 = libc::c_ulonglong;
pub type __int8_t = libc::c_schar;
pub type int8_t = __int8_t;
pub const CDE_OK: CDE_RETURN_VALUES = 0;
pub const CDE_OUT_OF_BOUNDS: CDE_RETURN_VALUES = 3;
pub const CDE_PARAM: CDE_RETURN_VALUES = 2;
pub const FILTER_ARM: FILTER_TYPE = 3;
pub const FILTER_E8E9: FILTER_TYPE = 2;
pub const FILTER_E8: FILTER_TYPE = 1;
pub const FILTER_DELTA: FILTER_TYPE = 0;
pub const ESCAPE: obj = 15;
pub type obj = libc::c_uint;
pub const HFL_SKIP_IF_UNKNOWN: HEADER_FLAGS = 4;
pub const HEAD_MARK: HEADER_TYPE = 0;
pub const HEAD_ENDARC: HEADER_TYPE = 5;
pub const HEAD_CRYPT: HEADER_TYPE = 4;
pub const CRC32: FILE_FLAGS = 4;
pub const UTIME: FILE_FLAGS = 2;
pub const REDIR_TYPE_NONE: REDIR_TYPE = 0;
pub const UNKNOWN_UNPACKED_SIZE: FILE_FLAGS = 8;
pub const EX_SUBDATA: EXTRA = 7;
pub const EX_CRYPT: EXTRA = 1;
pub const EX_VERSION: EXTRA = 4;
pub const EX_UOWNER: EXTRA = 6;
pub const REDIR_TYPE_HARDLINK: REDIR_TYPE = 4;
pub const REDIR_TYPE_WINSYMLINK: REDIR_TYPE = 2;
pub const REDIR_TYPE_UNIXSYMLINK: REDIR_TYPE = 1;
pub const EX_REDIR: EXTRA = 5;
pub const HAS_UNIX_NS: HTIME_FLAGS = 16;
pub const HAS_ATIME: HTIME_FLAGS = 8;
pub const HAS_CTIME: HTIME_FLAGS = 4;
pub const HAS_MTIME: HTIME_FLAGS = 2;
pub const IS_UNIX: HTIME_FLAGS = 1;
pub type HTIME_FLAGS = libc::c_uint;
pub const EX_HTIME: EXTRA = 3;
pub const BLAKE2sp: HASH_TYPE = 0;
pub type HASH_TYPE = libc::c_uint;
pub const EX_HASH: EXTRA = 2;
pub const HOST_UNIX: HOST_OS = 1;
pub const ATTR_SYSTEM: FILE_ATTRS = 4;
pub const ATTR_HIDDEN: FILE_ATTRS = 2;
pub const ATTR_READONLY: FILE_ATTRS = 1;
pub const ATTR_DIRECTORY: FILE_ATTRS = 16;
pub const HOST_WINDOWS: HOST_OS = 0;
pub const SOLID: COMP_INFO_FLAGS = 64;
pub const DIRECTORY: FILE_FLAGS = 1;
pub const HFL_DATA: HEADER_FLAGS = 2;
pub const HFL_EXTRA_DATA: HEADER_FLAGS = 1;
pub type HOST_OS = libc::c_uint;
pub type COMP_INFO_FLAGS = libc::c_uint;
pub type FILE_ATTRS = libc::c_uint;
pub type FILE_FLAGS = libc::c_uint;
pub const HEAD_FILE: HEADER_TYPE = 2;
pub const HEAD_SERVICE: HEADER_TYPE = 3;
pub const RECOVERY: LOCATOR_FLAGS = 2;
pub const QLIST: LOCATOR_FLAGS = 1;
pub type LOCATOR_FLAGS = libc::c_uint;
pub const LOCATOR: MAIN_EXTRA = 1;
pub const VOLUME_NUMBER: MAIN_FLAGS = 2;
pub const SOLID_0: MAIN_FLAGS = 4;
pub const VOLUME: MAIN_FLAGS = 1;
pub type MAIN_EXTRA = libc::c_uint;
pub type MAIN_FLAGS = libc::c_uint;
pub const LOCK: MAIN_FLAGS = 16;
pub const PROTECT: MAIN_FLAGS = 8;
pub const HEAD_MAIN: HEADER_TYPE = 1;
pub const HFL_SPLIT_BEFORE: HEADER_FLAGS = 8;
pub const HFL_SPLIT_AFTER: HEADER_FLAGS = 16;
pub type HEADER_TYPE = libc::c_uint;
pub const HEAD_UNKNOWN: HEADER_TYPE = 255;
pub const BEST: COMPRESSION_METHOD = 5;
pub const GOOD: COMPRESSION_METHOD = 4;
pub const NORMAL: COMPRESSION_METHOD = 3;
pub const FAST: COMPRESSION_METHOD = 2;
pub const FASTEST: COMPRESSION_METHOD = 1;
pub const STORE: COMPRESSION_METHOD = 0;
pub type COMPRESSION_METHOD = libc::c_uint;
pub const CDE_ALLOC: CDE_RETURN_VALUES = 1;
pub type EXTRA = libc::c_uint;
pub type REDIR_TYPE = libc::c_uint;
pub const REDIR_TYPE_FILECOPY: REDIR_TYPE = 5;
pub const REDIR_TYPE_JUNCTION: REDIR_TYPE = 3;
pub type FILTER_TYPE = libc::c_uint;
pub const FILTER_NONE: FILTER_TYPE = 8;
pub const FILTER_PPM: FILTER_TYPE = 7;
pub const FILTER_ITANIUM: FILTER_TYPE = 6;
pub const FILTER_RGB: FILTER_TYPE = 5;
pub const FILTER_AUDIO: FILTER_TYPE = 4;
pub type CDE_RETURN_VALUES = libc::c_uint;
pub type HEADER_FLAGS = libc::c_uint;
pub const HFL_INHERITED: HEADER_FLAGS = 64;
pub const HFL_CHILD: HEADER_FLAGS = 32;
pub const SIZE_MAX: size_t=1<<31;

pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;


pub const LAST_WT: warc_type_t = 9;
pub const WT_CONT: warc_type_t = 8;
pub const WT_CONV: warc_type_t = 7;
pub const WT_RVIS: warc_type_t = 6;
pub const WT_REQ: warc_type_t = 4;
pub const WT_META: warc_type_t = 2;
pub const WT_INFO: warc_type_t = 1;
pub const WT_NONE: warc_type_t = 0;

pub const WT_RSP: warc_type_t = 5;
pub const WT_RSRC: warc_type_t = 3;
pub type warc_type_t = libc::c_uint;


pub type xmlChar = libc::c_uchar;
pub type xmlInputCloseCallback
=
Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
pub type xmlInputReadCallback
=
Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_char,
                            _: libc::c_int) -> libc::c_int>;
pub type xmlParserSeverities = libc::c_uint;
pub const HEADER_MAGIC:libc::c_int=0x78617221;
pub const HEADER_SIZE:libc::c_int=28;
pub const HEADER_VERSION:libc::c_int=1;
pub const CKSUM_SHA1:libc::c_int=1;
pub const CKSUM_NONE:libc::c_int=0;
pub const CKSUM_MD5:libc::c_int=2;
pub const MD5_SIZE:libc::c_int=16;
pub const SHA1_SIZE:libc::c_int=20;
pub const MAX_SUM_SIZE:libc::c_int=20;
pub const XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub const XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;

pub const XML_READER_TYPE_XML_DECLARATION: C2RustUnnamed = 17;
pub const XML_READER_TYPE_END_ENTITY: C2RustUnnamed = 16;
pub const XML_READER_TYPE_END_ELEMENT: C2RustUnnamed = 15;
pub const XML_READER_TYPE_SIGNIFICANT_WHITESPACE: C2RustUnnamed = 14;
pub const XML_READER_TYPE_WHITESPACE: C2RustUnnamed = 13;
pub const XML_READER_TYPE_NOTATION: C2RustUnnamed = 12;
pub const XML_READER_TYPE_DOCUMENT_FRAGMENT: C2RustUnnamed = 11;
pub const XML_READER_TYPE_DOCUMENT_TYPE: C2RustUnnamed = 10;
pub const XML_READER_TYPE_DOCUMENT: C2RustUnnamed = 9;
pub const XML_READER_TYPE_COMMENT: C2RustUnnamed = 8;
pub const XML_READER_TYPE_PROCESSING_INSTRUCTION: C2RustUnnamed = 7;
pub const XML_READER_TYPE_ENTITY: C2RustUnnamed = 6;
pub const XML_READER_TYPE_ENTITY_REFERENCE: C2RustUnnamed = 5;
pub const XML_READER_TYPE_CDATA: C2RustUnnamed = 4;
pub const XML_READER_TYPE_TEXT: C2RustUnnamed = 3;
pub const XML_READER_TYPE_ATTRIBUTE: C2RustUnnamed = 2;
pub const XML_READER_TYPE_ELEMENT: C2RustUnnamed = 1;
pub const XML_READER_TYPE_NONE: C2RustUnnamed = 0;



pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_STREAM_END: lzma_ret = 1;
pub const LZMA_OK: lzma_ret = 0;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;

pub type nl_item = libc::c_int;
pub const _NL_NUM: libc::c_uint = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: libc::c_uint = 786448;
pub const _NL_IDENTIFICATION_CODESET: libc::c_uint = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: libc::c_uint = 786446;
pub const _NL_IDENTIFICATION_DATE: libc::c_uint = 786445;
pub const _NL_IDENTIFICATION_REVISION: libc::c_uint = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: libc::c_uint = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: libc::c_uint = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: libc::c_uint = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: libc::c_uint = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: libc::c_uint = 786439;
pub const _NL_IDENTIFICATION_FAX: libc::c_uint = 786438;
pub const _NL_IDENTIFICATION_TEL: libc::c_uint = 786437;
pub const _NL_IDENTIFICATION_EMAIL: libc::c_uint = 786436;
pub const _NL_IDENTIFICATION_CONTACT: libc::c_uint = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: libc::c_uint = 786434;
pub const _NL_IDENTIFICATION_SOURCE: libc::c_uint = 786433;
pub const _NL_IDENTIFICATION_TITLE: libc::c_uint = 786432;
pub const _NL_NUM_LC_MEASUREMENT: libc::c_uint = 720898;
pub const _NL_MEASUREMENT_CODESET: libc::c_uint = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: libc::c_uint = 720896;
pub const _NL_NUM_LC_TELEPHONE: libc::c_uint = 655365;
pub const _NL_TELEPHONE_CODESET: libc::c_uint = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: libc::c_uint = 655363;
pub const _NL_TELEPHONE_INT_SELECT: libc::c_uint = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: libc::c_uint = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: libc::c_uint = 655360;
pub const _NL_NUM_LC_ADDRESS: libc::c_uint = 589837;
pub const _NL_ADDRESS_CODESET: libc::c_uint = 589836;
pub const _NL_ADDRESS_LANG_LIB: libc::c_uint = 589835;
pub const _NL_ADDRESS_LANG_TERM: libc::c_uint = 589834;
pub const _NL_ADDRESS_LANG_AB: libc::c_uint = 589833;
pub const _NL_ADDRESS_LANG_NAME: libc::c_uint = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: libc::c_uint = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: libc::c_uint = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: libc::c_uint = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: libc::c_uint = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: libc::c_uint = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: libc::c_uint = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: libc::c_uint = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: libc::c_uint = 589824;
pub const _NL_NUM_LC_NAME: libc::c_uint = 524295;
pub const _NL_NAME_CODESET: libc::c_uint = 524294;
pub const _NL_NAME_NAME_MS: libc::c_uint = 524293;
pub const _NL_NAME_NAME_MISS: libc::c_uint = 524292;
pub const _NL_NAME_NAME_MRS: libc::c_uint = 524291;
pub const _NL_NAME_NAME_MR: libc::c_uint = 524290;
pub const _NL_NAME_NAME_GEN: libc::c_uint = 524289;
pub const _NL_NAME_NAME_FMT: libc::c_uint = 524288;
pub const _NL_NUM_LC_PAPER: libc::c_uint = 458755;
pub const _NL_PAPER_CODESET: libc::c_uint = 458754;
pub const _NL_PAPER_WIDTH: libc::c_uint = 458753;
pub const _NL_PAPER_HEIGHT: libc::c_uint = 458752;
pub const _NL_NUM_LC_MESSAGES: libc::c_uint = 327685;
pub const _NL_MESSAGES_CODESET: libc::c_uint = 327684;
pub const __NOSTR: libc::c_uint = 327683;
pub const __YESSTR: libc::c_uint = 327682;
pub const __NOEXPR: libc::c_uint = 327681;
pub const __YESEXPR: libc::c_uint = 327680;
pub const _NL_NUM_LC_NUMERIC: libc::c_uint = 65542;
pub const _NL_NUMERIC_CODESET: libc::c_uint = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: libc::c_uint = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: libc::c_uint = 65539;
pub const __GROUPING: libc::c_uint = 65538;
pub const THOUSEP: libc::c_uint = 65537;
pub const __THOUSANDS_SEP: libc::c_uint = 65537;
pub const RADIXCHAR: libc::c_uint = 65536;
pub const __DECIMAL_POINT: libc::c_uint = 65536;
pub const _NL_NUM_LC_MONETARY: libc::c_uint = 262190;
pub const _NL_MONETARY_CODESET: libc::c_uint = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: libc::c_uint = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: libc::c_uint = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: libc::c_uint = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: libc::c_uint = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: libc::c_uint = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: libc::c_uint = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: libc::c_uint = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: libc::c_uint = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: libc::c_uint = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: libc::c_uint = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: libc::c_uint = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: libc::c_uint = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: libc::c_uint = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: libc::c_uint = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: libc::c_uint = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: libc::c_uint = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: libc::c_uint = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: libc::c_uint = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: libc::c_uint = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: libc::c_uint = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: libc::c_uint = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: libc::c_uint = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: libc::c_uint = 262166;
pub const __INT_N_SIGN_POSN: libc::c_uint = 262165;
pub const __INT_P_SIGN_POSN: libc::c_uint = 262164;
pub const __INT_N_SEP_BY_SPACE: libc::c_uint = 262163;
pub const __INT_N_CS_PRECEDES: libc::c_uint = 262162;
pub const __INT_P_SEP_BY_SPACE: libc::c_uint = 262161;
pub const __INT_P_CS_PRECEDES: libc::c_uint = 262160;
pub const _NL_MONETARY_CRNCYSTR: libc::c_uint = 262159;
pub const __N_SIGN_POSN: libc::c_uint = 262158;
pub const __P_SIGN_POSN: libc::c_uint = 262157;
pub const __N_SEP_BY_SPACE: libc::c_uint = 262156;
pub const __N_CS_PRECEDES: libc::c_uint = 262155;
pub const __P_SEP_BY_SPACE: libc::c_uint = 262154;
pub const __P_CS_PRECEDES: libc::c_uint = 262153;
pub const __FRAC_DIGITS: libc::c_uint = 262152;
pub const __INT_FRAC_DIGITS: libc::c_uint = 262151;
pub const __NEGATIVE_SIGN: libc::c_uint = 262150;
pub const __POSITIVE_SIGN: libc::c_uint = 262149;
pub const __MON_GROUPING: libc::c_uint = 262148;
pub const __MON_THOUSANDS_SEP: libc::c_uint = 262147;
pub const __MON_DECIMAL_POINT: libc::c_uint = 262146;
pub const __CURRENCY_SYMBOL: libc::c_uint = 262145;
pub const __INT_CURR_SYMBOL: libc::c_uint = 262144;
pub const _NL_NUM_LC_CTYPE: libc::c_uint = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: libc::c_uint = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: libc::c_uint = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: libc::c_uint = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: libc::c_uint = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: libc::c_uint = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: libc::c_uint = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: libc::c_uint = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: libc::c_uint = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: libc::c_uint = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: libc::c_uint = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: libc::c_uint = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: libc::c_uint = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: libc::c_uint = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: libc::c_uint = 72;
pub const _NL_CTYPE_NONASCII_CASE: libc::c_uint = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: libc::c_uint = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: libc::c_uint = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: libc::c_uint = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: libc::c_uint = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: libc::c_uint = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: libc::c_uint = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: libc::c_uint = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: libc::c_uint = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: libc::c_uint = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: libc::c_uint = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: libc::c_uint = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: libc::c_uint = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: libc::c_uint = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: libc::c_uint = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: libc::c_uint = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: libc::c_uint = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: libc::c_uint = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: libc::c_uint = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: libc::c_uint = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: libc::c_uint = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: libc::c_uint = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: libc::c_uint = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: libc::c_uint = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: libc::c_uint = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: libc::c_uint = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: libc::c_uint = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: libc::c_uint = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: libc::c_uint = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: libc::c_uint = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: libc::c_uint = 41;
pub const _NL_CTYPE_INDIGITS9_WC: libc::c_uint = 40;
pub const _NL_CTYPE_INDIGITS8_WC: libc::c_uint = 39;
pub const _NL_CTYPE_INDIGITS7_WC: libc::c_uint = 38;
pub const _NL_CTYPE_INDIGITS6_WC: libc::c_uint = 37;
pub const _NL_CTYPE_INDIGITS5_WC: libc::c_uint = 36;
pub const _NL_CTYPE_INDIGITS4_WC: libc::c_uint = 35;
pub const _NL_CTYPE_INDIGITS3_WC: libc::c_uint = 34;
pub const _NL_CTYPE_INDIGITS2_WC: libc::c_uint = 33;
pub const _NL_CTYPE_INDIGITS1_WC: libc::c_uint = 32;
pub const _NL_CTYPE_INDIGITS0_WC: libc::c_uint = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: libc::c_uint = 30;
pub const _NL_CTYPE_INDIGITS9_MB: libc::c_uint = 29;
pub const _NL_CTYPE_INDIGITS8_MB: libc::c_uint = 28;
pub const _NL_CTYPE_INDIGITS7_MB: libc::c_uint = 27;
pub const _NL_CTYPE_INDIGITS6_MB: libc::c_uint = 26;
pub const _NL_CTYPE_INDIGITS5_MB: libc::c_uint = 25;
pub const _NL_CTYPE_INDIGITS4_MB: libc::c_uint = 24;
pub const _NL_CTYPE_INDIGITS3_MB: libc::c_uint = 23;
pub const _NL_CTYPE_INDIGITS2_MB: libc::c_uint = 22;
pub const _NL_CTYPE_INDIGITS1_MB: libc::c_uint = 21;
pub const _NL_CTYPE_INDIGITS0_MB: libc::c_uint = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: libc::c_uint = 19;
pub const _NL_CTYPE_MAP_OFFSET: libc::c_uint = 18;
pub const _NL_CTYPE_CLASS_OFFSET: libc::c_uint = 17;
pub const _NL_CTYPE_TOLOWER32: libc::c_uint = 16;
pub const _NL_CTYPE_TOUPPER32: libc::c_uint = 15;
pub const CODESET: libc::c_uint = 14;
pub const _NL_CTYPE_CODESET_NAME: libc::c_uint = 14;
pub const _NL_CTYPE_MB_CUR_MAX: libc::c_uint = 13;
pub const _NL_CTYPE_WIDTH: libc::c_uint = 12;
pub const _NL_CTYPE_MAP_NAMES: libc::c_uint = 11;
pub const _NL_CTYPE_CLASS_NAMES: libc::c_uint = 10;
pub const _NL_CTYPE_GAP6: libc::c_uint = 9;
pub const _NL_CTYPE_GAP5: libc::c_uint = 8;
pub const _NL_CTYPE_GAP4: libc::c_uint = 7;
pub const _NL_CTYPE_GAP3: libc::c_uint = 6;
pub const _NL_CTYPE_CLASS32: libc::c_uint = 5;
pub const _NL_CTYPE_GAP2: libc::c_uint = 4;
pub const _NL_CTYPE_TOLOWER: libc::c_uint = 3;
pub const _NL_CTYPE_GAP1: libc::c_uint = 2;
pub const _NL_CTYPE_TOUPPER: libc::c_uint = 1;
pub const _NL_CTYPE_CLASS: libc::c_uint = 0;
pub const _NL_NUM_LC_COLLATE: libc::c_uint = 196627;
pub const _NL_COLLATE_CODESET: libc::c_uint = 196626;
pub const _NL_COLLATE_COLLSEQWC: libc::c_uint = 196625;
pub const _NL_COLLATE_COLLSEQMB: libc::c_uint = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: libc::c_uint = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: libc::c_uint = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: libc::c_uint = 196621;
pub const _NL_COLLATE_INDIRECTWC: libc::c_uint = 196620;
pub const _NL_COLLATE_EXTRAWC: libc::c_uint = 196619;
pub const _NL_COLLATE_WEIGHTWC: libc::c_uint = 196618;
pub const _NL_COLLATE_TABLEWC: libc::c_uint = 196617;
pub const _NL_COLLATE_GAP3: libc::c_uint = 196616;
pub const _NL_COLLATE_GAP2: libc::c_uint = 196615;
pub const _NL_COLLATE_GAP1: libc::c_uint = 196614;
pub const _NL_COLLATE_INDIRECTMB: libc::c_uint = 196613;
pub const _NL_COLLATE_EXTRAMB: libc::c_uint = 196612;
pub const _NL_COLLATE_WEIGHTMB: libc::c_uint = 196611;
pub const _NL_COLLATE_TABLEMB: libc::c_uint = 196610;
pub const _NL_COLLATE_RULESETS: libc::c_uint = 196609;
pub const _NL_COLLATE_NRULES: libc::c_uint = 196608;
pub const _NL_NUM_LC_TIME: libc::c_uint = 131231;
pub const _NL_WABALTMON_12: libc::c_uint = 131230;
pub const _NL_WABALTMON_11: libc::c_uint = 131229;
pub const _NL_WABALTMON_10: libc::c_uint = 131228;
pub const _NL_WABALTMON_9: libc::c_uint = 131227;
pub const _NL_WABALTMON_8: libc::c_uint = 131226;
pub const _NL_WABALTMON_7: libc::c_uint = 131225;
pub const _NL_WABALTMON_6: libc::c_uint = 131224;
pub const _NL_WABALTMON_5: libc::c_uint = 131223;
pub const _NL_WABALTMON_4: libc::c_uint = 131222;
pub const _NL_WABALTMON_3: libc::c_uint = 131221;
pub const _NL_WABALTMON_2: libc::c_uint = 131220;
pub const _NL_WABALTMON_1: libc::c_uint = 131219;
pub const _NL_ABALTMON_12: libc::c_uint = 131218;
pub const _NL_ABALTMON_11: libc::c_uint = 131217;
pub const _NL_ABALTMON_10: libc::c_uint = 131216;
pub const _NL_ABALTMON_9: libc::c_uint = 131215;
pub const _NL_ABALTMON_8: libc::c_uint = 131214;
pub const _NL_ABALTMON_7: libc::c_uint = 131213;
pub const _NL_ABALTMON_6: libc::c_uint = 131212;
pub const _NL_ABALTMON_5: libc::c_uint = 131211;
pub const _NL_ABALTMON_4: libc::c_uint = 131210;
pub const _NL_ABALTMON_3: libc::c_uint = 131209;
pub const _NL_ABALTMON_2: libc::c_uint = 131208;
pub const _NL_ABALTMON_1: libc::c_uint = 131207;
pub const _NL_WALTMON_12: libc::c_uint = 131206;
pub const _NL_WALTMON_11: libc::c_uint = 131205;
pub const _NL_WALTMON_10: libc::c_uint = 131204;
pub const _NL_WALTMON_9: libc::c_uint = 131203;
pub const _NL_WALTMON_8: libc::c_uint = 131202;
pub const _NL_WALTMON_7: libc::c_uint = 131201;
pub const _NL_WALTMON_6: libc::c_uint = 131200;
pub const _NL_WALTMON_5: libc::c_uint = 131199;
pub const _NL_WALTMON_4: libc::c_uint = 131198;
pub const _NL_WALTMON_3: libc::c_uint = 131197;
pub const _NL_WALTMON_2: libc::c_uint = 131196;
pub const _NL_WALTMON_1: libc::c_uint = 131195;
pub const __ALTMON_12: libc::c_uint = 131194;
pub const __ALTMON_11: libc::c_uint = 131193;
pub const __ALTMON_10: libc::c_uint = 131192;
pub const __ALTMON_9: libc::c_uint = 131191;
pub const __ALTMON_8: libc::c_uint = 131190;
pub const __ALTMON_7: libc::c_uint = 131189;
pub const __ALTMON_6: libc::c_uint = 131188;
pub const __ALTMON_5: libc::c_uint = 131187;
pub const __ALTMON_4: libc::c_uint = 131186;
pub const __ALTMON_3: libc::c_uint = 131185;
pub const __ALTMON_2: libc::c_uint = 131184;
pub const __ALTMON_1: libc::c_uint = 131183;
pub const _NL_TIME_CODESET: libc::c_uint = 131182;
pub const _NL_W_DATE_FMT: libc::c_uint = 131181;
pub const _DATE_FMT: libc::c_uint = 131180;
pub const _NL_TIME_TIMEZONE: libc::c_uint = 131179;
pub const _NL_TIME_CAL_DIRECTION: libc::c_uint = 131178;
pub const _NL_TIME_FIRST_WORKDAY: libc::c_uint = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: libc::c_uint = 131176;
pub const _NL_TIME_WEEK_1STWEEK: libc::c_uint = 131175;
pub const _NL_TIME_WEEK_1STDAY: libc::c_uint = 131174;
pub const _NL_TIME_WEEK_NDAYS: libc::c_uint = 131173;
pub const _NL_WERA_T_FMT: libc::c_uint = 131172;
pub const _NL_WERA_D_T_FMT: libc::c_uint = 131171;
pub const _NL_WALT_DIGITS: libc::c_uint = 131170;
pub const _NL_WERA_D_FMT: libc::c_uint = 131169;
pub const _NL_WERA_YEAR: libc::c_uint = 131168;
pub const _NL_WT_FMT_AMPM: libc::c_uint = 131167;
pub const _NL_WT_FMT: libc::c_uint = 131166;
pub const _NL_WD_FMT: libc::c_uint = 131165;
pub const _NL_WD_T_FMT: libc::c_uint = 131164;
pub const _NL_WPM_STR: libc::c_uint = 131163;
pub const _NL_WAM_STR: libc::c_uint = 131162;
pub const _NL_WMON_12: libc::c_uint = 131161;
pub const _NL_WMON_11: libc::c_uint = 131160;
pub const _NL_WMON_10: libc::c_uint = 131159;
pub const _NL_WMON_9: libc::c_uint = 131158;
pub const _NL_WMON_8: libc::c_uint = 131157;
pub const _NL_WMON_7: libc::c_uint = 131156;
pub const _NL_WMON_6: libc::c_uint = 131155;
pub const _NL_WMON_5: libc::c_uint = 131154;
pub const _NL_WMON_4: libc::c_uint = 131153;
pub const _NL_WMON_3: libc::c_uint = 131152;
pub const _NL_WMON_2: libc::c_uint = 131151;
pub const _NL_WMON_1: libc::c_uint = 131150;
pub const _NL_WABMON_12: libc::c_uint = 131149;
pub const _NL_WABMON_11: libc::c_uint = 131148;
pub const _NL_WABMON_10: libc::c_uint = 131147;
pub const _NL_WABMON_9: libc::c_uint = 131146;
pub const _NL_WABMON_8: libc::c_uint = 131145;
pub const _NL_WABMON_7: libc::c_uint = 131144;
pub const _NL_WABMON_6: libc::c_uint = 131143;
pub const _NL_WABMON_5: libc::c_uint = 131142;
pub const _NL_WABMON_4: libc::c_uint = 131141;
pub const _NL_WABMON_3: libc::c_uint = 131140;
pub const _NL_WABMON_2: libc::c_uint = 131139;
pub const _NL_WABMON_1: libc::c_uint = 131138;
pub const _NL_WDAY_7: libc::c_uint = 131137;
pub const _NL_WDAY_6: libc::c_uint = 131136;
pub const _NL_WDAY_5: libc::c_uint = 131135;
pub const _NL_WDAY_4: libc::c_uint = 131134;
pub const _NL_WDAY_3: libc::c_uint = 131133;
pub const _NL_WDAY_2: libc::c_uint = 131132;
pub const _NL_WDAY_1: libc::c_uint = 131131;
pub const _NL_WABDAY_7: libc::c_uint = 131130;
pub const _NL_WABDAY_6: libc::c_uint = 131129;
pub const _NL_WABDAY_5: libc::c_uint = 131128;
pub const _NL_WABDAY_4: libc::c_uint = 131127;
pub const _NL_WABDAY_3: libc::c_uint = 131126;
pub const _NL_WABDAY_2: libc::c_uint = 131125;
pub const _NL_WABDAY_1: libc::c_uint = 131124;
pub const _NL_TIME_ERA_ENTRIES: libc::c_uint = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: libc::c_uint = 131122;
pub const ERA_T_FMT: libc::c_uint = 131121;
pub const ERA_D_T_FMT: libc::c_uint = 131120;
pub const ALT_DIGITS: libc::c_uint = 131119;
pub const ERA_D_FMT: libc::c_uint = 131118;
pub const __ERA_YEAR: libc::c_uint = 131117;
pub const ERA: libc::c_uint = 131116;
pub const T_FMT_AMPM: libc::c_uint = 131115;
pub const T_FMT: libc::c_uint = 131114;
pub const D_FMT: libc::c_uint = 131113;
pub const D_T_FMT: libc::c_uint = 131112;
pub const PM_STR: libc::c_uint = 131111;
pub const AM_STR: libc::c_uint = 131110;
pub const MON_12: libc::c_uint = 131109;
pub const MON_11: libc::c_uint = 131108;
pub const MON_10: libc::c_uint = 131107;
pub const MON_9: libc::c_uint = 131106;
pub const MON_8: libc::c_uint = 131105;
pub const MON_7: libc::c_uint = 131104;
pub const MON_6: libc::c_uint = 131103;
pub const MON_5: libc::c_uint = 131102;
pub const MON_4: libc::c_uint = 131101;
pub const MON_3: libc::c_uint = 131100;
pub const MON_2: libc::c_uint = 131099;
pub const MON_1: libc::c_uint = 131098;
pub const ABMON_12: libc::c_uint = 131097;
pub const ABMON_11: libc::c_uint = 131096;
pub const ABMON_10: libc::c_uint = 131095;
pub const ABMON_9: libc::c_uint = 131094;
pub const ABMON_8: libc::c_uint = 131093;
pub const ABMON_7: libc::c_uint = 131092;
pub const ABMON_6: libc::c_uint = 131091;
pub const ABMON_5: libc::c_uint = 131090;
pub const ABMON_4: libc::c_uint = 131089;
pub const ABMON_3: libc::c_uint = 131088;
pub const ABMON_2: libc::c_uint = 131087;
pub const ABMON_1: libc::c_uint = 131086;
pub const DAY_7: libc::c_uint = 131085;
pub const DAY_6: libc::c_uint = 131084;
pub const DAY_5: libc::c_uint = 131083;
pub const DAY_4: libc::c_uint = 131082;
pub const DAY_3: libc::c_uint = 131081;
pub const DAY_2: libc::c_uint = 131080;
pub const DAY_1: libc::c_uint = 131079;
pub const ABDAY_7: libc::c_uint = 131078;
pub const ABDAY_6: libc::c_uint = 131077;
pub const ABDAY_5: libc::c_uint = 131076;
pub const ABDAY_4: libc::c_uint = 131075;
pub const ABDAY_3: libc::c_uint = 131074;
pub const ABDAY_2: libc::c_uint = 131073;
pub const ABDAY_1: libc::c_uint = 131072;
pub type enctype = libc::c_uint;
pub const XZ: enctype = 4;
pub const LZMA: enctype = 3;
pub const BZIP2: enctype = 2;
pub const GZIP: enctype = 1;
pub const NONE: enctype = 0;


pub type xmlstatus = libc::c_uint;
pub const UNKNOWN: xmlstatus = 77;
pub const HAS_DATA:libc::c_uint=0x00001;
pub const HAS_PATHNAME:libc::c_uint=0x00002;
pub const HAS_SYMLINK:libc::c_uint=	0x00004;
pub const HAS_TIME:libc::c_uint= 0x00008;
pub const HAS_UID:libc::c_uint=	0x00010;
pub const HAS_GID:libc::c_uint=	0x00020;
pub const HAS_MODE:libc::c_uint=0x00040;
pub const HAS_TYPE:libc::c_uint=0x00080;
pub const HAS_DEV:libc::c_uint=	0x00100;
pub const HAS_DEVMAJOR:libc::c_uint=0x00200;
pub const HAS_DEVMINOR:libc::c_uint=0x00400;
pub const HAS_INO:libc::c_uint=	0x00800;
pub const HAS_FFLAGS:libc::c_uint=0x01000;
pub const HAS_XATTR:libc::c_uint=0x02000;
pub const HAS_ACL:libc::c_uint=0x04000;
pub const HAS_CTIME_XAR:libc::c_uint=0x08000;
pub const HAS_MTIME_XAR:libc::c_uint=0x10000;
pub const HAS_ATIME_XAR:libc::c_uint=0x20000;
pub const FILE_EXT2_Reserved: xmlstatus = 76;
pub const FILE_EXT2_TopDir: xmlstatus = 75;
pub const FILE_EXT2_DirSync: xmlstatus = 74;
pub const FILE_EXT2_NoTail: xmlstatus = 73;
pub const FILE_EXT2_Journaled: xmlstatus = 72;
pub const FILE_EXT2_iMagic: xmlstatus = 71;
pub const FILE_EXT2_HashIndexed: xmlstatus = 70;
pub const FILE_EXT2_BTree: xmlstatus = 69;
pub const FILE_EXT2_CompError: xmlstatus = 68;
pub const FILE_EXT2_NoCompBlock: xmlstatus = 67;
pub const FILE_EXT2_CompBlock: xmlstatus = 66;
pub const FILE_EXT2_CompDirty: xmlstatus = 65;
pub const FILE_EXT2_NoAtime: xmlstatus = 64;
pub const FILE_EXT2_NoDump: xmlstatus = 63;
pub const FILE_EXT2_AppendOnly: xmlstatus = 62;
pub const FILE_EXT2_Immutable: xmlstatus = 61;
pub const FILE_EXT2_Synchronous: xmlstatus = 60;
pub const FILE_EXT2_Compress: xmlstatus = 59;
pub const FILE_EXT2_Undelete: xmlstatus = 58;
pub const FILE_EXT2_SecureDeletion: xmlstatus = 57;
/* Linux file flags. */
pub const FILE_EXT2: xmlstatus = 56;
pub const FILE_FLAGS_SYS_SNAPSHOT: xmlstatus = 55;
pub const FILE_FLAGS_SYS_NOUNLINK: xmlstatus = 54;
pub const FILE_FLAGS_SYS_APPEND: xmlstatus = 53;
pub const FILE_FLAGS_SYS_IMMUTABLE: xmlstatus = 52;
pub const FILE_FLAGS_SYS_ARCHIVED: xmlstatus = 51;
pub const FILE_FLAGS_USER_NOUNLINK: xmlstatus = 50;
pub const FILE_FLAGS_USER_OPAQUE: xmlstatus = 49;
pub const FILE_FLAGS_USER_APPEND: xmlstatus = 48;
pub const FILE_FLAGS_USER_IMMUTABLE: xmlstatus = 47;
pub const FILE_FLAGS_USER_NODUMP: xmlstatus = 46;
/* BSD file flags. */
pub const FILE_FLAGS: xmlstatus = 45;
pub const FILE_ACL_APPLEEXTENDED: xmlstatus = 44;
pub const FILE_ACL_ACCESS: xmlstatus = 43;
pub const FILE_ACL_DEFAULT: xmlstatus = 42;
pub const FILE_ACL: xmlstatus = 41;
pub const FILE_NAME: xmlstatus = 40;
pub const FILE_TYPE: xmlstatus = 39;
pub const FILE_LINK: xmlstatus = 38;
pub const FILE_INODE: xmlstatus = 37;
pub const FILE_DEVICENO: xmlstatus = 36;
pub const FILE_DEVICE_MINOR: xmlstatus = 35;
pub const FILE_DEVICE_MAJOR: xmlstatus = 34;
pub const FILE_DEVICE: xmlstatus = 33;
pub const FILE_MODE: xmlstatus = 32;
pub const FILE_UID: xmlstatus = 31;
pub const FILE_USER: xmlstatus = 30;
pub const FILE_GID: xmlstatus = 29;
pub const FILE_GROUP: xmlstatus = 28;
pub const FILE_ATIME: xmlstatus = 27;
pub const FILE_MTIME: xmlstatus = 26;
pub const FILE_CTIME: xmlstatus = 25;
pub const FILE_EA_FSTYPE: xmlstatus = 24;
pub const FILE_EA_NAME: xmlstatus = 23;
pub const FILE_EA_E_CHECKSUM: xmlstatus = 22;
pub const FILE_EA_A_CHECKSUM: xmlstatus = 21;
pub const FILE_EA_ENCODING: xmlstatus = 20;
pub const FILE_EA_SIZE: xmlstatus = 19;
pub const FILE_EA_OFFSET: xmlstatus = 18;
pub const FILE_EA_LENGTH: xmlstatus = 17;
pub const FILE_EA: xmlstatus = 16;
pub const FILE_DATA_CONTENT: xmlstatus = 15;
pub const FILE_DATA_E_CHECKSUM: xmlstatus = 14;
pub const FILE_DATA_A_CHECKSUM: xmlstatus = 13;
pub const FILE_DATA_ENCODING: xmlstatus = 12;
pub const FILE_DATA_SIZE: xmlstatus = 11;
pub const FILE_DATA_OFFSET: xmlstatus = 10;
pub const FILE_DATA_LENGTH: xmlstatus = 9;
pub const FILE_DATA: xmlstatus = 8;
pub const TOC_FILE: xmlstatus = 7;
pub const TOC_CHECKSUM_SIZE: xmlstatus = 6;
pub const TOC_CHECKSUM_OFFSET: xmlstatus = 5;
pub const TOC_CHECKSUM: xmlstatus = 4;
pub const TOC_CREATION_TIME: xmlstatus = 3;
pub const TOC: xmlstatus = 2;
pub const XAR: xmlstatus = 1;
pub const INIT: xmlstatus = 0;
pub type lzma_vli = uint64_t;
pub type lzma_delta_type = libc::c_uint;
pub const LZMA_DELTA_TYPE_BYTE: lzma_delta_type = 0;
pub type evp_pkey_ctx_st = libc::c_void;
