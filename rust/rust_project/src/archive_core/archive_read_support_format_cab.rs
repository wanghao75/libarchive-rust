use archive_core::archive_endian::*;
use rust_ffi::archive_set_error_safe;
use rust_ffi::ffi_alias::alias_set::*;
use rust_ffi::ffi_defined_param::defined_param_get::*;
use rust_ffi::ffi_method::method_call::*;
use rust_ffi::ffi_struct::struct_transfer::*;

use super::archive_string::archive_string_default_conversion_for_read;

extern "C" {
    fn inflateReset(strm: z_streamp) -> i32;

    fn inflate(strm: z_streamp, flush: i32) -> i32;

    fn inflateEnd(strm: z_streamp) -> i32;
}

pub fn inflateReset_cab_safe(strm: z_streamp) -> i32 {
    return unsafe { inflateReset(strm) };
}

pub fn inflate_cab_safe(strm: z_streamp, flush: i32) -> i32 {
    return unsafe { inflate(strm, flush) };
}

pub fn inflateEnd_cab_safe(strm: z_streamp) -> i32 {
    return unsafe { inflateEnd(strm) };
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cab {
    pub entry_offset: int64_t,
    pub entry_bytes_remaining: int64_t,
    pub entry_unconsumed: int64_t,
    pub entry_compressed_bytes_read: int64_t,
    pub entry_uncompressed_bytes_read: int64_t,
    pub entry_cffolder: *mut cffolder,
    pub entry_cffile: *mut cffile,
    pub entry_cfdata: *mut cfdata,
    pub cab_offset: int64_t,
    pub cfheader: cfheader,
    pub ws: archive_wstring,
    pub found_header: i8,
    pub end_of_archive: i8,
    pub end_of_entry: i8,
    pub end_of_entry_cleanup: i8,
    pub read_data_invoked: i8,
    pub bytes_skipped: int64_t,
    pub uncompressed_buffer: *mut u8,
    pub uncompressed_buffer_size: size_t,
    pub init_default_conversion: i32,
    pub sconv: *mut archive_string_conv,
    pub sconv_default: *mut archive_string_conv,
    pub sconv_utf8: *mut archive_string_conv,
    pub format_name: [i8; 64],
    pub xstrm: lzx_stream,
    #[cfg(HAVE_ZLIB_H)]
    pub stream: z_stream,
    pub stream_valid: i8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzx_stream {
    pub next_in: *const u8,
    pub avail_in: int64_t,
    pub total_in: int64_t,
    pub next_out: *mut u8,
    pub avail_out: int64_t,
    pub total_out: int64_t,
    pub ds: *mut lzx_dec,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzx_dec {
    pub state: i32,
    pub w_size: i32,
    pub w_mask: i32,
    pub w_buff: *mut u8,
    pub w_pos: i32,
    pub copy_pos: i32,
    pub copy_len: i32,
    pub translation_size: uint32_t,
    pub translation: i8,
    pub block_type: i8,
    pub block_size: size_t,
    pub block_bytes_avail: size_t,
    pub r0: i32,
    pub r1: i32,
    pub r2: i32,
    pub rbytes: [u8; 4],
    pub rbytes_avail: i32,
    pub length_header: i32,
    pub position_slot: i32,
    pub offset_bits: i32,
    pub pos_tbl: *mut lzx_pos_tbl,
    pub br: lzx_br,
    pub at: huffman,
    pub lt: huffman,
    pub mt: huffman,
    pub pt: huffman,
    pub loop_0: i32,
    pub error: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffman {
    pub len_size: i32,
    pub freq: [i32; 17],
    pub bitlen: *mut u8,
    pub max_bits: i32,
    pub tbl_bits: i32,
    pub tree_used: i32,
    pub tbl: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzx_br {
    pub cache_buffer: uint64_t,
    pub cache_avail: i32,
    pub odd: u8,
    pub have_odd: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzx_pos_tbl {
    pub base: i32,
    pub footer_bits: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfheader {
    pub total_bytes: uint32_t,
    pub files_offset: uint32_t,
    pub folder_count: uint16_t,
    pub file_count: uint16_t,
    pub flags: uint16_t,
    pub setid: uint16_t,
    pub cabinet: uint16_t,
    pub major: u8,
    pub minor: u8,
    pub cffolder: u8,
    pub cfdata: u8,
    pub folder_array: *mut cffolder,
    pub file_array: *mut cffile,
    pub file_index: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cffile {
    pub uncompressed_size: uint32_t,
    pub offset: uint32_t,
    pub mtime: time_t,
    pub folder: uint16_t,
    pub attr: u8,
    pub pathname: archive_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cffolder {
    pub cfdata_offset_in_cab: uint32_t,
    pub cfdata_count: uint16_t,
    pub comptype: uint16_t,
    pub compdata: uint16_t,
    pub compname: *const i8,
    pub cfdata: cfdata,
    pub cfdata_index: i32,
    pub decompress_init: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfdata {
    pub sum: uint32_t,
    pub compressed_size: uint16_t,
    pub compressed_bytes_remaining: uint16_t,
    pub uncompressed_size: uint16_t,
    pub uncompressed_bytes_remaining: uint16_t,
    pub uncompressed_avail: uint16_t,
    pub read_offset: uint16_t,
    pub unconsumed: int64_t,
    pub memimage_size: size_t,
    pub memimage: *mut u8,
    pub sum_calculated: uint32_t,
    pub sum_extra: [u8; 4],
    pub sum_extra_avail: i32,
    pub sum_ptr: *const (),
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut i8,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: i32,
    pub adler: uLong,
    pub reserved: uLong,
}

#[repr(C)]
pub struct internal_state {
    pub dummy: i32,
}

static mut slots: [i32; 11] = [
    30 as i32,
    32 as i32,
    34 as i32,
    36 as i32,
    38 as i32,
    42 as i32,
    50 as i32,
    66 as i32,
    98 as i32,
    162 as i32,
    290 as i32,
];
static mut compression_name: [*const i8; 4] = [
    b"NONE\x00" as *const u8 as *const i8,
    b"MSZIP\x00" as *const u8 as *const i8,
    b"Quantum\x00" as *const u8 as *const i8,
    b"LZX\x00" as *const u8 as *const i8,
];
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_cab(mut _a: *mut archive) -> i32 {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut cab: *mut cab = 0 as *mut cab;
    let mut r: i32 = 0;
    let mut magic_test: i32 = __archive_check_magic_safe(
        _a,
        0xdeb0c5 as u32,
        1 as u32,
        b"archive_read_support_format_cab\x00" as *const u8 as *const i8,
    );
    if magic_test == -(30 as i32) {
        return -(30 as i32);
    }
    cab = calloc_safe(
        1 as i32 as u64,
        ::std::mem::size_of::<cab>() as u64,
    ) as *mut cab;
    let a_safe = unsafe { &mut *a };
    if cab.is_null() {
        archive_set_error_safe!(
            &mut a_safe.archive as *mut archive,
            ARCHIVE_CAB_DEFINED_PARAM.enomem,
            b"Can\'t allocate CAB data\x00" as *const u8 as *const i8
        );
        return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
    }
    let cab_safe = unsafe { &mut *cab };
    cab_safe.ws.s = 0 as *mut wchar_t;
    cab_safe.ws.length = 0 as i32 as size_t;
    cab_safe.ws.buffer_length = 0 as i32 as size_t;
    archive_wstring_ensure_safe(&mut cab_safe.ws, 256 as i32 as size_t);
    r = __archive_read_register_format_safe(
        a,
        cab as *mut (),
        b"cab\x00" as *const u8 as *const i8,
        Some(archive_read_format_cab_bid),
        Some(archive_read_format_cab_options),
        Some(archive_read_format_cab_read_header),
        Some(archive_read_format_cab_read_data),
        Some(archive_read_format_cab_read_data_skip),
        None,
        Some(archive_read_format_cab_cleanup),
        None,
        None,
    );
    if r != ARCHIVE_CAB_DEFINED_PARAM.archive_ok {
        free_safe(cab as *mut ());
    }
    return ARCHIVE_CAB_DEFINED_PARAM.archive_ok;
}
unsafe fn find_cab_magic(mut p: *const i8) -> i32 {
    match unsafe { *p.offset(4 as i32 as isize) as i32 } {
        0 => {
            /*
             * Note: Self-Extraction program has 'MSCF' string in their
             * program. If we were finding 'MSCF' string only, we got
             * wrong place for Cabinet header, thus, we have to check
             * following four bytes which are reserved and must be set
             * to zero.
             */
            if memcmp_safe(
                p as *const (),
                b"MSCF\x00\x00\x00\x00\x00" as *const u8 as *const i8
                    as *const (),
                8 as i32 as u64,
            ) == 0 as i32
            {
                return 0 as i32;
            }
            return 5 as i32;
        }
        70 => return 1 as i32,
        67 => return 2 as i32,
        83 => return 3 as i32,
        77 => return 4 as i32,
        _ => return 5 as i32,
    };
}
unsafe fn archive_read_format_cab_bid(
    mut a: *mut archive_read,
    mut best_bid: i32,
) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut bytes_avail: ssize_t = 0;
    let mut offset: ssize_t = 0;
    let mut window: ssize_t = 0;
    /* If there's already a better bid than we can ever
    make, don't bother testing. */
    if best_bid > 64 as i32 {
        return -(1 as i32);
    }
    p = __archive_read_ahead_safe(a, 8 as i32 as size_t, 0 as *mut ssize_t)
        as *const i8;
    if p.is_null() {
        return -(1 as i32);
    }
    if memcmp_safe(
        p as *const (),
        b"MSCF\x00\x00\x00\x00\x00" as *const u8 as *const i8 as *const (),
        8 as i32 as u64,
    ) == 0 as i32
    {
        return 64 as i32;
    }
    /*
     * Attempt to handle self-extracting archives
     * by noting a PE header and searching forward
     * up to 128k for a 'MSCF' marker.
     */
    if unsafe {
        *p.offset(0 as i32 as isize) as i32 == 'M' as i32
            && *p.offset(1 as i32 as isize) as i32 == 'Z' as i32
    } {
        offset = 0 as i32 as ssize_t;
        window = 4096 as i32 as ssize_t;
        while offset < (1024 as i32 * 128 as i32) as i64 {
            let mut h: *const i8 =
                __archive_read_ahead_safe(a, (offset + window) as size_t, &mut bytes_avail)
                    as *const i8;
            if h.is_null() {
                /* Remaining bytes are less than window. */
                window >>= 1 as i32;
                if window < 128 as i32 as i64 {
                    return 0 as i32;
                }
            } else {
                p = unsafe { h.offset(offset as isize) };
                while unsafe {
                    p.offset(8 as i32 as isize) < h.offset(bytes_avail as isize)
                } {
                    let mut next: i32 = 0;
                    next = find_cab_magic(p);
                    if next == 0 as i32 {
                        return 64 as i32;
                    }
                    p = unsafe { p.offset(next as isize) }
                }
                offset = unsafe { p.offset_from(h) as i64 }
            }
        }
    }
    return 0 as i32;
}
unsafe fn archive_read_format_cab_options(
    mut a: *mut archive_read,
    mut key: *const i8,
    mut val: *const i8,
) -> i32 {
    let mut cab: *mut cab = 0 as *mut cab;
    let mut ret: i32 = -(25 as i32);
    let a_safe;
    let cab_safe;
    unsafe {
        cab = (*(*a).format).data as *mut cab;
        a_safe = &mut *a;
        cab_safe = &mut *cab;
    }
    if strcmp_safe(key, b"hdrcharset\x00" as *const u8 as *const i8) == 0 as i32 {
        if unsafe {
            val.is_null()
                || *val.offset(0 as i32 as isize) as i32 == 0 as i32
        } {
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                ARCHIVE_CAB_DEFINED_PARAM.archive_errno_misc,
                b"cab: hdrcharset option needs a character-set name\x00" as *const u8
                    as *const i8
            );
        } else {
            cab_safe.sconv = archive_string_conversion_from_charset_safe(
                &mut a_safe.archive,
                val,
                0 as i32,
            );
            if !cab_safe.sconv.is_null() {
                ret = ARCHIVE_CAB_DEFINED_PARAM.archive_ok
            } else {
                ret = ARCHIVE_CAB_DEFINED_PARAM.archive_fatal
            }
        }
        return ret;
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return ARCHIVE_CAB_DEFINED_PARAM.archive_warn;
}
unsafe fn cab_skip_sfx(mut a: *mut archive_read) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut q: *const i8 = 0 as *const i8;
    let mut skip: size_t = 0;
    let mut bytes: ssize_t = 0;
    let mut window: ssize_t = 0;
    window = 4096 as i32 as ssize_t;
    loop {
        let mut h: *const i8 =
            __archive_read_ahead_safe(a, window as size_t, &mut bytes) as *const i8;
        if h.is_null() {
            /* Remaining size are less than window. */
            window >>= 1 as i32;
            let a_safe = unsafe { &mut *a };
            if window < 128 as i32 as i64 {
                archive_set_error_safe!(
                    &mut a_safe.archive as *mut archive,
                    ARCHIVE_CAB_DEFINED_PARAM.archive_errno_file_format,
                    b"Couldn\'t find out CAB header\x00" as *const u8 as *const i8
                );
                return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
            }
        } else {
            p = h;
            q = unsafe { p.offset(bytes as isize) };
            /*
             * Scan ahead until we find something that looks
             * like the cab header.
             */
            while unsafe { p.offset(8 as i32 as isize) < q } {
                let mut next: i32 = 0; /* invalid */
                next = find_cab_magic(p);
                if next == 0 as i32 {
                    skip = unsafe { p.offset_from(h) as i64 as size_t };
                    __archive_read_consume_safe(a, skip as int64_t);
                    return ARCHIVE_CAB_DEFINED_PARAM.archive_ok;
                }
                p = unsafe { p.offset(next as isize) }
            }
            skip = unsafe { p.offset_from(h) as i64 as size_t };
            __archive_read_consume_safe(a, skip as int64_t);
        }
    }
}
unsafe fn truncated_error(mut a: *mut archive_read) -> i32 {
    let a_safe = unsafe { &mut *a };
    archive_set_error_safe!(
        &mut a_safe.archive as *mut archive,
        ARCHIVE_CAB_DEFINED_PARAM.archive_errno_file_format,
        b"Truncated CAB header\x00" as *const u8 as *const i8
    );
    return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
}
unsafe fn cab_strnlen(mut p: *const u8, mut maxlen: size_t) -> ssize_t {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i <= maxlen {
        if unsafe { *p.offset(i as isize) as i32 == 0 as i32 } {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i > maxlen {
        return -(1 as i32) as ssize_t;
    }
    return i as ssize_t;
}
/* Read bytes as much as remaining. */
unsafe fn cab_read_ahead_remaining(
    mut a: *mut archive_read,
    mut min: size_t,
    mut avail: *mut ssize_t,
) -> *const () {
    let mut p: *const () = 0 as *const ();
    while min > 0 as i32 as u64 {
        p = __archive_read_ahead_safe(a, min, avail);
        if p != 0 as *mut () {
            return p;
        }
        min = min.wrapping_sub(1)
    }
    return 0 as *const ();
}
/* Convert a path separator '\' -> '/' */
unsafe fn cab_convert_path_separator_1(
    mut fn_0: *mut archive_string,
    mut attr: u8,
) -> i32 {
    let mut i: size_t = 0;
    let mut mb: i32 = 0;
    /* Easy check if we have '\' in multi-byte string. */
    mb = 0 as i32;
    i = 0 as i32 as size_t;
    let fn_0_safe = unsafe { &mut *fn_0 };
    while i < fn_0_safe.length {
        if unsafe { *(*fn_0).s.offset(i as isize) as i32 == '\\' as i32 } {
            if mb != 0 {
                break;
            }
            unsafe { *(*fn_0).s.offset(i as isize) = '/' as i32 as i8 };
            mb = 0 as i32
        } else if unsafe {
            *(*fn_0).s.offset(i as isize) as i32 & 0x80 as i32 != 0
                && attr as i32 & ARCHIVE_CAB_DEFINED_PARAM.attr_name_is_utf == 0
        } {
            mb = 1 as i32
        } else {
            mb = 0 as i32
        }
        i = i.wrapping_add(1)
    }
    if i == fn_0_safe.length {
        return 0 as i32;
    }
    return -(1 as i32);
}
/*
 * Replace a character '\' with '/' in wide character.
 */
unsafe fn cab_convert_path_separator_2(
    mut cab: *mut cab,
    mut entry: *mut archive_entry,
) {
    let mut wp: *const wchar_t = 0 as *const wchar_t;
    let mut i: size_t = 0;
    /* If a conversion to wide character failed, force the replacement. */
    wp = archive_entry_pathname_w_safe(entry);
    let cab_safe = unsafe { &mut *cab };
    if !wp.is_null() {
        cab_safe.ws.length = 0 as i32 as size_t;
        archive_wstrncat_safe(
            &mut cab_safe.ws,
            wp,
            (if wp.is_null() {
                0 as i32 as u64
            } else {
                wcslen_safe(wp)
            }),
        );
        i = 0 as i32 as size_t;
        while i < cab_safe.ws.length {
            unsafe {
                if *(*cab).ws.s.offset(i as isize) == '\\' as wchar_t {
                    *(*cab).ws.s.offset(i as isize) = '/' as wchar_t
                }
            }
            i = i.wrapping_add(1)
        }
        archive_entry_copy_pathname_w_safe(entry, cab_safe.ws.s);
    };
}
/*
 * Read CFHEADER, CFFOLDER and CFFILE.
 */
unsafe fn cab_read_header(mut a: *mut archive_read) -> i32 {
    let mut current_block: u64;
    let mut p: *const u8 = 0 as *const u8;
    let mut cab: *mut cab = 0 as *mut cab;
    let mut hd: *mut cfheader = 0 as *mut cfheader;
    let mut bytes: size_t = 0;
    let mut used: size_t = 0;
    let mut len: ssize_t = 0;
    let mut skip: int64_t = 0;
    let mut err: i32 = 0;
    let mut i: i32 = 0;
    let mut cur_folder: i32 = 0;
    let mut prev_folder: i32 = 0;
    let mut offset32: uint32_t = 0;
    let a_safe = unsafe { &mut *a };
    a_safe.archive.archive_format = ARCHIVE_CAB_DEFINED_PARAM.archive_format_cab;
    if a_safe.archive.archive_format_name.is_null() {
        a_safe.archive.archive_format_name = b"CAB\x00" as *const u8 as *const i8
    }
    p = __archive_read_ahead_safe(a, 42 as i32 as size_t, 0 as *mut ssize_t)
        as *const u8;
    if p.is_null() {
        return truncated_error(a);
    }
    cab = unsafe { (*(*a).format).data as *mut cab };
    if unsafe {
        (*cab).found_header as i32 == 0 as i32
            && *p.offset(0 as i32 as isize) as i32 == 'M' as i32
            && *p.offset(1 as i32 as isize) as i32 == 'Z' as i32
    } {
        /* This is an executable?  Must be self-extracting... */
        err = cab_skip_sfx(a);
        if err < ARCHIVE_CAB_DEFINED_PARAM.archive_warn {
            return err;
        }
        /* Re-read header after processing the SFX. */
        p = __archive_read_ahead_safe(a, 42 as i32 as size_t, 0 as *mut ssize_t)
            as *const u8;
        if p.is_null() {
            return truncated_error(a);
        }
    }
    let cab_safe = unsafe { &mut *cab };
    cab_safe.cab_offset = 0 as i32 as int64_t;
    /*
     * Read CFHEADER.
     */
    hd = &mut cab_safe.cfheader; /* Avoid compiling warning. */
    if unsafe {
        *p.offset((ARCHIVE_CAB_DEFINED_PARAM.cfheader_signature + 0 as i32) as isize)
            as i32
            != 'M' as i32
            || *p.offset((ARCHIVE_CAB_DEFINED_PARAM.cfheader_signature + 1 as i32) as isize)
                as i32
                != 'S' as i32
            || *p.offset((ARCHIVE_CAB_DEFINED_PARAM.cfheader_signature + 2 as i32) as isize)
                as i32
                != 'C' as i32
            || *p.offset((ARCHIVE_CAB_DEFINED_PARAM.cfheader_signature + 3 as i32) as isize)
                as i32
                != 'F' as i32
    } {
        archive_set_error_safe!(
            &mut a_safe.archive as *mut archive,
            84 as i32,
            b"Couldn\'t find out CAB header\x00" as *const u8 as *const i8
        );
        return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
    }
    let hd_safe = unsafe { &mut *hd };
    unsafe {
        (*hd).total_bytes = archive_le32dec(
            p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_cbcabinet as isize) as *const (),
        );
        (*hd).files_offset = archive_le32dec(
            p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_cofffiles as isize) as *const (),
        );
        (*hd).minor = *p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_versionminor as isize);
        (*hd).major = *p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_versionmajor as isize);
        (*hd).folder_count = archive_le16dec(
            p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_cfolders as isize) as *const (),
        );
    }
    if !(hd_safe.folder_count as i32 == 0 as i32) {
        unsafe {
            (*hd).file_count =
                archive_le16dec(p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_cfiles as isize)
                    as *const ())
        };
        if !(hd_safe.file_count as i32 == 0 as i32) {
            unsafe {
                (*hd).flags =
                    archive_le16dec(p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_flags as isize)
                        as *const ());
                (*hd).setid =
                    archive_le16dec(p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_setid as isize)
                        as *const ());
                (*hd).cabinet = archive_le16dec(
                    p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_icabinet as isize)
                        as *const (),
                );
            }
            used = (ARCHIVE_CAB_DEFINED_PARAM.cfheader_icabinet + 2 as i32) as size_t;
            if hd_safe.flags as i32 & ARCHIVE_CAB_DEFINED_PARAM.reserve_present != 0 {
                let mut cfheader: uint16_t = 0;
                cfheader = unsafe {
                    archive_le16dec(
                        p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_cbcfheader as isize)
                            as *const (),
                    )
                };
                if cfheader as u32 > 60000 as u32 {
                    current_block = 3979278900421119935;
                } else {
                    unsafe {
                        (*hd).cffolder =
                            *p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_cbcffolder as isize);
                        (*hd).cfdata =
                            *p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfheader_cbcfdata as isize);
                    }
                    /* abReserve */
                    used = (used as u64).wrapping_add(4 as i32 as u64)
                        as size_t as size_t; /* cbCFHeader, cbCFFolder and cbCFData */
                    used = (used as u64).wrapping_add(cfheader as u64) as size_t
                        as size_t;
                    current_block = 9007357115414505193;
                }
            } else {
                hd_safe.cffolder = 0 as i32 as u8;
                current_block = 9007357115414505193;
            }
            match current_block {
                3979278900421119935 => {}
                _ => {
                    if hd_safe.flags as i32 & ARCHIVE_CAB_DEFINED_PARAM.prev_cabinet != 0 {
                        /* How many bytes are used for szCabinetPrev. */
                        p = __archive_read_ahead_safe(
                            a,
                            used.wrapping_add(256 as i32 as u64),
                            0 as *mut ssize_t,
                        ) as *const u8;
                        if p.is_null() {
                            return truncated_error(a);
                        }
                        len = unsafe {
                            cab_strnlen(p.offset(used as isize), 255 as i32 as size_t)
                        };
                        if len <= 0 as i32 as i64 {
                            current_block = 3979278900421119935;
                        } else {
                            used = (used as u64).wrapping_add(
                                (len + 1 as i32 as i64) as u64,
                            ) as size_t as size_t;
                            /* How many bytes are used for szDiskPrev. */
                            p = __archive_read_ahead_safe(
                                a,
                                used.wrapping_add(256 as i32 as u64),
                                0 as *mut ssize_t,
                            ) as *const u8;
                            if p.is_null() {
                                return truncated_error(a);
                            }
                            len = unsafe {
                                cab_strnlen(p.offset(used as isize), 255 as i32 as size_t)
                            };
                            if len <= 0 as i32 as i64 {
                                current_block = 3979278900421119935;
                            } else {
                                used = (used as u64).wrapping_add(
                                    (len + 1 as i32 as i64) as u64,
                                ) as size_t as size_t;
                                current_block = 2989495919056355252;
                            }
                        }
                    } else {
                        current_block = 2989495919056355252;
                    }
                    match current_block {
                        3979278900421119935 => {}
                        _ => {
                            if hd_safe.flags as i32 & ARCHIVE_CAB_DEFINED_PARAM.next_cabinet
                                != 0
                            {
                                /* How many bytes are used for szCabinetNext. */
                                p = __archive_read_ahead_safe(
                                    a,
                                    used.wrapping_add(256 as i32 as u64),
                                    0 as *mut ssize_t,
                                ) as *const u8;
                                if p.is_null() {
                                    return truncated_error(a);
                                }
                                len = unsafe {
                                    cab_strnlen(
                                        p.offset(used as isize),
                                        255 as i32 as size_t,
                                    )
                                };
                                if len <= 0 as i32 as i64 {
                                    current_block = 3979278900421119935;
                                } else {
                                    used = (used as u64).wrapping_add(
                                        (len + 1 as i32 as i64) as u64,
                                    ) as size_t
                                        as size_t;
                                    /* How many bytes are used for szDiskNext. */
                                    p = __archive_read_ahead_safe(
                                        a,
                                        used.wrapping_add(256 as i32 as u64),
                                        0 as *mut ssize_t,
                                    )
                                        as *const u8;
                                    if p.is_null() {
                                        return truncated_error(a);
                                    }
                                    len = unsafe {
                                        cab_strnlen(
                                            p.offset(used as isize),
                                            255 as i32 as size_t,
                                        )
                                    };
                                    if len <= 0 as i32 as i64 {
                                        current_block = 3979278900421119935;
                                    } else {
                                        used = (used as u64).wrapping_add(
                                            (len + 1 as i32 as i64)
                                                as u64,
                                        ) as size_t
                                            as size_t;
                                        current_block = 6072622540298447352;
                                    }
                                }
                            } else {
                                current_block = 6072622540298447352;
                            }
                            match current_block {
                                3979278900421119935 => {}
                                _ => {
                                    __archive_read_consume_safe(a, used as int64_t);
                                    cab_safe.cab_offset = (cab_safe.cab_offset as u64)
                                        .wrapping_add(used)
                                        as int64_t
                                        as int64_t;
                                    used = 0 as i32 as size_t;
                                    /*
                                     * Read CFFOLDER.
                                     */
                                    hd_safe.folder_array = calloc_safe(
                                        hd_safe.folder_count as u64,
                                        ::std::mem::size_of::<cffolder>() as u64,
                                    )
                                        as *mut cffolder;
                                    if hd_safe.folder_array.is_null() {
                                        current_block = 446655935564687995;
                                    } else {
                                        bytes = 8 as i32 as size_t;
                                        if hd_safe.flags as i32
                                            & ARCHIVE_CAB_DEFINED_PARAM.reserve_present
                                            != 0
                                        {
                                            bytes = (bytes as u64)
                                                .wrapping_add(hd_safe.cffolder as u64)
                                                as size_t
                                                as size_t
                                        }
                                        bytes = (bytes as u64)
                                            .wrapping_mul(hd_safe.folder_count as u64)
                                            as size_t
                                            as size_t;
                                        p = __archive_read_ahead_safe(a, bytes, 0 as *mut ssize_t)
                                            as *const u8;
                                        if p.is_null() {
                                            return truncated_error(a);
                                        }
                                        offset32 = 0 as i32 as uint32_t;
                                        i = 0 as i32;
                                        loop {
                                            if !(i < hd_safe.folder_count as i32) {
                                                current_block = 12027283704867122503;
                                                break;
                                            }
                                            let mut folder: *mut cffolder = unsafe {
                                                &mut *(*hd).folder_array.offset(i as isize)
                                                    as *mut cffolder
                                            };
                                            unsafe {
                                                (*folder).cfdata_offset_in_cab = archive_le32dec(
                                                    p.offset(
                                                        ARCHIVE_CAB_DEFINED_PARAM
                                                            .cffolder_coffcabstart
                                                            as isize,
                                                    )
                                                        as *const (),
                                                );
                                                (*folder).cfdata_count = archive_le16dec(p.offset(
                                                    ARCHIVE_CAB_DEFINED_PARAM.cffolder_ccfdata
                                                        as isize,
                                                )
                                                    as *const ());
                                                (*folder).comptype = (archive_le16dec(p.offset(
                                                    ARCHIVE_CAB_DEFINED_PARAM.cffolder_typecompress
                                                        as isize,
                                                )
                                                    as *const ())
                                                    as i32
                                                    & 0xf as i32)
                                                    as uint16_t;
                                                (*folder).compdata = (archive_le16dec(p.offset(
                                                    ARCHIVE_CAB_DEFINED_PARAM.cffolder_typecompress
                                                        as isize,
                                                )
                                                    as *const ())
                                                    as i32
                                                    >> 8 as i32)
                                                    as uint16_t;

                                                /* Get a compression name. */
                                                if ((*folder).comptype as u64)
                                                < (::std::mem::size_of::<[*const i8; 4]>()
                                                    as u64)
                                                    .wrapping_div(::std::mem::size_of::<
                                                        *const i8,
                                                    >(
                                                    )
                                                        as u64)
                                            {
                                                (*folder).compname =unsafe{
                                                    compression_name[(*folder).comptype as usize]
                                                }
                                            } else {
                                                (*folder).compname = b"UNKNOWN\x00" as *const u8
                                                    as *const i8
                                            } /* abReserve */
                                            }
                                            p = unsafe { p.offset(8 as i32 as isize) };
                                            used = (used as u64)
                                                .wrapping_add(8 as i32 as u64)
                                                as size_t
                                                as size_t;
                                            if hd_safe.flags as i32
                                                & ARCHIVE_CAB_DEFINED_PARAM.reserve_present
                                                != 0
                                            {
                                                p = unsafe {
                                                    p.offset((*hd).cffolder as i32 as isize)
                                                };
                                                used = (used as u64)
                                                    .wrapping_add(hd_safe.cffolder as u64)
                                                    as size_t
                                                    as size_t
                                            }
                                            /*
                                             * Sanity check if each data is acceptable.
                                             */
                                            let folder_safe = unsafe { &mut *folder };
                                            if offset32 >= folder_safe.cfdata_offset_in_cab {
                                                current_block = 3979278900421119935;
                                                break;
                                            }
                                            offset32 = folder_safe.cfdata_offset_in_cab;
                                            /* Set a request to initialize zlib for the CFDATA of
                                             * this folder. */
                                            folder_safe.decompress_init =
                                                0 as i32 as i8;
                                            i += 1
                                        }
                                        match current_block {
                                            3979278900421119935 => {}
                                            _ => {
                                                __archive_read_consume_safe(a, used as int64_t);
                                                cab_safe.cab_offset = (cab_safe.cab_offset
                                                    as u64)
                                                    .wrapping_add(used)
                                                    as int64_t
                                                    as int64_t;
                                                /*
                                                 * Read CFFILE.
                                                 */
                                                /* Seek read pointer to the offset of CFFILE if needed. */
                                                skip = hd_safe.files_offset as int64_t
                                                    - cab_safe.cab_offset;
                                                if skip < 0 as i32 as i64 {
                                                    archive_set_error_safe!(
                                                        &mut (*a).archive as *mut archive,
                                                        ARCHIVE_CAB_DEFINED_PARAM
                                                            .archive_errno_misc,
                                                        b"Invalid offset of CFFILE %jd < %jd\x00"
                                                            as *const u8
                                                            as *const i8,
                                                        (*hd).files_offset as intmax_t,
                                                        (*cab).cab_offset
                                                    );
                                                    return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
                                                }
                                                if skip != 0 {
                                                    __archive_read_consume_safe(a, skip);
                                                    cab_safe.cab_offset += skip
                                                }
                                                /* Allocate memory for CFDATA */
                                                hd_safe.file_array = calloc_safe(
                                                    hd_safe.file_count as u64,
                                                    ::std::mem::size_of::<cffile>()
                                                        as u64,
                                                )
                                                    as *mut cffile;
                                                if hd_safe.file_array.is_null() {
                                                    current_block = 446655935564687995;
                                                } else {
                                                    prev_folder = -(1 as i32);
                                                    i = 0 as i32;
                                                    loop {
                                                        if !(i < hd_safe.file_count as i32)
                                                        {
                                                            current_block = 9343041660989783267;
                                                            break;
                                                        }

                                                        let mut file: *mut cffile = unsafe {
                                                            &mut *(*hd)
                                                                .file_array
                                                                .offset(i as isize)
                                                                as *mut cffile
                                                        };

                                                        let mut avail: ssize_t = 0;
                                                        p = __archive_read_ahead_safe(
                                                            a,
                                                            16 as i32 as size_t,
                                                            0 as *mut ssize_t,
                                                        )
                                                            as *const u8;
                                                        if p.is_null() {
                                                            return truncated_error(a);
                                                        }
                                                        unsafe {
                                                            (*file).uncompressed_size =
                                                                archive_le32dec(
                                                                    p.offset(
                                                                        ARCHIVE_CAB_DEFINED_PARAM
                                                                            .cffile_cbfile
                                                                            as isize,
                                                                    )
                                                                        as *const (),
                                                                );
                                                            (*file).offset = archive_le32dec(
                                                                p.offset(
                                                                    ARCHIVE_CAB_DEFINED_PARAM
                                                                        .cffile_uofffolderstart
                                                                        as isize,
                                                                )
                                                                    as *const (),
                                                            );
                                                            (*file).folder = archive_le16dec(
                                                                p.offset(
                                                                    ARCHIVE_CAB_DEFINED_PARAM
                                                                        .cffile_ifolder
                                                                        as isize,
                                                                )
                                                                    as *const (),
                                                            );
                                                            (*file).mtime = cab_dos_time(
                                                                p.offset(
                                                                    ARCHIVE_CAB_DEFINED_PARAM
                                                                        .cffile_date_time
                                                                        as isize,
                                                                ),
                                                            );
                                                            (*file).attr = archive_le16dec(
                                                                p.offset(
                                                                    ARCHIVE_CAB_DEFINED_PARAM
                                                                        .cffile_attribs
                                                                        as isize,
                                                                )
                                                                    as *const (),
                                                            )
                                                                as uint8_t;
                                                        }
                                                        __archive_read_consume_safe(
                                                            a,
                                                            16 as i32 as int64_t,
                                                        );
                                                        cab_safe.cab_offset +=
                                                            16 as i32 as i64;
                                                        p = cab_read_ahead_remaining(
                                                            a,
                                                            256 as i32 as size_t,
                                                            &mut avail,
                                                        )
                                                            as *const u8;
                                                        if p.is_null() {
                                                            return truncated_error(a);
                                                        }
                                                        len = cab_strnlen(
                                                            p,
                                                            (avail
                                                                - 1 as i32 as i64)
                                                                as size_t,
                                                        );
                                                        if len <= 0 as i32 as i64 {
                                                            current_block = 3979278900421119935;
                                                            break;
                                                        }
                                                        /* Copy a pathname.  */
                                                        let file_safe = unsafe { &mut *file };
                                                        file_safe.pathname.s =
                                                            0 as *mut i8;
                                                        file_safe.pathname.length =
                                                            0 as i32 as size_t;
                                                        file_safe.pathname.buffer_length =
                                                            0 as i32 as size_t;
                                                        file_safe.pathname.length =
                                                            0 as i32 as size_t;
                                                        archive_strncat_safe(
                                                            &mut file_safe.pathname,
                                                            p as *const (),
                                                            len as size_t,
                                                        );
                                                        __archive_read_consume_safe(
                                                            a,
                                                            len + 1 as i32 as i64,
                                                        );
                                                        cab_safe.cab_offset +=
                                                            len + 1 as i32 as i64;
                                                        /*
                                                         * Sanity check if each data is acceptable.
                                                         */
                                                        if file_safe.uncompressed_size
                                                            > 0x7fff8000 as i32
                                                                as u32
                                                        {
                                                            current_block = 3979278900421119935; /* Too large */
                                                            break; /* Too large */
                                                        }
                                                        if (file_safe.offset as int64_t
                                                            + file_safe.uncompressed_size
                                                                as int64_t)
                                                            as i64
                                                            > 0x7fff8000 as i64
                                                        {
                                                            current_block = 3979278900421119935;
                                                            break;
                                                        }
                                                        match file_safe.folder as i32 {
                                                            65534 => {
                                                                /* This must be last file in a folder. */
                                                                if i != hd_safe.file_count
                                                                    as i32
                                                                    - 1 as i32
                                                                {
                                                                    current_block =
                                                                        3979278900421119935;
                                                                    break;
                                                                }
                                                                cur_folder = hd_safe.folder_count
                                                                    as i32
                                                                    - 1 as i32;
                                                                current_block =
                                                                    17392506108461345148;
                                                            }
                                                            65535 => {
                                                                /* This must be only one file in a folder. */
                                                                if hd_safe.file_count as i32
                                                                    != 1 as i32
                                                                {
                                                                    current_block =
                                                                        3979278900421119935;
                                                                    break;
                                                                }
                                                                /* FALL THROUGH */
                                                                current_block = 6145811189024720193;
                                                            }
                                                            65533 => {
                                                                current_block = 6145811189024720193;
                                                            }
                                                            _ => {
                                                                if file_safe.folder as i32
                                                                    >= hd_safe.folder_count
                                                                        as i32
                                                                {
                                                                    current_block =
                                                                        3979278900421119935;
                                                                    break;
                                                                }
                                                                cur_folder =
                                                                    file_safe.folder as i32;
                                                                current_block =
                                                                    17392506108461345148;
                                                            }
                                                        }
                                                        match current_block {
                                                            6145811189024720193 =>
                                                            /* This must be first file in a folder. */
                                                            {
                                                                if i != 0 as i32 {
                                                                    current_block =
                                                                        3979278900421119935;
                                                                    break;
                                                                }
                                                                cur_folder = 0 as i32;
                                                                prev_folder = cur_folder;
                                                                offset32 = file_safe.offset
                                                            }
                                                            _ => {}
                                                        }
                                                        /* Dot not back track. */
                                                        if cur_folder < prev_folder {
                                                            current_block = 3979278900421119935;
                                                            break;
                                                        }
                                                        if cur_folder != prev_folder {
                                                            offset32 = 0 as i32 as uint32_t
                                                        }
                                                        prev_folder = cur_folder;
                                                        /* Make sure there are not any blanks from last file
                                                         * contents. */
                                                        if offset32 != file_safe.offset {
                                                            current_block = 3979278900421119935;
                                                            break;
                                                        }
                                                        offset32 = (offset32 as u32)
                                                            .wrapping_add(
                                                                file_safe.uncompressed_size,
                                                            )
                                                            as uint32_t
                                                            as uint32_t;
                                                        /* CFDATA is available for file contents. */
                                                        if unsafe {
                                                            (*file).uncompressed_size
                                                                > 0 as i32 as u32
                                                                && (*(*hd)
                                                                    .folder_array
                                                                    .offset(cur_folder as isize))
                                                                .cfdata_count
                                                                    as i32
                                                                    == 0 as i32
                                                        } {
                                                            current_block = 3979278900421119935;
                                                            break;
                                                        }
                                                        i += 1
                                                    }
                                                    match current_block {
                                                        3979278900421119935 => {}
                                                        _ => {
                                                            if hd_safe.cabinet as i32
                                                                != 0 as i32
                                                                || hd_safe.flags as i32
                                                                    & (ARCHIVE_CAB_DEFINED_PARAM
                                                                        .prev_cabinet
                                                                        | ARCHIVE_CAB_DEFINED_PARAM
                                                                            .next_cabinet)
                                                                    != 0
                                                            {
                                                                archive_set_error_safe!(&mut a_safe.archive
                                                                                      as
                                                                                      *mut archive,
                                                                                      ARCHIVE_CAB_DEFINED_PARAM.archive_errno_file_format,
                                                                                  b"Multivolume cabinet file is unsupported\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const i8);
                                                                return ARCHIVE_CAB_DEFINED_PARAM
                                                                    .archive_warn;
                                                            }
                                                            return 0 as i32;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    match current_block {
                                        3979278900421119935 => {}
                                        _ => {
                                            archive_set_error_safe!(
                                                &mut a_safe.archive as *mut archive,
                                                ARCHIVE_CAB_DEFINED_PARAM.enomem,
                                                b"Can\'t allocate memory for CAB data\x00"
                                                    as *const u8
                                                    as *const i8
                                            );
                                            return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    archive_set_error_safe!(
        &mut a_safe.archive as *mut archive,
        ARCHIVE_CAB_DEFINED_PARAM.archive_errno_file_format,
        b"Invalid CAB header\x00" as *const u8 as *const i8
    );
    return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
}
unsafe fn archive_read_format_cab_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> i32 {
    let mut cab: *mut cab = 0 as *mut cab;
    let mut hd: *mut cfheader = 0 as *mut cfheader;
    let mut prev_folder: *mut cffolder = 0 as *mut cffolder;
    let mut file: *mut cffile = 0 as *mut cffile;
    let mut sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut err: i32 = ARCHIVE_CAB_DEFINED_PARAM.archive_ok;
    let mut r: i32 = 0;
    cab = unsafe { (*(*a).format).data as *mut cab };
    let cab_safe = unsafe { &mut *cab };
    if cab_safe.found_header as i32 == 0 as i32 {
        err = cab_read_header(a);
        if err < ARCHIVE_CAB_DEFINED_PARAM.archive_warn {
            return err;
        }
        /* We've found the header. */
        cab_safe.found_header = 1 as i32 as i8
    }
    hd = &mut cab_safe.cfheader;
    let hd_safe = unsafe { &mut *hd };
    if hd_safe.file_index >= hd_safe.file_count as i32 {
        cab_safe.end_of_archive = 1 as i32 as i8;
        return ARCHIVE_CAB_DEFINED_PARAM.archive_eof;
    }
    let fresh0 = hd_safe.file_index;
    hd_safe.file_index = hd_safe.file_index + 1;
    file = unsafe { &mut *(*hd).file_array.offset(fresh0 as isize) as *mut cffile };
    cab_safe.end_of_entry = 0 as i32 as i8;
    cab_safe.end_of_entry_cleanup = 0 as i32 as i8;
    cab_safe.entry_compressed_bytes_read = 0 as i32 as int64_t;
    cab_safe.entry_uncompressed_bytes_read = 0 as i32 as int64_t;
    cab_safe.entry_unconsumed = 0 as i32 as int64_t;
    cab_safe.entry_cffile = file;
    /*
     * Choose a proper folder.
     */
    prev_folder = cab_safe.entry_cffolder;
    unsafe {
        match (*file).folder as i32 {
            65533 | 65535 => {
                (*cab).entry_cffolder =
                    &mut *(*hd).folder_array.offset(0 as i32 as isize) as *mut cffolder
            }
            65534 => {
                (*cab).entry_cffolder = &mut *(*hd)
                    .folder_array
                    .offset(((*hd).folder_count as i32 - 1 as i32) as isize)
                    as *mut cffolder
            }
            _ => {
                (*cab).entry_cffolder =
                    &mut *(*hd).folder_array.offset((*file).folder as isize) as *mut cffolder
            }
        }
    }
    /* If a cffolder of this file is changed, reset a cfdata to read
     * file contents from next cfdata. */
    if prev_folder != cab_safe.entry_cffolder {
        cab_safe.entry_cfdata = 0 as *mut cfdata
    }
    /* If a pathname is UTF-8, prepare a string conversion object
     * for UTF-8 and use it. */
    let file_safe = unsafe { &mut *file };
    let a_safe = unsafe { &mut *a };
    if file_safe.attr as i32 & ARCHIVE_CAB_DEFINED_PARAM.attr_name_is_utf != 0 {
        if cab_safe.sconv_utf8.is_null() {
            cab_safe.sconv_utf8 = archive_string_conversion_from_charset_safe(
                &mut a_safe.archive,
                b"UTF-8\x00" as *const u8 as *const i8,
                1 as i32,
            );
            if cab_safe.sconv_utf8.is_null() {
                return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
            }
        }
        sconv = cab_safe.sconv_utf8
    } else if !cab_safe.sconv.is_null() {
        /* Choose the conversion specified by the option. */
        sconv = cab_safe.sconv
    } else {
        /* Choose the default conversion. */
        if cab_safe.init_default_conversion == 0 {
            cab_safe.sconv_default =
                unsafe { archive_string_default_conversion_for_read(&mut a_safe.archive) };
            cab_safe.init_default_conversion = 1 as i32
        }
        sconv = cab_safe.sconv_default
    }
    /*
     * Set a default value and common data
     */
    r = cab_convert_path_separator_1(&mut file_safe.pathname, file_safe.attr);
    if _archive_entry_copy_pathname_l_safe(
        entry,
        file_safe.pathname.s,
        file_safe.pathname.length,
        sconv,
    ) != 0 as i32
    {
        if unsafe { *__errno_location() == ARCHIVE_CAB_DEFINED_PARAM.enomem } {
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                ARCHIVE_CAB_DEFINED_PARAM.enomem,
                b"Can\'t allocate memory for Pathname\x00" as *const u8 as *const i8
            );
            return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
        }
        archive_set_error_safe!(
            &mut (*a).archive as *mut archive,
            ARCHIVE_CAB_DEFINED_PARAM.archive_errno_file_format,
            b"Pathname cannot be converted from %s to current locale.\x00" as *const u8
                as *const i8,
            archive_string_conversion_charset_name(sconv)
        );
        err = ARCHIVE_CAB_DEFINED_PARAM.archive_warn
    }
    if r < 0 as i32 {
        /* Convert a path separator '\' -> '/' */
        cab_convert_path_separator_2(cab, entry);
    }
    archive_entry_set_size_safe(entry, file_safe.uncompressed_size as la_int64_t);
    if file_safe.attr as i32 & ARCHIVE_CAB_DEFINED_PARAM.attr_rdonly != 0 {
        archive_entry_set_mode_safe(
            entry,
            ARCHIVE_CAB_DEFINED_PARAM.ae_ifreg as mode_t | 0o555 as i32 as u32,
        );
    } else {
        archive_entry_set_mode_safe(
            entry,
            ARCHIVE_CAB_DEFINED_PARAM.ae_ifreg as mode_t | 0o666 as i32 as u32,
        );
    }
    archive_entry_set_mtime_safe(entry, file_safe.mtime, 0 as i32 as i64);
    cab_safe.entry_bytes_remaining = file_safe.uncompressed_size as int64_t;
    cab_safe.entry_offset = 0 as i32 as int64_t;
    /* We don't need compress data. */
    if file_safe.uncompressed_size == 0 as i32 as u32 {
        cab_safe.end_of_entry = 1 as i32 as i8;
        cab_safe.end_of_entry_cleanup = cab_safe.end_of_entry
    }
    /* Set up a more descriptive format name. */
    unsafe {
        sprintf(
            (*cab).format_name.as_mut_ptr(),
            b"CAB %d.%d (%s)\x00" as *const u8 as *const i8,
            (*hd).major as i32,
            (*hd).minor as i32,
            (*(*cab).entry_cffolder).compname,
        );
    }
    a_safe.archive.archive_format_name = cab_safe.format_name.as_mut_ptr();
    return err;
}
unsafe fn archive_read_format_cab_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const (),
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> i32 {
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let mut r: i32 = 0;
    let buff_safe;
    let size_safe;
    let offset_safe;
    let a_safe;
    unsafe {
        buff_safe = &mut *buff;
        size_safe = &mut *size;
        offset_safe = &mut *offset;
        a_safe = &mut *a;
    }
    match unsafe { (*(*cab).entry_cffile).folder as i32 } {
        65533 | 65534 | 65535 => {
            *buff_safe = 0 as *const ();
            *size_safe = 0 as i32 as size_t;
            *offset_safe = 0 as i32 as int64_t;
            archive_clear_error_safe(&mut a_safe.archive);
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                ARCHIVE_CAB_DEFINED_PARAM.archive_errno_file_format,
                b"Cannot restore this file split in multivolume.\x00" as *const u8
                    as *const i8
            );
            return ARCHIVE_CAB_DEFINED_PARAM.archive_failed;
        }
        _ => {}
    }
    let cab_safe = unsafe { &mut *cab };
    if cab_safe.read_data_invoked as i32 == 0 as i32 {
        if cab_safe.bytes_skipped != 0 {
            if cab_safe.entry_cfdata.is_null() {
                r = cab_next_cfdata(a);
                if r < 0 as i32 {
                    return r;
                }
            }
            if cab_consume_cfdata(a, cab_safe.bytes_skipped) < 0 as i32 as i64 {
                return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
            }
            cab_safe.bytes_skipped = 0 as i32 as int64_t
        }
        cab_safe.read_data_invoked = 1 as i32 as i8
    }
    if cab_safe.entry_unconsumed != 0 {
        /* Consume as much as the compressor actually used. */
        r = cab_consume_cfdata(a, cab_safe.entry_unconsumed) as i32;
        cab_safe.entry_unconsumed = 0 as i32 as int64_t;
        if r < 0 as i32 {
            return r;
        }
    }
    if cab_safe.end_of_archive as i32 != 0 || cab_safe.end_of_entry as i32 != 0 {
        if cab_safe.end_of_entry_cleanup == 0 {
            /* End-of-entry cleanup done. */
            cab_safe.end_of_entry_cleanup = 1 as i32 as i8
        }
        *offset_safe = cab_safe.entry_offset;
        *size_safe = 0 as i32 as size_t;
        *buff_safe = 0 as *const ();
        return ARCHIVE_CAB_DEFINED_PARAM.archive_eof;
    }
    return cab_read_data(a, buff, size, offset);
}
unsafe fn cab_checksum_cfdata_4(
    mut p: *const (),
    mut bytes: size_t,
    mut seed: uint32_t,
) -> uint32_t {
    let mut b: *const u8 = 0 as *const u8;
    let mut u32num: u32 = 0;
    let mut sum: uint32_t = 0;
    u32num = (bytes as u32).wrapping_div(4 as i32 as u32);
    sum = seed;
    b = p as *const u8;
    while u32num > 0 as i32 as u32 {
        sum ^= archive_le32dec(b as *const ());
        b = unsafe { b.offset(4 as i32 as isize) };
        u32num = u32num.wrapping_sub(1)
    }
    return sum;
}
unsafe fn cab_checksum_cfdata(
    mut p: *const (),
    mut bytes: size_t,
    mut seed: uint32_t,
) -> uint32_t {
    let mut b: *const u8 = 0 as *const u8;
    let mut sum: uint32_t = 0;
    let mut t: uint32_t = 0;
    sum = cab_checksum_cfdata_4(p, bytes, seed);
    b = p as *const u8;
    b = unsafe { b.offset((bytes & !(3 as i32) as u64) as isize) };
    t = 0 as i32 as uint32_t;
    let mut current_block_6: u64;
    match bytes & 3 as i32 as u64 {
        3 => {
            let fresh1 = b;
            unsafe {
                b = b.offset(1);
                t |= (*fresh1 as uint32_t) << 16 as i32;
            }
            current_block_6 = 3089271934609210602;
        }
        2 => {
            current_block_6 = 3089271934609210602;
        }
        1 => {
            current_block_6 = 1403743547856234815;
        }
        _ => {
            current_block_6 = 1917311967535052937;
        }
    }
    match current_block_6 {
        3089271934609210602 =>
        /* FALL THROUGH */
        {
            let fresh2 = b;
            unsafe {
                b = b.offset(1);
                t |= (*fresh2 as uint32_t) << 8 as i32;
            }
            current_block_6 = 1403743547856234815;
        }
        _ => {}
    }
    match current_block_6 {
        1403743547856234815 =>
        /* FALL THROUGH */
        {
            t |= unsafe { *b as u32 }
        }
        _ => {}
    }
    sum ^= t;
    return sum;
}
unsafe fn cab_checksum_update(mut a: *mut archive_read, mut bytes: size_t) {
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let mut cfdata: *mut cfdata = unsafe { (*cab).entry_cfdata };
    let mut p: *const u8 = 0 as *const u8;
    let mut sumbytes: size_t = 0;
    let cfdata_safe = unsafe { &mut *cfdata };
    if cfdata_safe.sum == 0 as i32 as u32
        || cfdata_safe.sum_ptr == 0 as *mut ()
    {
        return;
    }
    /*
     * Calculate the sum of this CFDATA.
     * Make sure CFDATA must be calculated in four bytes.
     */
    p = cfdata_safe.sum_ptr as *const u8;
    sumbytes = bytes;
    if cfdata_safe.sum_extra_avail != 0 {
        while cfdata_safe.sum_extra_avail < 4 as i32
            && sumbytes > 0 as i32 as u64
        {
            let fresh3 = p;
            p = unsafe { p.offset(1) };
            let fresh4 = cfdata_safe.sum_extra_avail;
            cfdata_safe.sum_extra_avail = cfdata_safe.sum_extra_avail + 1;
            cfdata_safe.sum_extra[fresh4 as usize] = unsafe { *fresh3 };
            sumbytes = sumbytes.wrapping_sub(1)
        }
        if cfdata_safe.sum_extra_avail == 4 as i32 {
            cfdata_safe.sum_calculated = cab_checksum_cfdata_4(
                cfdata_safe.sum_extra.as_mut_ptr() as *const (),
                4 as i32 as size_t,
                cfdata_safe.sum_calculated,
            );
            cfdata_safe.sum_extra_avail = 0 as i32
        }
    }
    if sumbytes != 0 {
        let mut odd: i32 = (sumbytes & 3 as i32 as u64) as i32;
        if sumbytes.wrapping_sub(odd as u64) > 0 as i32 as u64 {
            cfdata_safe.sum_calculated = cab_checksum_cfdata_4(
                p as *const (),
                sumbytes.wrapping_sub(odd as u64),
                cfdata_safe.sum_calculated,
            )
        }
        if odd != 0 {
            unsafe {
                memcpy_safe(
                    (*cfdata).sum_extra.as_mut_ptr() as *mut (),
                    p.offset(sumbytes as isize).offset(-(odd as isize)) as *const (),
                    odd as u64,
                );
            }
        }
        cfdata_safe.sum_extra_avail = odd
    }
    cfdata_safe.sum_ptr = 0 as *const ();
}
unsafe fn cab_checksum_finish(mut a: *mut archive_read) -> i32 {
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let cab_safe = unsafe { &mut *cab };
    let mut cfdata: *mut cfdata = cab_safe.entry_cfdata;
    let mut l: i32 = 0;
    /* Do not need to compute a sum. */
    let cfdata_safe = unsafe { &mut *cfdata };
    if cfdata_safe.sum == 0 as i32 as u32 {
        return 0 as i32;
    }
    /*
     * Calculate the sum of remaining CFDATA.
     */
    if cfdata_safe.sum_extra_avail != 0 {
        cfdata_safe.sum_calculated = cab_checksum_cfdata(
            cfdata_safe.sum_extra.as_mut_ptr() as *const (),
            cfdata_safe.sum_extra_avail as size_t,
            cfdata_safe.sum_calculated,
        );
        cfdata_safe.sum_extra_avail = 0 as i32
    }
    l = 4 as i32;
    if cab_safe.cfheader.flags as i32 & ARCHIVE_CAB_DEFINED_PARAM.reserve_present != 0 {
        l += cab_safe.cfheader.cfdata as i32
    }
    cfdata_safe.sum_calculated = unsafe {
        cab_checksum_cfdata(
            (*cfdata)
                .memimage
                .offset(ARCHIVE_CAB_DEFINED_PARAM.cfdata_cbdata as isize)
                as *const (),
            l as size_t,
            (*cfdata).sum_calculated,
        )
    };
    if cfdata_safe.sum_calculated != cfdata_safe.sum {
        archive_set_error_safe!(
            &mut (*a).archive as *mut archive,
            ARCHIVE_CAB_DEFINED_PARAM.archive_errno_file_format,
            b"Checksum error CFDATA[%d] %x:%x in %d bytes\x00" as *const u8 as *const i8,
            (*(*cab).entry_cffolder).cfdata_index - 1 as i32,
            (*cfdata).sum,
            (*cfdata).sum_calculated,
            (*cfdata).compressed_size as i32
        );
        return ARCHIVE_CAB_DEFINED_PARAM.archive_failed;
    }
    return ARCHIVE_CAB_DEFINED_PARAM.archive_ok;
}
/*
 * Read CFDATA if needed.
 */
unsafe fn cab_next_cfdata(mut a: *mut archive_read) -> i32 {
    let mut current_block: u64;
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let cab_safe = unsafe { &mut *cab };
    let mut cfdata: *mut cfdata = cab_safe.entry_cfdata;
    /* There are remaining bytes in current CFDATA, use it first. */
    let mut cfdata_safe = unsafe { &mut *cfdata };
    if !cfdata.is_null()
        && cfdata_safe.uncompressed_bytes_remaining as i32 > 0 as i32
    {
        return ARCHIVE_CAB_DEFINED_PARAM.archive_ok;
    }
    let cab_cffolder_safe = unsafe { &mut (*(*cab).entry_cffolder) };
    let cab_cffile_safe = unsafe { &mut (*(*cab).entry_cffile) };
    if cfdata.is_null() {
        let mut skip: int64_t = 0;
        cab_cffolder_safe.cfdata_index = 0 as i32;
        /* Seek read pointer to the offset of CFDATA if needed. */
        skip = cab_cffolder_safe.cfdata_offset_in_cab as i64 - cab_safe.cab_offset;
        if skip < 0 as i32 as i64 {
            let mut folder_index: i32 = 0;
            match cab_cffile_safe.folder as i32 {
                65533 | 65535 => folder_index = 0 as i32,
                65534 => {
                    folder_index = cab_safe.cfheader.folder_count as i32 - 1 as i32
                }
                _ => folder_index = cab_cffile_safe.folder as i32,
            }
            archive_set_error_safe!(
                &mut (*a).archive as *mut archive,
                ARCHIVE_CAB_DEFINED_PARAM.archive_errno_misc,
                b"Invalid offset of CFDATA in folder(%d) %jd < %jd\x00" as *const u8
                    as *const i8,
                folder_index,
                (*(*cab).entry_cffolder).cfdata_offset_in_cab as intmax_t,
                (*cab).cab_offset
            );
            return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
        }
        if skip > 0 as i32 as i64 {
            if __archive_read_consume_safe(a, skip) < 0 as i32 as i64 {
                return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
            }
            cab_safe.cab_offset = cab_cffolder_safe.cfdata_offset_in_cab as int64_t
        }
    }
    /*
     * Read a CFDATA.
     */
    if cab_cffolder_safe.cfdata_index < cab_cffolder_safe.cfdata_count as i32 {
        let mut p: *const u8 = 0 as *const u8;
        let mut l: i32 = 0;
        cfdata = &mut cab_cffolder_safe.cfdata;
        cfdata_safe = unsafe { &mut *cfdata };
        cab_cffolder_safe.cfdata_index += 1;
        cab_safe.entry_cfdata = cfdata;
        cfdata_safe.sum_calculated = 0 as i32 as uint32_t;
        cfdata_safe.sum_extra_avail = 0 as i32;
        cfdata_safe.sum_ptr = 0 as *const ();
        l = 8 as i32;
        if cab_safe.cfheader.flags as i32 & ARCHIVE_CAB_DEFINED_PARAM.reserve_present != 0 {
            l += cab_safe.cfheader.cfdata as i32
        }
        p = __archive_read_ahead_safe(a, l as size_t, 0 as *mut ssize_t) as *const u8;
        if p.is_null() {
            return truncated_error(a);
        }
        cfdata_safe.sum = unsafe {
            archive_le32dec(
                p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfdata_csum as isize) as *const ()
            )
        };
        cfdata_safe.compressed_size = unsafe {
            archive_le16dec(
                p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfdata_cbdata as isize) as *const ()
            )
        };
        cfdata_safe.compressed_bytes_remaining = cfdata_safe.compressed_size;
        cfdata_safe.uncompressed_size = unsafe {
            archive_le16dec(
                p.offset(ARCHIVE_CAB_DEFINED_PARAM.cfdata_cbuncomp as isize) as *const ()
            )
        };
        cfdata_safe.uncompressed_bytes_remaining = cfdata_safe.uncompressed_size;
        cfdata_safe.uncompressed_avail = 0 as i32 as uint16_t;
        cfdata_safe.read_offset = 0 as i32 as uint16_t;
        cfdata_safe.unconsumed = 0 as i32 as int64_t;
        /*
         * Sanity check if data size is acceptable.
         */
        let a_safe = unsafe { &mut *a };
        if cfdata_safe.compressed_size as i32 == 0 as i32
            || cfdata_safe.compressed_size as i32
                > 0x8000 as i32 + 6144 as i32
            || cfdata_safe.uncompressed_size as i32 > 0x8000 as i32
        {
            current_block = 2305958262682200376;
        } else {
            if cfdata_safe.uncompressed_size as i32 == 0 as i32 {
                match cab_cffile_safe.folder as i32 {
                    65535 | 65534 => {
                        current_block = 1434579379687443766;
                    }
                    65533 | _ => {
                        current_block = 2305958262682200376;
                    }
                }
            } else {
                current_block = 1434579379687443766;
            }
            match current_block {
                2305958262682200376 => {}
                _ =>
                /* If CFDATA is not last in a folder, an uncompressed
                 * size must be 0x8000(32KBi) */
                {
                    if (cab_cffolder_safe.cfdata_index
                        < cab_cffolder_safe.cfdata_count as i32
                        && cfdata_safe.uncompressed_size as i32 != 0x8000 as i32)
                        || (cab_cffolder_safe.comptype as i32
                            == ARCHIVE_CAB_DEFINED_PARAM.comptype_none
                            && cfdata_safe.compressed_size as i32
                                != cfdata_safe.uncompressed_size as i32)
                    {
                        current_block = 2305958262682200376;
                    } else {
                        /* A compressed data size and an uncompressed data size must
                         * be the same in no compression mode. */
                        /*
                         * Save CFDATA image for sum check.
                         */
                        if cfdata_safe.memimage_size < l as size_t {
                            free_safe(cfdata_safe.memimage as *mut ());
                            cfdata_safe.memimage =
                                malloc_safe(l as u64) as *mut u8;
                            if cfdata_safe.memimage.is_null() {
                                archive_set_error_safe!(
                                    &mut a_safe.archive as *mut archive,
                                    ARCHIVE_CAB_DEFINED_PARAM.enomem,
                                    b"Can\'t allocate memory for CAB data\x00" as *const u8
                                        as *const i8
                                );
                                return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
                            }
                            cfdata_safe.memimage_size = l as size_t
                        }
                        memcpy_safe(
                            cfdata_safe.memimage as *mut (),
                            p as *const (),
                            l as u64,
                        );
                        /* Consume bytes as much as we used. */
                        __archive_read_consume_safe(a, l as int64_t);
                        cab_safe.cab_offset += l as i64;
                        current_block = 16779030619667747692;
                    }
                }
            }
        }
        match current_block {
            16779030619667747692 => {}
            _ => {
                archive_set_error_safe!(
                    &mut a_safe.archive as *mut archive,
                    ARCHIVE_CAB_DEFINED_PARAM.archive_errno_file_format,
                    b"Invalid CFDATA\x00" as *const u8 as *const i8
                );
                return ARCHIVE_CAB_DEFINED_PARAM.archive_fatal;
            }
        }
    } else if cab_cffolder_safe.cfdata_count as i32 > 0 as i32 {
        /* Run out of all CFDATA in a folder. */
        cfdata_safe.compressed_size = 0 as i32 as uint16_t;
        cfdata_safe.uncompressed_size = 0 as i32 as uint16_t;
        cfdata_safe.compressed_bytes_remaining = 0 as i32 as uint16_t;
        cfdata_safe.uncompressed_bytes_remaining = 0 as i32 as uint16_t
    } else {
        /* Current folder does not have any CFDATA. */
        cfdata = &mut cab_cffolder_safe.cfdata;
        cab_safe.entry_cfdata = cfdata;
        memset_safe(
            cfdata as *mut (),
            0 as i32,
            ::std::mem::size_of::<cfdata>() as u64,
        );
    }
    return ARCHIVE_CAB_DEFINED_PARAM.archive_ok;
}
/*
 * Read ahead CFDATA.
 */
unsafe fn cab_read_ahead_cfdata(
    mut a: *mut archive_read,
    mut avail: *mut ssize_t,
) -> *const () {
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let mut err: i32 = 0;
    err = cab_next_cfdata(a);
    let avail_safe = unsafe { &mut *avail };
    if err < ARCHIVE_CAB_DEFINED_PARAM.archive_ok {
        *avail_safe = err as ssize_t;
        return 0 as *const ();
    }
    let cab_cffolder_safe = unsafe { &mut (*(*cab).entry_cffolder) };
    match cab_cffolder_safe.comptype as i32 {
        0 => return cab_read_ahead_cfdata_none(a, avail),
        1 => return cab_read_ahead_cfdata_deflate(a, avail),
        3 => return cab_read_ahead_cfdata_lzx(a, avail),
        _ => {
            /* Unsupported compression. */
            archive_set_error_safe!(
                &mut (*a).archive as *mut archive,
                ARCHIVE_CAB_DEFINED_PARAM.archive_errno_file_format,
                b"Unsupported CAB compression : %s\x00" as *const u8 as *const i8,
                cab_cffolder_safe.compname
            );
            *avail_safe = ARCHIVE_CAB_DEFINED_PARAM.archive_failed as ssize_t;
            return 0 as *const ();
        }
    };
}
/*
 * Read ahead CFDATA as uncompressed data.
 */
unsafe fn cab_read_ahead_cfdata_none(
    mut a: *mut archive_read,
    mut avail: *mut ssize_t,
) -> *const () {
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    let mut d: *const () = 0 as *const ();
    cfdata = unsafe { (*cab).entry_cfdata };
    /*
     * Note: '1' here is a performance optimization.
     * Recall that the decompression layer returns a count of
     * available bytes; asking for more than that forces the
     * decompressor to combine reads by copying data.
     */
    d = __archive_read_ahead_safe(a, 1 as i32 as size_t, avail);
    let cfdata_safe = unsafe { &mut *cfdata };
    let avail_safe = unsafe { &mut *avail };
    if *avail_safe <= 0 as i32 as i64 {
        *avail_safe = truncated_error(a) as ssize_t;
        return 0 as *const ();
    }
    if *avail_safe > cfdata_safe.uncompressed_bytes_remaining as i64 {
        *avail_safe = cfdata_safe.uncompressed_bytes_remaining as ssize_t
    }
    cfdata_safe.uncompressed_avail = cfdata_safe.uncompressed_size;
    cfdata_safe.unconsumed = *avail_safe;
    cfdata_safe.sum_ptr = d;
    return d;
}
/*
 * Read ahead CFDATA as deflate data.
 */

/* HAVE_ZLIB_H */
#[cfg(not(HAVE_ZLIB_H))]
unsafe fn cab_read_ahead_cfdata_deflate(
    mut a: *mut archive_read,
    mut avail: *mut ssize_t,
) -> *const () {
    let avail_safe = unsafe { &mut *avail };
    let a_safe = unsafe { &mut *a };
    *avail_safe = ARCHIVE_CAB_DEFINED_PARAM.archive_fatal as ssize_t;
    archive_set_error_safe!(
        &mut a_safe.archive as *mut archive,
        ARCHIVE_CAB_DEFINED_PARAM.archive_errno_misc,
        b"libarchive compiled without deflate support (no libz)\x00" as *const u8
            as *const i8
    );
    return 0 as *const ();
}

/* HAVE_ZLIB_H */

#[cfg(HAVE_ZLIB_H)]
unsafe fn cab_read_ahead_cfdata_deflate(
    mut a: *mut archive_read,
    mut avail: *mut ssize_t,
) -> *const () {
    let mut current_block: u64;
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    let mut d: *const () = 0 as *const ();
    let mut r: i32 = 0;
    let mut mszip: i32 = 0;
    let mut uavail: uint16_t = 0;
    let mut eod: i8 = 0 as i32 as i8;
    let cab_safe = unsafe { &mut *cab };
    cfdata = cab_safe.entry_cfdata;
    /* If the buffer hasn't been allocated, allocate it now. */
    let a_safe = unsafe { &mut *a };
    let avail_safe = unsafe { &mut *avail };
    if cab_safe.uncompressed_buffer.is_null() {
        cab_safe.uncompressed_buffer_size = 0x8000 as i32 as size_t;
        cab_safe.uncompressed_buffer =
            malloc_safe(cab_safe.uncompressed_buffer_size) as *mut u8;
        if cab_safe.uncompressed_buffer.is_null() {
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                ARCHIVE_CAB_DEFINED_PARAM.enomem,
                b"No memory for CAB reader\x00" as *const u8 as *const i8
            );
            *avail_safe = ARCHIVE_CAB_DEFINED_PARAM.archive_fatal as ssize_t;
            return 0 as *const ();
        }
    }
    let cfdata_safe = unsafe { &mut *cfdata };
    uavail = cfdata_safe.uncompressed_avail;
    if uavail as i32 == cfdata_safe.uncompressed_size as i32 {
        d = unsafe {
            (*cab)
                .uncompressed_buffer
                .offset((*cfdata).read_offset as i32 as isize)
                as *const ()
        };
        *avail_safe = (uavail as i32 - cfdata_safe.read_offset as i32) as ssize_t;
        return d;
    }
    let cab_cffolder_safe = unsafe { &mut (*(*cab).entry_cffolder) };
    let a_safe = unsafe { &mut *a };
    if cab_cffolder_safe.decompress_init == 0 {
        cab_safe.stream.next_in = 0 as *mut Bytef;
        cab_safe.stream.avail_in = 0 as i32 as uInt;
        cab_safe.stream.total_in = 0 as i32 as uLong;
        cab_safe.stream.next_out = 0 as *mut Bytef;
        cab_safe.stream.avail_out = 0 as i32 as uInt;
        cab_safe.stream.total_out = 0 as i32 as uLong;
        if cab_safe.stream_valid != 0 {
            r = inflateReset_cab_safe(&mut cab_safe.stream)
        } else {
            r = inflateInit2__safe(
                &mut cab_safe.stream,
                -(15 as i32),
                b"1.2.3\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<z_stream>() as u64 as i32,
            )
        }
        /* Don't check for zlib header */
        if r != 0 as i32 {
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                ARCHIVE_CAB_DEFINED_PARAM.archive_errno_misc,
                b"Can\'t initialize deflate decompression.\x00" as *const u8 as *const i8
            );
            *avail_safe = ARCHIVE_CAB_DEFINED_PARAM.archive_fatal as ssize_t;
            return 0 as *const ();
        }
        /* Stream structure has been set up. */
        cab_safe.stream_valid = 1 as i32 as i8;
        /* We've initialized decompression for this stream. */
        cab_cffolder_safe.decompress_init = 1 as i32 as i8
    }
    if cfdata_safe.compressed_bytes_remaining as i32
        == cfdata_safe.compressed_size as i32
    {
        mszip = 2 as i32
    } else {
        mszip = 0 as i32
    }
    eod = 0 as i32 as i8;
    cab_safe.stream.total_out = uavail as uLong;
    loop
    /*
     * We always uncompress all data in current CFDATA.
     */
    {
        if !(eod == 0 && cab_safe.stream.total_out < cfdata_safe.uncompressed_size as u64)
        {
            current_block = 10778260831612459202;
            break;
        }
        let mut bytes_avail: ssize_t = 0;
        cab_safe.stream.next_out = unsafe {
            (*cab)
                .uncompressed_buffer
                .offset((*cab).stream.total_out as isize)
        };
        cab_safe.stream.avail_out = (cfdata_safe.uncompressed_size as u64)
            .wrapping_sub(cab_safe.stream.total_out) as uInt;
        d = __archive_read_ahead_safe(a, 1 as i32 as size_t, &mut bytes_avail);
        if bytes_avail <= 0 as i32 as i64 {
            *avail_safe = truncated_error(a) as ssize_t;
            return 0 as *const ();
        }
        if bytes_avail > cfdata_safe.compressed_bytes_remaining as i64 {
            bytes_avail = cfdata_safe.compressed_bytes_remaining as ssize_t
        }
        /*
         * A bug in zlib.h: stream.next_in should be marked 'const'
         * but isn't (the library never alters data through the
         * next_in pointer, only reads it).  The result: this ugly
         * cast to remove 'const'.
         */
        cab_safe.stream.next_in = d as uintptr_t as *mut Bytef;
        cab_safe.stream.avail_in = bytes_avail as uInt;
        cab_safe.stream.total_in = 0 as i32 as uLong;
        /* Cut out a tow-byte MSZIP signature(0x43, 0x4b). */
        if mszip > 0 as i32 {
            if bytes_avail <= 0 as i32 as i64 {
                current_block = 4648980483242066537;
                break;
            }
            if bytes_avail <= mszip as i64 {
                if mszip == 2 as i32 {
                    if unsafe {
                        *(*cab).stream.next_in.offset(0 as i32 as isize) as i32
                            != 0x43 as i32
                    } {
                        current_block = 4648980483242066537;
                        break;
                    }
                    if unsafe {
                        bytes_avail > 1 as i32 as i64
                            && *(*cab).stream.next_in.offset(1 as i32 as isize)
                                as i32
                                != 0x4b as i32
                    } {
                        current_block = 4648980483242066537;
                        break;
                    }
                } else if unsafe {
                    *(*cab).stream.next_in.offset(0 as i32 as isize) as i32
                        != 0x4b as i32
                } {
                    current_block = 4648980483242066537;
                    break;
                }
                cfdata_safe.unconsumed = bytes_avail;
                cfdata_safe.sum_ptr = d;
                if cab_minimum_consume_cfdata(a, cfdata_safe.unconsumed)
                    < 0 as i32 as i64
                {
                    *avail_safe = ARCHIVE_CAB_DEFINED_PARAM.archive_fatal as ssize_t;
                    return 0 as *const ();
                }
                mszip -= bytes_avail as i32;
                continue;
            } else {
                if unsafe {
                    mszip == 1 as i32
                        && *(*cab).stream.next_in.offset(0 as i32 as isize) as i32
                            != 0x4b as i32
                } {
                    current_block = 4648980483242066537;
                    break;
                }
                if unsafe {
                    mszip == 2 as i32
                        && (*(*cab).stream.next_in.offset(0 as i32 as isize) as i32
                            != 0x43 as i32
                            || *(*cab).stream.next_in.offset(1 as i32 as isize)
                                as i32
                                != 0x4b as i32)
                } {
                    current_block = 4648980483242066537;
                    break;
                }
                cab_safe.stream.next_in = unsafe { cab_safe.stream.next_in.offset(mszip as isize) };
                cab_safe.stream.avail_in = (cab_safe.stream.avail_in as u32)
                    .wrapping_sub(mszip as u32)
                    as uInt as uInt;
                cab_safe.stream.total_in = (cab_safe.stream.total_in as u64)
                    .wrapping_add(mszip as u64)
                    as uLong as uLong;
                mszip = 0 as i32
            }
        }
        r = inflate_cab_safe(&mut cab_safe.stream, 0 as i32);
        match r {
            0 => {}
            1 => eod = 1 as i32 as i8,
            _ => {
                current_block = 12144037074258575129;
                break;
            }
        }
        cfdata_safe.unconsumed = cab_safe.stream.total_in as int64_t;
        cfdata_safe.sum_ptr = d;
        if cab_minimum_consume_cfdata(a, cfdata_safe.unconsumed) < 0 as i32 as i64
        {
            *avail_safe = ARCHIVE_CAB_DEFINED_PARAM.archive_fatal as ssize_t;
            return 0 as *const ();
        }
    }
    match current_block {
        10778260831612459202 => {
            uavail = cab_safe.stream.total_out as uint16_t;
            if (uavail as i32) < cfdata_safe.uncompressed_size as i32 {
                archive_set_error_safe!(
                    &mut (*a).archive as *mut archive,
                    -(1 as i32),
                    b"Invalid uncompressed size (%d < %d)\x00" as *const u8 as *const i8,
                    uavail as i32,
                    (*cfdata).uncompressed_size as i32
                );
                *avail_safe = ARCHIVE_CAB_DEFINED_PARAM.archive_fatal as ssize_t;
                return 0 as *const ();
            }
            /*
             * Note: I suspect there is a bug in makecab.exe because, in rare
             * case, compressed bytes are still remaining regardless we have
             * gotten all uncompressed bytes, which size is recorded in CFDATA,
             * as much as we need, and we have to use the garbage so as to
             * correctly compute the sum of CFDATA accordingly.
             */
            if cfdata_safe.compressed_bytes_remaining as i32 > 0 as i32 {
                let mut bytes_avail_0: ssize_t = 0;
                d = __archive_read_ahead_safe(
                    a,
                    cfdata_safe.compressed_bytes_remaining as size_t,
                    &mut bytes_avail_0,
                );
                if bytes_avail_0 <= 0 as i32 as i64 {
                    *avail_safe = truncated_error(a) as ssize_t;
                    return 0 as *const ();
                }
                cfdata_safe.unconsumed = cfdata_safe.compressed_bytes_remaining as int64_t;
                cfdata_safe.sum_ptr = d;
                if cab_minimum_consume_cfdata(a, cfdata_safe.unconsumed)
                    < 0 as i32 as i64
                {
                    *avail_safe = ARCHIVE_CAB_DEFINED_PARAM.archive_fatal as ssize_t;
                    return 0 as *const ();
                }
            }
            /*
             * Set dictionary data for decompressing of next CFDATA, which
             * in the same folder. This is why we always do decompress CFDATA
             * even if beginning CFDATA or some of CFDATA are not used in
             * skipping file data.
             */
            if cab_cffolder_safe.cfdata_index < cab_cffolder_safe.cfdata_count as i32 {
                r = inflateReset_cab_safe(&mut cab_safe.stream);
                if r != ARCHIVE_CAB_DEFINED_PARAM.z_ok {
                    current_block = 12144037074258575129;
                } else {
                    r = inflateSetDictionary_safe(
                        &mut cab_safe.stream,
                        cab_safe.uncompressed_buffer,
                        cfdata_safe.uncompressed_size as uInt,
                    );
                    if r != ARCHIVE_CAB_DEFINED_PARAM.z_ok {
                        current_block = 12144037074258575129;
                    } else {
                        current_block = 796174441944384681;
                    }
                }
            } else {
                current_block = 796174441944384681;
            }
            match current_block {
                12144037074258575129 => {}
                _ => {
                    d = unsafe {
                        (*cab)
                            .uncompressed_buffer
                            .offset((*cfdata).read_offset as i32 as isize)
                            as *const ()
                    };
                    *avail_safe =
                        (uavail as i32 - cfdata_safe.read_offset as i32) as ssize_t;
                    cfdata_safe.uncompressed_avail = uavail;
                    return d;
                }
            }
        }
        4648980483242066537 => {
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                ARCHIVE_CAB_DEFINED_PARAM.archive_errno_misc,
                b"CFDATA incorrect(no MSZIP signature)\x00" as *const u8 as *const i8
            );
            *avail_safe = ARCHIVE_CAB_DEFINED_PARAM.archive_fatal as ssize_t;
            return 0 as *const ();
        }
        _ => {}
    }
    match r {
        -4 => {
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                ARCHIVE_CAB_DEFINED_PARAM.enomem,
                b"Out of memory for deflate decompression\x00" as *const u8 as *const i8
            );
        }
        _ => {
            archive_set_error_safe!(
                &mut (*a).archive as *mut archive,
                ARCHIVE_CAB_DEFINED_PARAM.archive_errno_misc,
                b"Deflate decompression failed (%d)\x00" as *const u8 as *const i8,
                r
            );
        }
    }
    *avail_safe = ARCHIVE_CAB_DEFINED_PARAM.archive_fatal as ssize_t;
    return 0 as *const ();
}

unsafe fn cab_read_ahead_cfdata_lzx(
    mut a: *mut archive_read,
    mut avail: *mut ssize_t,
) -> *const () {
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    let mut d: *const () = 0 as *const ();
    let mut r: i32 = 0;
    let mut uavail: uint16_t = 0;
    let cab_safe = unsafe { &mut *cab };
    cfdata = cab_safe.entry_cfdata;
    let a_safe = unsafe { &mut *a };
    let avail_safe = unsafe { &mut *avail };
    /* If the buffer hasn't been allocated, allocate it now. */
    if cab_safe.uncompressed_buffer.is_null() {
        cab_safe.uncompressed_buffer_size = 0x8000 as i32 as size_t;
        cab_safe.uncompressed_buffer =
            malloc_safe(cab_safe.uncompressed_buffer_size) as *mut u8;
        if cab_safe.uncompressed_buffer.is_null() {
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                12 as i32,
                b"No memory for CAB reader\x00" as *const u8 as *const i8
            );
            *avail_safe = -(30 as i32) as ssize_t;
            return 0 as *const ();
        }
    }
    let cfdata_safe = unsafe { &mut *cfdata };
    uavail = cfdata_safe.uncompressed_avail;
    if uavail as i32 == cfdata_safe.uncompressed_size as i32 {
        d = unsafe {
            (*cab)
                .uncompressed_buffer
                .offset((*cfdata).read_offset as i32 as isize)
                as *const ()
        };
        *avail_safe = (uavail as i32 - cfdata_safe.read_offset as i32) as ssize_t;
        return d;
    }
    let cab_cffolder_safe = unsafe { &mut (*(*cab).entry_cffolder) };
    if cab_cffolder_safe.decompress_init == 0 {
        r = lzx_decode_init(
            &mut cab_safe.xstrm,
            cab_cffolder_safe.compdata as i32,
        );
        if r != 0 as i32 {
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                -(1 as i32),
                b"Can\'t initialize LZX decompression.\x00" as *const u8 as *const i8
            );
            *avail_safe = -(30 as i32) as ssize_t;
            return 0 as *const ();
        }
        /* We've initialized decompression for this stream. */
        cab_cffolder_safe.decompress_init = 1 as i32 as i8
    }
    /* Clean up remaining bits of previous CFDATA. */
    lzx_cleanup_bitstream(&mut cab_safe.xstrm);
    cab_safe.xstrm.total_out = uavail as int64_t;
    while cab_safe.xstrm.total_out < cfdata_safe.uncompressed_size as i64 {
        let mut bytes_avail: ssize_t = 0;
        cab_safe.xstrm.next_out = unsafe {
            (*cab)
                .uncompressed_buffer
                .offset((*cab).xstrm.total_out as isize)
        };
        cab_safe.xstrm.avail_out =
            cfdata_safe.uncompressed_size as i64 - cab_safe.xstrm.total_out;
        d = __archive_read_ahead_safe(a, 1 as i32 as size_t, &mut bytes_avail);
        if bytes_avail <= 0 as i32 as i64 {
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                84 as i32,
                b"Truncated CAB file data\x00" as *const u8 as *const i8
            );
            *avail_safe = -(30 as i32) as ssize_t;
            return 0 as *const ();
        }
        if bytes_avail > cfdata_safe.compressed_bytes_remaining as i64 {
            bytes_avail = cfdata_safe.compressed_bytes_remaining as ssize_t
        }
        cab_safe.xstrm.next_in = d as *const u8;
        cab_safe.xstrm.avail_in = bytes_avail;
        cab_safe.xstrm.total_in = 0 as i32 as int64_t;
        r = lzx_decode(
            &mut cab_safe.xstrm,
            (cfdata_safe.compressed_bytes_remaining as i64 == bytes_avail) as i32,
        );
        match r {
            0 | 1 => {}
            _ => {
                archive_set_error_safe!(
                    &mut (*a).archive as *mut archive,
                    -(1 as i32),
                    b"LZX decompression failed (%d)\x00" as *const u8 as *const i8,
                    r
                );
                *avail_safe = -(30 as i32) as ssize_t;
                return 0 as *const ();
            }
        }
        cfdata_safe.unconsumed = cab_safe.xstrm.total_in;
        cfdata_safe.sum_ptr = d;
        if cab_minimum_consume_cfdata(a, cfdata_safe.unconsumed) < 0 as i32 as i64
        {
            *avail_safe = -(30 as i32) as ssize_t;
            return 0 as *const ();
        }
    }
    uavail = cab_safe.xstrm.total_out as uint16_t;
    /*
     * Make sure a read pointer advances to next CFDATA.
     */
    if cfdata_safe.compressed_bytes_remaining as i32 > 0 as i32 {
        let mut bytes_avail_0: ssize_t = 0;
        d = __archive_read_ahead_safe(
            a,
            cfdata_safe.compressed_bytes_remaining as size_t,
            &mut bytes_avail_0,
        );
        if bytes_avail_0 <= 0 as i32 as i64 {
            *avail_safe = truncated_error(a) as ssize_t;
            return 0 as *const ();
        }
        cfdata_safe.unconsumed = cfdata_safe.compressed_bytes_remaining as int64_t;
        cfdata_safe.sum_ptr = d;
        if cab_minimum_consume_cfdata(a, cfdata_safe.unconsumed) < 0 as i32 as i64
        {
            *avail_safe = -(30 as i32) as ssize_t;
            return 0 as *const ();
        }
    }
    /*
     * Translation reversal of x86 processor CALL byte sequence(E8).
     */

    lzx_translation(
        &mut cab_safe.xstrm,
        cab_safe.uncompressed_buffer as *mut (),
        cfdata_safe.uncompressed_size as size_t,
        ((cab_cffolder_safe.cfdata_index - 1 as i32) * 0x8000 as i32) as uint32_t,
    );
    d = unsafe {
        (*cab)
            .uncompressed_buffer
            .offset((*cfdata).read_offset as i32 as isize) as *const ()
    };
    *avail_safe = (uavail as i32 - cfdata_safe.read_offset as i32) as ssize_t;
    cfdata_safe.uncompressed_avail = uavail;
    return d;
}
/*
 * Consume CFDATA.
 * We always decompress CFDATA to consume CFDATA as much as we need
 * in uncompressed bytes because all CFDATA in a folder are related
 * so we do not skip any CFDATA without decompressing.
 * Note: If the folder of a CFFILE is iFoldCONTINUED_PREV_AND_NEXT or
 * iFoldCONTINUED_FROM_PREV, we won't decompress because a CFDATA for
 * the CFFILE is remaining bytes of previous Multivolume CAB file.
 */
unsafe fn cab_consume_cfdata(
    mut a: *mut archive_read,
    mut consumed_bytes: int64_t,
) -> int64_t {
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    let mut cbytes: int64_t = 0;
    let mut rbytes: int64_t = 0;
    let mut err: i32 = 0;
    rbytes = cab_minimum_consume_cfdata(a, consumed_bytes);
    if rbytes < 0 as i32 as i64 {
        return -(30 as i32) as int64_t;
    }
    let cab_safe = unsafe { &mut *cab };
    cfdata = cab_safe.entry_cfdata;
    let cfdata_safe = unsafe { &mut *cfdata };
    let a_safe = unsafe { &mut *a };
    let cab_cffile_safe = unsafe { &mut (*(*cab).entry_cffile) };
    while rbytes > 0 as i32 as i64 {
        let mut avail: ssize_t = 0;
        if cfdata_safe.compressed_size as i32 == 0 as i32 {
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                84 as i32,
                b"Invalid CFDATA\x00" as *const u8 as *const i8
            );
            return -(30 as i32) as int64_t;
        }
        cbytes = cfdata_safe.uncompressed_bytes_remaining as int64_t;
        if cbytes > rbytes {
            cbytes = rbytes
        }
        rbytes -= cbytes;
        if cfdata_safe.uncompressed_avail as i32 == 0 as i32
            && (cab_cffile_safe.folder as i32 == 0xffff as i32
                || cab_cffile_safe.folder as i32 == 0xfffd as i32)
        {
            /* We have not read any data yet. */
            if cbytes == cfdata_safe.uncompressed_bytes_remaining as i64 {
                /* Skip whole current CFDATA. */
                __archive_read_consume_safe(a, cfdata_safe.compressed_size as int64_t);
                cab_safe.cab_offset += cfdata_safe.compressed_size as i64;
                cfdata_safe.compressed_bytes_remaining = 0 as i32 as uint16_t;
                cfdata_safe.uncompressed_bytes_remaining = 0 as i32 as uint16_t;
                err = cab_next_cfdata(a);
                if err < 0 as i32 {
                    return err as int64_t;
                }
                cfdata = cab_safe.entry_cfdata;
                if cfdata_safe.uncompressed_size as i32 == 0 as i32 {
                    match cab_cffile_safe.folder as i32 {
                        65535 | 65534 | 65533 => rbytes = 0 as i32 as int64_t,
                        _ => {}
                    }
                }
            } else {
                cfdata_safe.read_offset = (cfdata_safe.read_offset as i32
                    + cbytes as uint16_t as i32)
                    as uint16_t;
                cfdata_safe.uncompressed_bytes_remaining =
                    (cfdata_safe.uncompressed_bytes_remaining as i32
                        - cbytes as uint16_t as i32) as uint16_t;
                break;
            }
        } else if cbytes == 0 as i32 as i64 {
            err = cab_next_cfdata(a);
            if err < 0 as i32 {
                return err as int64_t;
            }
            cfdata = cab_safe.entry_cfdata;
            if cfdata_safe.uncompressed_size as i32 == 0 as i32 {
                match cab_cffile_safe.folder as i32 {
                    65535 | 65534 | 65533 => return -(30 as i32) as int64_t,
                    _ => {}
                }
            }
        } else {
            while cbytes > 0 as i32 as i64 {
                cab_read_ahead_cfdata(a, &mut avail);
                if avail <= 0 as i32 as i64 {
                    return -(30 as i32) as int64_t;
                }
                if avail > cbytes {
                    avail = cbytes
                }
                if cab_minimum_consume_cfdata(a, avail) < 0 as i32 as i64 {
                    return -(30 as i32) as int64_t;
                }
                cbytes -= avail
            }
        }
    }
    return consumed_bytes;
}
/*
 * Consume CFDATA as much as we have already gotten and
 * compute the sum of CFDATA.
 */
unsafe fn cab_minimum_consume_cfdata(
    mut a: *mut archive_read,
    mut consumed_bytes: int64_t,
) -> int64_t {
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    let mut cbytes: int64_t = 0;
    let mut rbytes: int64_t = 0;
    let mut err: i32 = 0;
    let cab_safe = unsafe { &mut *cab };
    cfdata = cab_safe.entry_cfdata;
    rbytes = consumed_bytes;
    let cfdata_safe = unsafe { &mut *cfdata };
    if unsafe { (*(*cab).entry_cffolder).comptype as i32 == 0 as i32 } {
        if consumed_bytes < cfdata_safe.unconsumed {
            cbytes = consumed_bytes
        } else {
            cbytes = cfdata_safe.unconsumed
        }
        rbytes -= cbytes;
        cfdata_safe.read_offset = (cfdata_safe.read_offset as i32
            + cbytes as uint16_t as i32) as uint16_t;
        cfdata_safe.uncompressed_bytes_remaining =
            (cfdata_safe.uncompressed_bytes_remaining as i32
                - cbytes as uint16_t as i32) as uint16_t;
        cfdata_safe.unconsumed -= cbytes
    } else {
        cbytes = (cfdata_safe.uncompressed_avail as i32
            - cfdata_safe.read_offset as i32) as int64_t;
        if cbytes > 0 as i32 as i64 {
            if consumed_bytes < cbytes {
                cbytes = consumed_bytes
            }
            rbytes -= cbytes;
            cfdata_safe.read_offset = (cfdata_safe.read_offset as i32
                + cbytes as uint16_t as i32)
                as uint16_t;
            cfdata_safe.uncompressed_bytes_remaining =
                (cfdata_safe.uncompressed_bytes_remaining as i32
                    - cbytes as uint16_t as i32) as uint16_t
        }
        if cfdata_safe.unconsumed != 0 {
            cbytes = cfdata_safe.unconsumed;
            cfdata_safe.unconsumed = 0 as i32 as int64_t
        } else {
            cbytes = 0 as i32 as int64_t
        }
    }
    if cbytes != 0 {
        /* Compute the sum. */
        cab_checksum_update(a, cbytes as size_t);
        /* Consume as much as the compressor actually used. */
        __archive_read_consume_safe(a, cbytes);
        cab_safe.cab_offset += cbytes;
        cfdata_safe.compressed_bytes_remaining =
            (cfdata_safe.compressed_bytes_remaining as i32
                - cbytes as uint16_t as i32) as uint16_t;
        if cfdata_safe.compressed_bytes_remaining as i32 == 0 as i32 {
            err = cab_checksum_finish(a);
            if err < 0 as i32 {
                return err as int64_t;
            }
        }
    }
    return rbytes;
}
/*
 * Returns ARCHIVE_OK if successful, ARCHIVE_FATAL otherwise, sets
 * cab->end_of_entry if it consumes all of the data.
 */
unsafe fn cab_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const (),
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> i32 {
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let mut bytes_avail: ssize_t = 0;
    let cab_safe = unsafe { &mut *cab };
    let buff_safe;
    let size_safe;
    let offset_safe;
    unsafe {
        buff_safe = &mut *buff;
        size_safe = &mut *size;
        offset_safe = &mut *offset;
    }
    if cab_safe.entry_bytes_remaining == 0 as i32 as i64 {
        *buff_safe = 0 as *const ();
        *size_safe = 0 as i32 as size_t;
        *offset_safe = cab_safe.entry_offset;
        cab_safe.end_of_entry = 1 as i32 as i8;
        return 0 as i32;
    }
    *buff_safe = cab_read_ahead_cfdata(a, &mut bytes_avail);
    let cab_cfdata_safe = unsafe { &mut (*(*cab).entry_cfdata) };
    let a_safe = unsafe { &mut *a };
    if bytes_avail <= 0 as i32 as i64 {
        *buff_safe = 0 as *const ();
        *size_safe = 0 as i32 as size_t;
        *offset_safe = 0 as i32 as int64_t;
        if bytes_avail == 0 as i32 as i64
            && cab_cfdata_safe.uncompressed_size as i32 == 0 as i32
        {
            /* All of CFDATA in a folder has been handled. */
            archive_set_error_safe!(
                &mut a_safe.archive as *mut archive,
                84 as i32,
                b"Invalid CFDATA\x00" as *const u8 as *const i8
            );
            return -(30 as i32);
        } else {
            return bytes_avail as i32;
        }
    }
    if bytes_avail > cab_safe.entry_bytes_remaining {
        bytes_avail = cab_safe.entry_bytes_remaining
    }
    *size_safe = bytes_avail as size_t;
    *offset_safe = cab_safe.entry_offset;
    cab_safe.entry_offset += bytes_avail;
    cab_safe.entry_bytes_remaining -= bytes_avail;
    if cab_safe.entry_bytes_remaining == 0 as i32 as i64 {
        cab_safe.end_of_entry = 1 as i32 as i8
    }
    cab_safe.entry_unconsumed = bytes_avail;
    if unsafe { (*(*cab).entry_cffolder).comptype as i32 == 0 as i32 } {
        /* Don't consume more than current entry used. */
        if cab_cfdata_safe.unconsumed > cab_safe.entry_unconsumed {
            cab_cfdata_safe.unconsumed = cab_safe.entry_unconsumed
        }
    }
    return 0 as i32;
}
unsafe fn archive_read_format_cab_read_data_skip(
    mut a: *mut archive_read,
) -> i32 {
    let mut cab: *mut cab = 0 as *mut cab;
    let mut bytes_skipped: int64_t = 0;
    let mut r: i32 = 0;
    cab = unsafe { (*(*a).format).data as *mut cab };
    let cab_safe = unsafe { &mut *cab };
    if cab_safe.end_of_archive != 0 {
        return 1 as i32;
    }
    if cab_safe.read_data_invoked == 0 {
        cab_safe.bytes_skipped += cab_safe.entry_bytes_remaining;
        cab_safe.entry_bytes_remaining = 0 as i32 as int64_t;
        /* This entry is finished and done. */
        cab_safe.end_of_entry = 1 as i32 as i8;
        cab_safe.end_of_entry_cleanup = cab_safe.end_of_entry;
        return 0 as i32;
    }
    if cab_safe.entry_unconsumed != 0 {
        /* Consume as much as the compressor actually used. */
        r = cab_consume_cfdata(a, cab_safe.entry_unconsumed) as i32;
        cab_safe.entry_unconsumed = 0 as i32 as int64_t;
        if r < 0 as i32 {
            return r;
        }
    } else if cab_safe.entry_cfdata.is_null() {
        r = cab_next_cfdata(a);
        if r < 0 as i32 {
            return r;
        }
    }
    /* if we've already read to end of data, we're done. */
    if cab_safe.end_of_entry_cleanup != 0 {
        return 0 as i32;
    }
    /*
     * If the length is at the beginning, we can skip the
     * compressed data much more quickly.
     */
    bytes_skipped = cab_consume_cfdata(a, cab_safe.entry_bytes_remaining);
    if bytes_skipped < 0 as i32 as i64 {
        return -(30 as i32);
    }
    /* If the compression type is none(uncompressed), we've already
     * consumed data as much as the current entry size. */
    unsafe {
        if (*(*cab).entry_cffolder).comptype as i32 == 0 as i32
            && !(*cab).entry_cfdata.is_null()
        {
            (*(*cab).entry_cfdata).unconsumed = 0 as i32 as int64_t
        }
    }
    /* This entry is finished and done. */
    cab_safe.end_of_entry = 1 as i32 as i8;
    cab_safe.end_of_entry_cleanup = cab_safe.end_of_entry;
    return 0 as i32;
}
unsafe fn archive_read_format_cab_cleanup(mut a: *mut archive_read) -> i32 {
    let mut cab: *mut cab = unsafe { (*(*a).format).data as *mut cab };
    let cab_safe = unsafe { &mut *cab };
    let mut hd: *mut cfheader = &mut cab_safe.cfheader;
    let mut i: i32 = 0;
    let hd_safe = unsafe { &mut *hd };
    if !hd_safe.folder_array.is_null() {
        i = 0 as i32;
        while i < hd_safe.folder_count as i32 {
            unsafe {
                free((*(*hd).folder_array.offset(i as isize)).cfdata.memimage as *mut ());
            }
            i += 1
        }
        free_safe(hd_safe.folder_array as *mut ());
    }
    if !hd_safe.file_array.is_null() {
        i = 0 as i32;
        while i < cab_safe.cfheader.file_count as i32 {
            unsafe {
                archive_string_free(&mut (*(*hd).file_array.offset(i as isize)).pathname);
            }
            i += 1
        }
        free_safe(hd_safe.file_array as *mut ());
    }
    match () {
        #[cfg(HAVE_ZLIB_H)]
        _ => {
            if cab_safe.stream_valid != 0 {
                inflateEnd_cab_safe(&mut cab_safe.stream);
            }
        }
        #[cfg(not(HAVE_ZLIB_H))]
        _ => {}
    }

    lzx_decode_free(&mut cab_safe.xstrm);
    archive_wstring_free_safe(&mut cab_safe.ws);
    free_safe(cab_safe.uncompressed_buffer as *mut ());
    free_safe(cab as *mut ());
    unsafe { (*(*a).format).data = 0 as *mut () };
    return 0 as i32;
}
/* Convert an MSDOS-style date/time into Unix-style time. */
unsafe fn cab_dos_time(mut p: *const u8) -> time_t {
    let mut msTime: i32 = 0; /* Years since 1900. */
    let mut msDate: i32 = 0; /* Month number.     */
    let mut ts: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const i8,
    }; /* Day of month.     */
    msDate = archive_le16dec(p as *const ()) as i32;
    msTime = unsafe {
        archive_le16dec(p.offset(2 as i32 as isize) as *const ()) as i32
    };
    memset_safe(
        &mut ts as *mut tm as *mut (),
        0 as i32,
        ::std::mem::size_of::<tm>() as u64,
    );
    ts.tm_year = (msDate >> 9 as i32 & 0x7f as i32) + 80 as i32;
    ts.tm_mon = (msDate >> 5 as i32 & 0xf as i32) - 1 as i32;
    ts.tm_mday = msDate & 0x1f as i32;
    ts.tm_hour = msTime >> 11 as i32 & 0x1f as i32;
    ts.tm_min = msTime >> 5 as i32 & 0x3f as i32;
    ts.tm_sec = msTime << 1 as i32 & 0x3e as i32;
    ts.tm_isdst = -(1 as i32);
    return mktime_safe(&mut ts);
}
/* ****************************************************************
 *
 * LZX decompression code.
 *
 *****************************************************************/
/*
 * Initialize LZX decoder.
 *
 * Returns ARCHIVE_OK if initialization was successful.
 * Returns ARCHIVE_FAILED if w_bits has unsupported value.
 * Returns ARCHIVE_FATAL if initialization failed; memory allocation
 * error occurred.
 */
unsafe fn lzx_decode_init(
    mut strm: *mut lzx_stream,
    mut w_bits: i32,
) -> i32 {
    let mut ds: *mut lzx_dec = 0 as *mut lzx_dec;
    let mut slot: i32 = 0;
    let mut w_size: i32 = 0;
    let mut w_slot: i32 = 0;
    let mut base: i32 = 0;
    let mut footer: i32 = 0;
    let mut base_inc: [i32; 18] = [0; 18];
    let strm_safe = unsafe { &mut *strm };
    if strm_safe.ds.is_null() {
        strm_safe.ds = calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<lzx_dec>() as u64,
        ) as *mut lzx_dec;
        if strm_safe.ds.is_null() {
            return -(30 as i32);
        }
    }
    ds = strm_safe.ds;
    let ds_safe = unsafe { &mut *ds };
    ds_safe.error = -(25 as i32);
    /* Allow bits from 15(32KBi) up to 21(2MBi) */
    if w_bits < 15 as i32 || w_bits > 21 as i32 {
        return -(25 as i32);
    }
    ds_safe.error = -(30 as i32);
    /*
     * Alloc window
     */
    w_size = ds_safe.w_size;
    w_slot = unsafe { slots[(w_bits - 15 as i32) as usize] };
    ds_safe.w_size = ((1 as u32) << w_bits) as i32;
    ds_safe.w_mask = ds_safe.w_size - 1 as i32;
    if ds_safe.w_buff.is_null() || w_size != ds_safe.w_size {
        free_safe(ds_safe.w_buff as *mut ());
        ds_safe.w_buff = malloc_safe(ds_safe.w_size as u64) as *mut u8;
        if ds_safe.w_buff.is_null() {
            return -(30 as i32);
        }
        free_safe(ds_safe.pos_tbl as *mut ());
        ds_safe.pos_tbl = malloc_safe(
            (::std::mem::size_of::<lzx_pos_tbl>() as u64)
                .wrapping_mul(w_slot as u64),
        ) as *mut lzx_pos_tbl;
        if ds_safe.pos_tbl.is_null() {
            return -(30 as i32);
        }
    }
    footer = 0 as i32;
    while footer < 18 as i32 {
        base_inc[footer as usize] = (1 as i32) << footer;
        footer += 1
    }
    footer = 0 as i32;
    base = footer;
    slot = 0 as i32;
    while slot < w_slot {
        let mut n: i32 = 0;
        if footer == 0 as i32 {
            base = slot
        } else {
            base += base_inc[footer as usize]
        }
        if footer < 17 as i32 {
            footer = -(2 as i32);
            n = base;
            while n != 0 {
                footer += 1;
                n >>= 1 as i32
            }
            if footer <= 0 as i32 {
                footer = 0 as i32
            }
        }
        unsafe {
            (*(*ds).pos_tbl.offset(slot as isize)).base = base;
            (*(*ds).pos_tbl.offset(slot as isize)).footer_bits = footer;
        }
        slot += 1
    }
    ds_safe.w_pos = 0 as i32;
    ds_safe.state = 0 as i32;
    ds_safe.br.cache_buffer = 0 as i32 as uint64_t;
    ds_safe.br.cache_avail = 0 as i32;
    ds_safe.r2 = 1 as i32;
    ds_safe.r1 = ds_safe.r2;
    ds_safe.r0 = ds_safe.r1;
    /* Initialize aligned offset tree. */
    if lzx_huffman_init(
        &mut ds_safe.at,
        8 as i32 as size_t,
        8 as i32,
    ) != 0 as i32
    {
        return -(30 as i32);
    }
    /* Initialize pre-tree. */
    if lzx_huffman_init(
        &mut ds_safe.pt,
        20 as i32 as size_t,
        10 as i32,
    ) != 0 as i32
    {
        return -(30 as i32);
    }
    /* Initialize Main tree. */
    if lzx_huffman_init(
        &mut ds_safe.mt,
        (256 as i32 + (w_slot << 3 as i32)) as size_t,
        16 as i32,
    ) != 0 as i32
    {
        return -(30 as i32);
    }
    /* Initialize Length tree. */
    if lzx_huffman_init(
        &mut ds_safe.lt,
        249 as i32 as size_t,
        16 as i32,
    ) != 0 as i32
    {
        return -(30 as i32);
    }
    ds_safe.error = 0 as i32;
    return 0 as i32;
}
/*
 * Release LZX decoder.
 */
unsafe fn lzx_decode_free(mut strm: *mut lzx_stream) {
    let strm_safe = unsafe { &mut *strm };
    if strm_safe.ds.is_null() {
        return;
    }
    let strm_ds_safe = unsafe { &mut (*(*strm).ds) };
    free_safe(strm_ds_safe.w_buff as *mut ());
    free_safe(strm_ds_safe.pos_tbl as *mut ());
    lzx_huffman_free(&mut strm_ds_safe.at);
    lzx_huffman_free(&mut strm_ds_safe.pt);
    lzx_huffman_free(&mut strm_ds_safe.mt);
    lzx_huffman_free(&mut strm_ds_safe.lt);
    free_safe(strm_safe.ds as *mut ());
    strm_safe.ds = 0 as *mut lzx_dec;
}
/*
 * E8 Call Translation reversal.
 */
unsafe fn lzx_translation(
    mut strm: *mut lzx_stream,
    mut p: *mut (),
    mut size: size_t,
    mut offset: uint32_t,
) {
    let strm_safe = unsafe { &mut *strm };
    let mut ds: *mut lzx_dec = strm_safe.ds;
    let mut b: *mut u8 = 0 as *mut u8;
    let mut end: *mut u8 = 0 as *mut u8;
    let ds_safe = unsafe { &mut *ds };
    if ds_safe.translation == 0 || size <= 10 as i32 as u64 {
        return;
    }
    b = p as *mut u8;
    end = unsafe {
        b.offset(size as isize)
            .offset(-(10 as i32 as isize))
    };
    while b < end && {
        b = unsafe {
            memchr_safe(
                b as *const (),
                0xe8 as i32,
                end.offset_from(b) as i64 as u64,
            ) as *mut u8
        };
        !b.is_null()
    } {
        let mut i: size_t =
            unsafe { b.offset_from(p as *mut u8) as i64 as size_t };
        let mut cp: int32_t = 0;
        let mut displacement: int32_t = 0;
        let mut value: int32_t = 0;
        cp = offset.wrapping_add(i as uint32_t) as int32_t;
        value = unsafe {
            archive_le32dec(
                &mut *b.offset(1 as i32 as isize) as *mut u8
                    as *const (),
            ) as int32_t
        };
        if value >= -cp && value < ds_safe.translation_size as int32_t {
            if value >= 0 as i32 {
                displacement = value - cp
            } else {
                displacement =
                    (value as u32).wrapping_add(ds_safe.translation_size) as int32_t
            }
            unsafe {
                archive_le32enc(
                    &mut *b.offset(1 as i32 as isize) as *mut u8
                        as *mut (),
                    displacement as uint32_t,
                );
            }
        }
        b = unsafe { b.offset(5 as i32 as isize) }
    }
}
static mut cache_masks: [uint32_t; 36] = [
    0 as i32 as uint32_t,
    0x1 as i32 as uint32_t,
    0x3 as i32 as uint32_t,
    0x7 as i32 as uint32_t,
    0xf as i32 as uint32_t,
    0x1f as i32 as uint32_t,
    0x3f as i32 as uint32_t,
    0x7f as i32 as uint32_t,
    0xff as i32 as uint32_t,
    0x1ff as i32 as uint32_t,
    0x3ff as i32 as uint32_t,
    0x7ff as i32 as uint32_t,
    0xfff as i32 as uint32_t,
    0x1fff as i32 as uint32_t,
    0x3fff as i32 as uint32_t,
    0x7fff as i32 as uint32_t,
    0xffff as i32 as uint32_t,
    0x1ffff as i32 as uint32_t,
    0x3ffff as i32 as uint32_t,
    0x7ffff as i32 as uint32_t,
    0xfffff as i32 as uint32_t,
    0x1fffff as i32 as uint32_t,
    0x3fffff as i32 as uint32_t,
    0x7fffff as i32 as uint32_t,
    0xffffff as i32 as uint32_t,
    0x1ffffff as i32 as uint32_t,
    0x3ffffff as i32 as uint32_t,
    0x7ffffff as i32 as uint32_t,
    0xfffffff as i32 as uint32_t,
    0x1fffffff as i32 as uint32_t,
    0x3fffffff as i32 as uint32_t,
    0x7fffffff as i32 as uint32_t,
    0xffffffff as u32,
    0xffffffff as u32,
    0xffffffff as u32,
    0xffffffff as u32,
];
/*
 * Shift away used bits in the cache data and fill it up with following bits.
 * Call this when cache buffer does not have enough bits you need.
 *
 * Returns 1 if the cache buffer is full.
 * Returns 0 if the cache buffer is not full; input buffer is empty.
 */
unsafe fn lzx_br_fillup(mut strm: *mut lzx_stream, mut br: *mut lzx_br) -> i32 {
    /*
     * x86 processor family can read misaligned data without an access error.
     */
    let br_safe = unsafe { &mut *br };
    let mut n: i32 = (8 as i32 as u64)
        .wrapping_mul(::std::mem::size_of::<uint64_t>() as u64)
        .wrapping_sub(br_safe.cache_avail as u64)
        as i32;
    loop {
        let strm_safe = unsafe { &mut *strm };
        match n >> 4 as i32 {
            4 => {
                if strm_safe.avail_in >= 8 as i32 as i64 {
                    unsafe {
                        br_safe.cache_buffer = (*(*strm).next_in.offset(1 as i32 as isize)
                            as uint64_t)
                            << 56 as i32
                            | (*(*strm).next_in.offset(0 as i32 as isize) as uint64_t)
                                << 48 as i32
                            | (*(*strm).next_in.offset(3 as i32 as isize) as uint64_t)
                                << 40 as i32
                            | (*(*strm).next_in.offset(2 as i32 as isize) as uint64_t)
                                << 32 as i32
                            | ((*(*strm).next_in.offset(5 as i32 as isize) as uint32_t)
                                << 24 as i32)
                                as u64
                            | ((*(*strm).next_in.offset(4 as i32 as isize) as uint32_t)
                                << 16 as i32)
                                as u64
                            | ((*(*strm).next_in.offset(7 as i32 as isize) as uint32_t)
                                << 8 as i32) as u64
                            | *(*strm).next_in.offset(6 as i32 as isize) as uint32_t
                                as u64;
                        (*strm).next_in = (*strm).next_in.offset(8 as i32 as isize);
                    }
                    strm_safe.avail_in -= 8 as i32 as i64;
                    br_safe.cache_avail += 8 as i32 * 8 as i32;
                    return 1 as i32;
                }
            }
            3 => {
                if strm_safe.avail_in >= 6 as i32 as i64 {
                    unsafe {
                        (*br).cache_buffer = (*br).cache_buffer << 48 as i32
                            | (*(*strm).next_in.offset(1 as i32 as isize) as uint64_t)
                                << 40 as i32
                            | (*(*strm).next_in.offset(0 as i32 as isize) as uint64_t)
                                << 32 as i32
                            | ((*(*strm).next_in.offset(3 as i32 as isize) as uint32_t)
                                << 24 as i32)
                                as u64
                            | ((*(*strm).next_in.offset(2 as i32 as isize) as uint32_t)
                                << 16 as i32)
                                as u64
                            | ((*(*strm).next_in.offset(5 as i32 as isize) as uint32_t)
                                << 8 as i32) as u64
                            | *(*strm).next_in.offset(4 as i32 as isize) as uint32_t
                                as u64;
                        (*strm).next_in = (*strm).next_in.offset(6 as i32 as isize);
                    }
                    strm_safe.avail_in -= 6 as i32 as i64;
                    br_safe.cache_avail += 6 as i32 * 8 as i32;
                    return 1 as i32;
                }
            }
            0 => {
                /* We have enough compressed data in
                 * the cache buffer.*/
                return 1 as i32;
            }
            _ => {}
        }
        if strm_safe.avail_in < 2 as i32 as i64 {
            /* There is not enough compressed data to
             * fill up the cache buffer. */
            if strm_safe.avail_in == 1 as i32 as i64 {
                let fresh5 = strm_safe.next_in;
                unsafe {
                    (*strm).next_in = (*strm).next_in.offset(1);
                    (*br).odd = *fresh5;
                }
                strm_safe.avail_in -= 1;
                br_safe.have_odd = 1 as i32 as i8
            }
            return 0 as i32;
        }
        br_safe.cache_buffer = br_safe.cache_buffer << 16 as i32
            | archive_le16dec(strm_safe.next_in as *const ()) as u64;
        strm_safe.next_in = unsafe { (*strm).next_in.offset(2 as i32 as isize) };
        strm_safe.avail_in -= 2 as i32 as i64;
        br_safe.cache_avail += 16 as i32;
        n -= 16 as i32
    }
}
unsafe fn lzx_br_fixup(mut strm: *mut lzx_stream, mut br: *mut lzx_br) {
    let br_safe = unsafe { &mut *br };
    let mut n: i32 = (8 as i32 as u64)
        .wrapping_mul(::std::mem::size_of::<uint64_t>() as u64)
        .wrapping_sub(br_safe.cache_avail as u64)
        as i32;
    let strm_safe = unsafe { &mut *strm };
    if br_safe.have_odd as i32 != 0
        && n >= 16 as i32
        && strm_safe.avail_in > 0 as i32 as i64
    {
        br_safe.cache_buffer = unsafe {
            br_safe.cache_buffer << 16 as i32
                | ((*(*strm).next_in as uint16_t as i32) << 8 as i32)
                    as u64
                | br_safe.odd as u64
        };
        strm_safe.next_in = unsafe { strm_safe.next_in.offset(1) };
        strm_safe.avail_in -= 1;
        br_safe.cache_avail += 16 as i32;
        br_safe.have_odd = 0 as i32 as i8
    };
}
unsafe fn lzx_cleanup_bitstream(mut strm: *mut lzx_stream) {
    let strm_ds_safe = unsafe { &mut (*(*strm).ds) };
    strm_ds_safe.br.cache_avail = 0 as i32;
    strm_ds_safe.br.have_odd = 0 as i32 as i8;
}
unsafe fn lzx_decode(mut strm: *mut lzx_stream, mut last: i32) -> i32 {
    let strm_safe = unsafe { &mut *strm };
    let mut ds: *mut lzx_dec = strm_safe.ds;
    let mut avail_in: int64_t = 0;
    let mut r: i32 = 0;
    let ds_safe = unsafe { &mut *ds };
    if ds_safe.error != 0 {
        return ds_safe.error;
    }
    if ds_safe.error != 0 {
        return ds_safe.error;
    }
    avail_in = strm_safe.avail_in;
    lzx_br_fixup(strm, &mut ds_safe.br);
    loop {
        if ds_safe.state < 18 as i32 {
            r = lzx_read_blocks(strm, last)
        } else {
            let mut bytes_written: int64_t = strm_safe.avail_out;
            unsafe {
                r = lzx_decode_blocks(strm, last);
            }
            bytes_written -= strm_safe.avail_out;
            strm_safe.next_out = unsafe { strm_safe.next_out.offset(bytes_written as isize) };
            strm_safe.total_out += bytes_written
        }
        if !(r == 100 as i32) {
            break;
        }
    }
    strm_safe.total_in += avail_in - strm_safe.avail_in;
    return r;
}
unsafe fn lzx_read_blocks(
    mut strm: *mut lzx_stream,
    mut last: i32,
) -> i32 {
    let mut current_block: u64;
    let strm_safe = unsafe { &mut *strm };
    let mut ds: *mut lzx_dec = strm_safe.ds;
    let ds_safe = unsafe { &mut *ds };
    let mut br: *mut lzx_br = &mut ds_safe.br;
    let mut i: i32 = 0;
    let mut r: i32 = 0;
    let br_safe = unsafe { &mut *br };
    's_16: loop {
        match ds_safe.state {
            0 => {
                if !(br_safe.cache_avail >= 1 as i32
                    || lzx_br_fillup(strm, br) != 0
                    || br_safe.cache_avail >= 1 as i32)
                {
                    ds_safe.state = 0 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                } else {
                    ds_safe.translation = unsafe {
                        ((br_safe.cache_buffer >> br_safe.cache_avail - 1 as i32)
                            as uint32_t
                            & cache_masks[1 as i32 as usize])
                            as i8
                    };
                    br_safe.cache_avail -= 1 as i32
                }
                current_block = 15836835945094511460;
            }
            1 => {
                current_block = 15836835945094511460;
            }
            2 => {
                current_block = 16145219462989692018;
            }
            3 => {
                current_block = 18257203903591193900;
            }
            4 => {
                current_block = 16465730530612695416;
            }
            5 | 6 | 7 => {
                current_block = 1724319918354933278;
            }
            8 => {
                current_block = 5023038348526654800;
            }
            9 => {
                current_block = 1130861444095256174;
            }
            10 => {
                /*
                 * Read Aligned offset tree.
                 */
                if !(br_safe.cache_avail >= 3 as i32 * ds_safe.at.len_size
                    || lzx_br_fillup(strm, br) != 0
                    || br_safe.cache_avail >= 3 as i32 * ds_safe.at.len_size)
                {
                    ds_safe.state = 10 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                } else {
                    memset_safe(
                        ds_safe.at.freq.as_mut_ptr() as *mut (),
                        0 as i32,
                        ::std::mem::size_of::<[i32; 17]>() as u64,
                    );
                    i = 0 as i32;
                    while i < ds_safe.at.len_size {
                        unsafe {
                            *(*ds).at.bitlen.offset(i as isize) = ((br_safe.cache_buffer
                                >> br_safe.cache_avail - 3 as i32)
                                as uint32_t
                                & cache_masks[3 as i32 as usize])
                                as u8;
                            ds_safe.at.freq[*(*ds).at.bitlen.offset(i as isize) as usize] += 1;
                        }
                        br_safe.cache_avail -= 3 as i32;
                        i += 1
                    }
                    if lzx_make_huffman_table(&mut ds_safe.at) == 0 {
                        break;
                    }
                }
                current_block = 10834452935023522597;
            }
            11 => {
                current_block = 10834452935023522597;
            }
            12 => {
                current_block = 17947203442271072565;
            }
            13 => {
                current_block = 12175694472802639057;
            }
            14 => {
                current_block = 14663568441095876955;
            }
            15 => {
                current_block = 11522940221586662047;
            }
            16 => {
                current_block = 10662747035737099349;
            }
            17 => {
                current_block = 8491080914264407520;
            }
            _ => {
                continue;
            }
        }
        match current_block {
            15836835945094511460 =>
            /* FALL THROUGH */
            {
                if ds_safe.translation != 0 {
                    if !(br_safe.cache_avail >= 32 as i32
                        || lzx_br_fillup(strm, br) != 0
                        || br_safe.cache_avail >= 32 as i32)
                    {
                        ds_safe.state = 1 as i32;
                        if last != 0 {
                            break;
                        }
                        return 0 as i32;
                    } else {
                        ds_safe.translation_size = unsafe {
                            (br_safe.cache_buffer >> br_safe.cache_avail - 16 as i32)
                                as uint32_t
                                & cache_masks[16 as i32 as usize]
                        };
                        br_safe.cache_avail -= 16 as i32;
                        ds_safe.translation_size <<= 16 as i32;
                        ds_safe.translation_size |= unsafe {
                            (br_safe.cache_buffer >> br_safe.cache_avail - 16 as i32)
                                as uint32_t
                                & cache_masks[16 as i32 as usize]
                        };
                        br_safe.cache_avail -= 16 as i32
                    }
                    current_block = 16145219462989692018;
                } else {
                    current_block = 16145219462989692018;
                }
            }
            10834452935023522597 =>
            /* FALL THROUGH */
            {
                ds_safe.loop_0 = 0 as i32;
                /* FALL THROUGH */
                current_block = 17947203442271072565;
            }
            _ => {}
        }
        match current_block {
            17947203442271072565 =>
            /*
             * Read Pre-tree for first 256 elements of main tree.
             */
            {
                if lzx_read_pre_tree(strm) == 0 {
                    ds_safe.state = 12 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                } else {
                    if lzx_make_huffman_table(&mut ds_safe.pt) == 0 {
                        break;
                    }
                    ds_safe.loop_0 = 0 as i32
                }
                current_block = 12175694472802639057;
            }
            16145219462989692018 =>
            /* FALL THROUGH */
            {
                if !(br_safe.cache_avail >= 3 as i32
                    || lzx_br_fillup(strm, br) != 0
                    || br_safe.cache_avail >= 3 as i32)
                {
                    ds_safe.state = 2 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                } else {
                    ds_safe.block_type = unsafe {
                        ((br_safe.cache_buffer >> br_safe.cache_avail - 3 as i32)
                            as uint32_t
                            & cache_masks[3 as i32 as usize])
                            as i8
                    };
                    br_safe.cache_avail -= 3 as i32;
                    /* Check a block type. */
                    match ds_safe.block_type as i32 {
                        1 | 2 | 3 => {}
                        _ => {
                            break;
                        }
                    }
                }
                current_block = 18257203903591193900;
            }
            _ => {}
        }
        match current_block {
            12175694472802639057 =>
            /* FALL THROUGH */
            /*
             * Get path lengths of first 256 elements of main tree.
             */
            {
                r = lzx_read_bitlen(strm, &mut ds_safe.mt, 256 as i32);
                if r < 0 as i32 {
                    break;
                }
                if r == 0 {
                    ds_safe.state = 13 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                } else {
                    ds_safe.loop_0 = 0 as i32
                }
                current_block = 14663568441095876955;
            }
            18257203903591193900 =>
            /* FALL THROUGH */
            {
                if !(br_safe.cache_avail >= 24 as i32
                    || lzx_br_fillup(strm, br) != 0
                    || br_safe.cache_avail >= 24 as i32)
                {
                    ds_safe.state = 3 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                } else {
                    ds_safe.block_size = unsafe {
                        ((br_safe.cache_buffer >> br_safe.cache_avail - 8 as i32)
                            as uint32_t
                            & cache_masks[8 as i32 as usize])
                            as size_t
                    };
                    br_safe.cache_avail -= 8 as i32;
                    ds_safe.block_size <<= 16 as i32;
                    ds_safe.block_size |= unsafe {
                        ((br_safe.cache_buffer >> br_safe.cache_avail - 16 as i32)
                            as uint32_t
                            & cache_masks[16 as i32 as usize])
                            as u64
                    };
                    br_safe.cache_avail -= 16 as i32;
                    if ds_safe.block_size == 0 as i32 as u64 {
                        break;
                    }
                    ds_safe.block_bytes_avail = ds_safe.block_size;
                    if ds_safe.block_type as i32 != 3 as i32 {
                        if ds_safe.block_type as i32 == 1 as i32 {
                            ds_safe.state = 11 as i32
                        } else {
                            ds_safe.state = 10 as i32
                        }
                        continue;
                    }
                }
                current_block = 16465730530612695416;
            }
            _ => {}
        }
        match current_block {
            16465730530612695416 =>
            /*
             * Handle an Uncompressed Block.
             */
            /* Skip padding to align following field on
             * 16-bit boundary. */
            {
                if br_safe.cache_avail & 0xf as i32 != 0 {
                    br_safe.cache_avail &= !(0xf as i32)
                } else if br_safe.cache_avail >= 16 as i32
                    || lzx_br_fillup(strm, br) != 0
                    || br_safe.cache_avail >= 16 as i32
                {
                    br_safe.cache_avail -= 16 as i32
                } else {
                    ds_safe.state = 4 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                }
                /* Preparation to read repeated offsets R0,R1 and R2. */
                ds_safe.rbytes_avail = 0 as i32;
                ds_safe.state = 5 as i32;
                current_block = 1724319918354933278;
            }
            14663568441095876955 =>
            /*
             * Read Pre-tree for remaining elements of main tree.
             */
            {
                if lzx_read_pre_tree(strm) == 0 {
                    ds_safe.state = 14 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                } else {
                    if lzx_make_huffman_table(&mut ds_safe.pt) == 0 {
                        break;
                    }
                    ds_safe.loop_0 = 256 as i32
                }
                current_block = 11522940221586662047;
            }
            _ => {}
        }
        match current_block {
            1724319918354933278 => {
                loop
                /* FALL THROUGH */
                {
                    let mut u16: uint16_t = 0;
                    /* Drain bits in the cache buffer of
                     * bit-stream. */
                    if br_safe.cache_avail >= 32 as i32 {
                        u16 = unsafe {
                            ((br_safe.cache_buffer >> br_safe.cache_avail - 16 as i32)
                                as uint32_t
                                & cache_masks[16 as i32 as usize])
                                as uint16_t
                        };
                        br_safe.cache_avail -= 16 as i32;
                        archive_le16enc(ds_safe.rbytes.as_mut_ptr() as *mut (), u16);
                        u16 = unsafe {
                            ((br_safe.cache_buffer >> br_safe.cache_avail - 16 as i32)
                                as uint32_t
                                & cache_masks[16 as i32 as usize])
                                as uint16_t
                        };
                        br_safe.cache_avail -= 16 as i32;
                        unsafe {
                            archive_le16enc(
                                ds_safe
                                    .rbytes
                                    .as_mut_ptr()
                                    .offset(2 as i32 as isize)
                                    as *mut (),
                                u16,
                            );
                        }
                        ds_safe.rbytes_avail = 4 as i32
                    } else if br_safe.cache_avail >= 16 as i32 {
                        u16 = unsafe {
                            ((br_safe.cache_buffer >> br_safe.cache_avail - 16 as i32)
                                as uint32_t
                                & cache_masks[16 as i32 as usize])
                                as uint16_t
                        };
                        br_safe.cache_avail -= 16 as i32;
                        archive_le16enc(ds_safe.rbytes.as_mut_ptr() as *mut (), u16);
                        ds_safe.rbytes_avail = 2 as i32
                    }
                    if ds_safe.rbytes_avail < 4 as i32
                        && ds_safe.br.have_odd as i32 != 0
                    {
                        let fresh6 = ds_safe.rbytes_avail;
                        ds_safe.rbytes_avail = ds_safe.rbytes_avail + 1;
                        ds_safe.rbytes[fresh6 as usize] = ds_safe.br.odd;
                        ds_safe.br.have_odd = 0 as i32 as i8
                    }
                    while ds_safe.rbytes_avail < 4 as i32 {
                        if strm_safe.avail_in <= 0 as i32 as i64 {
                            if last != 0 {
                                break 's_16;
                            }
                            return 0 as i32;
                        } else {
                            let fresh7 = strm_safe.next_in;
                            strm_safe.next_in = unsafe { strm_safe.next_in.offset(1) };
                            let fresh8 = ds_safe.rbytes_avail;
                            ds_safe.rbytes_avail = ds_safe.rbytes_avail + 1;
                            ds_safe.rbytes[fresh8 as usize] = unsafe { *fresh7 };
                            strm_safe.avail_in -= 1
                        }
                    }
                    ds_safe.rbytes_avail = 0 as i32;
                    if ds_safe.state == 5 as i32 {
                        ds_safe.r0 =
                            archive_le32dec(ds_safe.rbytes.as_mut_ptr() as *const ())
                                as i32;
                        if ds_safe.r0 < 0 as i32 {
                            break 's_16;
                        }
                        ds_safe.state = 6 as i32
                    } else if ds_safe.state == 6 as i32 {
                        ds_safe.r1 =
                            archive_le32dec(ds_safe.rbytes.as_mut_ptr() as *const ())
                                as i32;
                        if ds_safe.r1 < 0 as i32 {
                            break 's_16;
                        }
                        ds_safe.state = 7 as i32
                    } else if ds_safe.state == 7 as i32 {
                        ds_safe.r2 =
                            archive_le32dec(ds_safe.rbytes.as_mut_ptr() as *const ())
                                as i32;
                        if ds_safe.r2 < 0 as i32 {
                            break 's_16;
                        }
                        /* We've gotten all repeated offsets. */
                        ds_safe.state = 8 as i32
                    }
                    if !(ds_safe.state != 8 as i32) {
                        break;
                    }
                }
                /* FALL THROUGH */
                current_block = 5023038348526654800;
            }
            11522940221586662047 =>
            /* FALL THROUGH */
            /*
             * Get path lengths of remaining elements of main tree.
             */
            {
                r = lzx_read_bitlen(strm, &mut ds_safe.mt, -(1 as i32));
                if r < 0 as i32 {
                    break;
                }
                if r == 0 {
                    ds_safe.state = 15 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                } else {
                    if lzx_make_huffman_table(&mut ds_safe.mt) == 0 {
                        break;
                    }
                    ds_safe.loop_0 = 0 as i32
                }
                current_block = 10662747035737099349;
            }
            _ => {}
        }
        match current_block {
            5023038348526654800 => {
                /*
                 * Copy bytes form next_in to next_out directly.
                 */
                while ds_safe.block_bytes_avail != 0 {
                    let mut l: i32 = 0;
                    if strm_safe.avail_out <= 0 as i32 as i64 {
                        /* Output buffer is empty. */
                        return 0 as i32;
                    }
                    if strm_safe.avail_in <= 0 as i32 as i64 {
                        /* Input buffer is empty. */
                        if last != 0 {
                            break 's_16;
                        }
                        return 0 as i32;
                    } else {
                        l = ds_safe.block_bytes_avail as i32;
                        if l > ds_safe.w_size - ds_safe.w_pos {
                            l = ds_safe.w_size - ds_safe.w_pos
                        }
                        if l as i64 > strm_safe.avail_out {
                            l = strm_safe.avail_out as i32
                        }
                        if l as i64 > strm_safe.avail_in {
                            l = strm_safe.avail_in as i32
                        }
                        memcpy_safe(
                            strm_safe.next_out as *mut (),
                            strm_safe.next_in as *const (),
                            l as u64,
                        );
                        unsafe {
                            memcpy_safe(
                                &mut *ds_safe.w_buff.offset(ds_safe.w_pos as isize)
                                    as *mut u8
                                    as *mut (),
                                strm_safe.next_in as *const (),
                                l as u64,
                            );
                            strm_safe.next_in = strm_safe.next_in.offset(l as isize);
                            strm_safe.avail_in -= l as i64;
                            strm_safe.next_out = strm_safe.next_out.offset(l as isize);
                        }
                        strm_safe.avail_out -= l as i64;
                        strm_safe.total_out += l as i64;
                        ds_safe.w_pos = ds_safe.w_pos + l & ds_safe.w_mask;
                        ds_safe.block_bytes_avail = (ds_safe.block_bytes_avail as u64)
                            .wrapping_sub(l as u64)
                            as size_t as size_t
                    }
                }
                /* FALL THROUGH */
                current_block = 1130861444095256174;
            }
            10662747035737099349 =>
            /*
             * Read Pre-tree for remaining elements of main tree.
             */
            {
                if unsafe { lzx_read_pre_tree(strm) == 0 } {
                    ds_safe.state = 16 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                } else {
                    if unsafe { lzx_make_huffman_table(&mut ds_safe.pt) == 0 } {
                        break;
                    }
                    ds_safe.loop_0 = 0 as i32
                }
                current_block = 8491080914264407520;
            }
            _ => {}
        }
        match current_block {
            1130861444095256174 =>
            /* Re-align; skip padding byte. */
            {
                if ds_safe.block_size & 1 as i32 as u64 != 0 {
                    if strm_safe.avail_in <= 0 as i32 as i64 {
                        /* Input buffer is empty. */
                        ds_safe.state = 9 as i32;
                        if last != 0 {
                            break;
                        }
                        return 0 as i32;
                    } else {
                        strm_safe.next_in = unsafe { strm_safe.next_in.offset(1) };
                        strm_safe.avail_in -= 1
                    }
                }
                /* This block ended. */
                ds_safe.state = 2 as i32;
                return 1 as i32;
            }
            _ =>
            /* FALL THROUGH */
            /*
             * Get path lengths of remaining elements of main tree.
             */
            {
                r = lzx_read_bitlen(strm, &mut ds_safe.lt, -(1 as i32));
                if r < 0 as i32 {
                    break;
                }
                if r == 0 {
                    ds_safe.state = 17 as i32;
                    if last != 0 {
                        break;
                    }
                    return 0 as i32;
                } else {
                    if lzx_make_huffman_table(&mut ds_safe.lt) == 0 {
                        break;
                    }
                    ds_safe.state = 18 as i32;
                    return 100 as i32;
                }
            }
        }
    }
    ds_safe.error = -(25 as i32);
    return ds_safe.error;
}
unsafe fn lzx_decode_blocks(
    mut strm: *mut lzx_stream,
    mut last: i32,
) -> i32 {
    let mut current_block: u64;
    let strm_safe = unsafe { &mut *strm };
    let mut ds: *mut lzx_dec = strm_safe.ds;
    let ds_safe = unsafe { &mut *ds };
    let mut bre: lzx_br = ds_safe.br;
    let mut at: *mut huffman = &mut ds_safe.at;
    let mut lt: *mut huffman = &mut ds_safe.lt;
    let mut mt: *mut huffman = &mut ds_safe.mt;
    let mut pos_tbl: *const lzx_pos_tbl = ds_safe.pos_tbl;
    let mut noutp: *mut u8 = strm_safe.next_out;
    let mut endp: *mut u8 = unsafe { noutp.offset(strm_safe.avail_out as isize) };
    let mut w_buff: *mut u8 = ds_safe.w_buff;
    let at_safe = unsafe { &mut *at };
    let mut at_bitlen: *mut u8 = at_safe.bitlen;
    let lt_safe = unsafe { &mut *lt };
    let mut lt_bitlen: *mut u8 = lt_safe.bitlen;
    let mt_safe = unsafe { &mut *mt };
    let mut mt_bitlen: *mut u8 = mt_safe.bitlen;
    let mut block_bytes_avail: size_t = ds_safe.block_bytes_avail;
    let mut at_max_bits: i32 = at_safe.max_bits;
    let mut lt_max_bits: i32 = lt_safe.max_bits;
    let mut mt_max_bits: i32 = mt_safe.max_bits;
    let mut c: i32 = 0;
    let mut copy_len: i32 = ds_safe.copy_len;
    let mut copy_pos: i32 = ds_safe.copy_pos;
    let mut w_pos: i32 = ds_safe.w_pos;
    let mut w_mask: i32 = ds_safe.w_mask;
    let mut w_size: i32 = ds_safe.w_size;
    let mut length_header: i32 = ds_safe.length_header;
    let mut offset_bits: i32 = ds_safe.offset_bits;
    let mut position_slot: i32 = ds_safe.position_slot;
    let mut r0: i32 = ds_safe.r0;
    let mut r1: i32 = ds_safe.r1;
    let mut r2: i32 = ds_safe.r2;
    let mut state: i32 = ds_safe.state;
    let mut block_type: i8 = ds_safe.block_type;
    's_73: loop {
        match state {
            18 => {
                current_block = 7149356873433890176;
            }
            19 => {
                current_block = 10531935732394949456;
            }
            20 => {
                current_block = 17539127078321057713;
            }
            21 => {
                current_block = 2144261415468338347;
            }
            22 => {
                current_block = 9521147444787763968;
            }
            _ => {
                continue;
            }
        }
        loop {
            match current_block {
                7149356873433890176 => {
                    if block_bytes_avail == 0 as i32 as u64 {
                        /* This block ended. */
                        ds_safe.state = 2 as i32;
                        ds_safe.br = bre;
                        ds_safe.block_bytes_avail = block_bytes_avail;
                        ds_safe.copy_len = copy_len;
                        ds_safe.copy_pos = copy_pos;
                        ds_safe.length_header = length_header;
                        ds_safe.position_slot = position_slot;
                        ds_safe.r0 = r0;
                        ds_safe.r1 = r1;
                        ds_safe.r2 = r2;
                        ds_safe.w_pos = w_pos;
                        strm_safe.avail_out = unsafe { endp.offset_from(noutp) as i64 };
                        return 1 as i32;
                    }
                    if noutp >= endp {
                        current_block = 5333453573631877616;
                        break 's_73;
                    }
                    if !(bre.cache_avail >= mt_max_bits
                        || lzx_br_fillup(strm, &mut bre) != 0
                        || bre.cache_avail >= mt_max_bits)
                    {
                        if last == 0 {
                            current_block = 5333453573631877616;
                            break 's_73;
                            /* Over read. */
                        }
                        /* Remaining bits are less than
                         * maximum bits(mt.max_bits) but maybe
                         * it still remains as much as we need,
                         * so we should try to use it with
                         * dummy bits. */
                        unsafe {
                            c = lzx_decode_huffman(
                                mt,
                                (bre.cache_buffer << mt_max_bits - bre.cache_avail) as uint32_t
                                    & cache_masks[mt_max_bits as usize],
                            );
                        }
                        bre.cache_avail -= unsafe { *mt_bitlen.offset(c as isize) as i32 };
                        if !(bre.cache_avail >= 0 as i32) {
                            current_block = 17444092441624531628;
                            break 's_73;
                        }
                    } else {
                        unsafe {
                            c = lzx_decode_huffman(
                                mt,
                                (bre.cache_buffer >> bre.cache_avail - mt_max_bits) as uint32_t
                                    & cache_masks[mt_max_bits as usize],
                            );
                        }
                        bre.cache_avail -= unsafe { *mt_bitlen.offset(c as isize) as i32 }
                    }
                    if c > 127 as i32 * 2 as i32 + 1 as i32 {
                        /*
                         * Get a match code, its length and offset.
                         */
                        c -= 127 as i32 * 2 as i32
                            + 1 as i32
                            + 1 as i32;
                        length_header = c & 7 as i32;
                        position_slot = c >> 3 as i32;
                        /* FALL THROUGH */
                        current_block = 10531935732394949456;
                    } else {
                        /*
                         * 'c' is exactly literal code.
                         */
                        /* Save a decoded code to reference it
                         * afterward. */
                        unsafe {
                            *w_buff.offset(w_pos as isize) = c as u8;
                        }
                        w_pos = w_pos + 1 as i32 & w_mask;
                        /* Store the decoded code to output buffer. */
                        let fresh9 = noutp;
                        unsafe {
                            noutp = noutp.offset(1);
                            *fresh9 = c as u8;
                        }
                        block_bytes_avail = block_bytes_avail.wrapping_sub(1);
                        current_block = 7149356873433890176;
                    }
                }
                10531935732394949456 =>
                /*
                 * Get a length.
                 */
                {
                    if length_header == 7 as i32 {
                        if !(bre.cache_avail >= lt_max_bits
                            || lzx_br_fillup(strm, &mut bre) != 0
                            || bre.cache_avail >= lt_max_bits)
                        {
                            if last == 0 {
                                state = 19 as i32;
                                current_block = 5333453573631877616;
                                break 's_73;
                            } else {
                                unsafe {
                                    c = lzx_decode_huffman(
                                        lt,
                                        (bre.cache_buffer << lt_max_bits - bre.cache_avail)
                                            as uint32_t
                                            & cache_masks[lt_max_bits as usize],
                                    );
                                    bre.cache_avail -= *lt_bitlen.offset(c as isize) as i32;
                                }
                                if !(bre.cache_avail >= 0 as i32) {
                                    current_block = 17444092441624531628;
                                    break 's_73;
                                }
                            }
                            /* Over read. */
                        } else {
                            unsafe {
                                c = lzx_decode_huffman(
                                    lt,
                                    (bre.cache_buffer >> bre.cache_avail - lt_max_bits) as uint32_t
                                        & cache_masks[lt_max_bits as usize],
                                );
                                bre.cache_avail -= *lt_bitlen.offset(c as isize) as i32
                            }
                        }
                        copy_len = c + 7 as i32 + 2 as i32
                    } else {
                        copy_len = length_header + 2 as i32
                    }
                    if copy_len as size_t > block_bytes_avail {
                        current_block = 17444092441624531628;
                        break 's_73;
                    }
                    /*
                     * Get an offset.
                     */
                    match position_slot {
                        0 => {
                            /* Use repeated offset 0. */
                            copy_pos = r0;
                            state = 21 as i32;
                            break;
                        }
                        1 => {
                            /* Use repeated offset 1. */
                            copy_pos = r1;
                            /* Swap repeated offset. */
                            r1 = r0;
                            r0 = copy_pos;
                            state = 21 as i32;
                            break;
                        }
                        2 => {
                            /* Use repeated offset 2. */
                            copy_pos = r2;
                            /* Swap repeated offset. */
                            r2 = r0;
                            r0 = copy_pos;
                            state = 21 as i32;
                            break;
                        }
                        _ => {
                            offset_bits =
                                unsafe { (*pos_tbl.offset(position_slot as isize)).footer_bits };
                            /* FALL THROUGH */
                            current_block = 17539127078321057713;
                        }
                    }
                }
                9521147444787763968 =>
                /*
                 * Copy several bytes as extracted data from the window
                 * into the output buffer.
                 */
                {
                    let mut s: *const u8 = 0 as *const u8;
                    let mut l: i32 = 0;
                    l = copy_len;
                    if copy_pos > w_pos {
                        if l > w_size - copy_pos {
                            l = w_size - copy_pos
                        }
                    } else if l > w_size - w_pos {
                        l = w_size - w_pos
                    }
                    unsafe {
                        if noutp.offset(l as isize) >= endp {
                            l = endp.offset_from(noutp) as i64 as i32
                        }
                        s = w_buff.offset(copy_pos as isize);
                    }
                    if l >= 8 as i32 && (copy_pos + l < w_pos || w_pos + l < copy_pos) {
                        unsafe {
                            memcpy_safe(
                                w_buff.offset(w_pos as isize) as *mut (),
                                s as *const (),
                                l as u64,
                            );
                        }
                        memcpy_safe(
                            noutp as *mut (),
                            s as *const (),
                            l as u64,
                        );
                    } else {
                        let mut d: *mut u8 = 0 as *mut u8;
                        let mut li: i32 = 0;
                        d = unsafe { w_buff.offset(w_pos as isize) };
                        li = 0 as i32;
                        while li < l {
                            unsafe {
                                let ref mut fresh10 = *d.offset(li as isize);
                                *fresh10 = *s.offset(li as isize);
                                *noutp.offset(li as isize) = *fresh10;
                            }
                            li += 1
                        }
                    }
                    noutp = unsafe { noutp.offset(l as isize) };
                    copy_pos = copy_pos + l & w_mask;
                    w_pos = w_pos + l & w_mask;
                    block_bytes_avail = (block_bytes_avail as u64)
                        .wrapping_sub(l as u64)
                        as size_t as size_t;
                    if copy_len <= l {
                        /* A copy of current pattern ended. */
                        state = 18 as i32;
                        break;
                    } else {
                        copy_len -= l;
                        if !(noutp >= endp) {
                            current_block = 9521147444787763968;
                            continue;
                        }
                        /* Output buffer is empty. */
                        state = 22 as i32;
                        current_block = 5333453573631877616;
                        break 's_73;
                    }
                }
                2144261415468338347 =>
                /* FALL THROUGH */
                /*
                 * Compute a real position in window.
                 */
                {
                    copy_pos = w_pos - copy_pos & w_mask;
                    /* FALL THROUGH */
                    current_block = 9521147444787763968;
                }
                _ =>
                /*
                 * Get the offset, which is a distance from
                 * current window position.
                 */
                {
                    if block_type as i32 == 2 as i32
                        && offset_bits >= 3 as i32
                    {
                        let mut offbits: i32 = offset_bits - 3 as i32;
                        if !(bre.cache_avail >= offbits
                            || lzx_br_fillup(strm, &mut bre) != 0
                            || bre.cache_avail >= offbits)
                        {
                            state = 20 as i32;
                            if last != 0 {
                                current_block = 17444092441624531628;
                                break 's_73;
                            } else {
                                current_block = 5333453573631877616;
                                break 's_73;
                            }
                        } else {
                            unsafe {
                                copy_pos = (((bre.cache_buffer >> bre.cache_avail - offbits)
                                    as uint32_t
                                    & cache_masks[offbits as usize])
                                    << 3 as i32)
                                    as i32;
                            }
                            /* Get an aligned number. */
                            if !(bre.cache_avail >= offbits + at_max_bits
                                || lzx_br_fillup(strm, &mut bre) != 0
                                || bre.cache_avail >= offbits + at_max_bits)
                            {
                                if last == 0 {
                                    state = 20 as i32;
                                    current_block = 5333453573631877616;
                                    break 's_73;
                                } else {
                                    bre.cache_avail -= offbits;
                                    unsafe {
                                        c = lzx_decode_huffman(
                                            at,
                                            (bre.cache_buffer << at_max_bits - bre.cache_avail)
                                                as uint32_t
                                                & cache_masks[at_max_bits as usize],
                                        );
                                        bre.cache_avail -=
                                            *at_bitlen.offset(c as isize) as i32;
                                    }
                                    if !(bre.cache_avail >= 0 as i32) {
                                        current_block = 17444092441624531628;
                                        break 's_73;
                                    }
                                }
                                /* Over read. */
                            } else {
                                bre.cache_avail -= offbits;
                                unsafe {
                                    c = lzx_decode_huffman(
                                        at,
                                        (bre.cache_buffer >> bre.cache_avail - at_max_bits)
                                            as uint32_t
                                            & cache_masks[at_max_bits as usize],
                                    );
                                    bre.cache_avail -= *at_bitlen.offset(c as isize) as i32
                                }
                            }
                            /* Add an aligned number. */
                            copy_pos += c
                        }
                    } else if !(bre.cache_avail >= offset_bits
                        || lzx_br_fillup(strm, &mut bre) != 0
                        || bre.cache_avail >= offset_bits)
                    {
                        state = 20 as i32;
                        if last != 0 {
                            current_block = 17444092441624531628;
                            break 's_73;
                        } else {
                            current_block = 5333453573631877616;
                            break 's_73;
                        }
                    } else {
                        copy_pos = unsafe {
                            ((bre.cache_buffer >> bre.cache_avail - offset_bits) as uint32_t
                                & cache_masks[offset_bits as usize])
                                as i32
                        };
                        bre.cache_avail -= offset_bits
                    }
                    copy_pos += unsafe {
                        (*pos_tbl.offset(position_slot as isize)).base - 2 as i32
                    };
                    /* Update repeated offset LRU queue. */
                    r2 = r1;
                    r1 = r0;
                    r0 = copy_pos;
                    current_block = 2144261415468338347;
                }
            }
        }
    }
    match current_block {
        17444092441624531628 => {
            ds_safe.error = -(25 as i32);
            return ds_safe.error;
        }
        _ =>
        /* Output buffer is empty. */
        {
            ds_safe.br = bre;
            ds_safe.block_bytes_avail = block_bytes_avail;
            ds_safe.copy_len = copy_len;
            ds_safe.copy_pos = copy_pos;
            ds_safe.length_header = length_header;
            ds_safe.offset_bits = offset_bits;
            ds_safe.position_slot = position_slot;
            ds_safe.r0 = r0;
            ds_safe.r1 = r1;
            ds_safe.r2 = r2;
            ds_safe.state = state;
            ds_safe.w_pos = w_pos;
            strm_safe.avail_out = unsafe { endp.offset_from(noutp) as i64 };
            return 0 as i32;
        }
    };
}
unsafe fn lzx_read_pre_tree(mut strm: *mut lzx_stream) -> i32 {
    let strm_safe = unsafe { &mut *strm };
    let mut ds: *mut lzx_dec = strm_safe.ds;
    let ds_safe = unsafe { &mut *ds };
    let mut br: *mut lzx_br = &mut ds_safe.br;
    let mut i: i32 = 0;
    if ds_safe.loop_0 == 0 as i32 {
        memset_safe(
            ds_safe.pt.freq.as_mut_ptr() as *mut (),
            0 as i32,
            ::std::mem::size_of::<[i32; 17]>() as u64,
        );
    }
    i = ds_safe.loop_0;
    let br_safe = unsafe { &mut *br };
    while i < ds_safe.pt.len_size {
        if !(br_safe.cache_avail >= 4 as i32
            || lzx_br_fillup(strm, br) != 0
            || br_safe.cache_avail >= 4 as i32)
        {
            ds_safe.loop_0 = i;
            return 0 as i32;
        }
        unsafe {
            *(*ds).pt.bitlen.offset(i as isize) =
                (((*br).cache_buffer >> (*br).cache_avail - 4 as i32) as uint32_t
                    & cache_masks[4 as i32 as usize]) as u8;
            (*ds).pt.freq[*(*ds).pt.bitlen.offset(i as isize) as usize] += 1;
        }
        br_safe.cache_avail -= 4 as i32;
        i += 1
    }
    ds_safe.loop_0 = i;
    return 1 as i32;
}
/*
 * Read a bunch of bit-lengths from pre-tree.
 */
unsafe fn lzx_read_bitlen(
    mut strm: *mut lzx_stream,
    mut d: *mut huffman,
    mut end: i32,
) -> i32 {
    let mut current_block: u64;
    let strm_safe = unsafe { &mut *strm };
    let mut ds: *mut lzx_dec = strm_safe.ds;
    let ds_safe = unsafe { &mut *ds };
    let mut br: *mut lzx_br = &mut ds_safe.br;
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ret: i32 = 0;
    let mut same: i32 = 0;
    let mut rbits: u32 = 0;
    i = ds_safe.loop_0;
    let d_safe = unsafe { &mut *d };
    if i == 0 as i32 {
        memset_safe(
            d_safe.freq.as_mut_ptr() as *mut (),
            0 as i32,
            ::std::mem::size_of::<[i32; 17]>() as u64,
        );
    }
    ret = 0 as i32;
    if end < 0 as i32 {
        end = d_safe.len_size
    }
    let br_safe = unsafe { &mut *br };
    loop {
        if !(i < end) {
            current_block = 5141539773904409130;
            break;
        }
        ds_safe.loop_0 = i;
        if !(br_safe.cache_avail >= ds_safe.pt.max_bits
            || lzx_br_fillup(strm, br) != 0
            || br_safe.cache_avail >= ds_safe.pt.max_bits)
        {
            current_block = 15354980847687936399;
            break;
        }
        rbits = unsafe {
            (br_safe.cache_buffer >> br_safe.cache_avail - ds_safe.pt.max_bits) as uint32_t
                & cache_masks[ds_safe.pt.max_bits as usize]
        };
        c = lzx_decode_huffman(&mut ds_safe.pt, rbits);
        match c {
            17 => {
                /* several zero lengths, from 4 to 19. */
                if unsafe {
                    !((*br).cache_avail
                        >= *(*ds).pt.bitlen.offset(c as isize) as i32 + 4 as i32
                        || lzx_br_fillup(strm, br) != 0
                        || (*br).cache_avail
                            >= *(*ds).pt.bitlen.offset(c as isize) as i32
                                + 4 as i32)
                } {
                    current_block = 15354980847687936399; /* Invalid */
                    break;
                }
                unsafe {
                    (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as i32;
                }
                same = unsafe {
                    ((br_safe.cache_buffer >> br_safe.cache_avail - 4 as i32) as uint32_t
                        & cache_masks[4 as i32 as usize])
                        .wrapping_add(4 as i32 as u32)
                        as i32
                };
                if i + same > end {
                    return -(1 as i32);
                }
                br_safe.cache_avail -= 4 as i32;
                j = 0 as i32;
                while j < same {
                    let fresh11 = i;
                    i = i + 1;
                    unsafe {
                        *(*d).bitlen.offset(fresh11 as isize) = 0 as i32 as u8;
                    }
                    j += 1
                }
            }
            18 => {
                /* many zero lengths, from 20 to 51. */
                if unsafe {
                    !((*br).cache_avail
                        >= *(*ds).pt.bitlen.offset(c as isize) as i32 + 5 as i32
                        || lzx_br_fillup(strm, br) != 0
                        || (*br).cache_avail
                            >= *(*ds).pt.bitlen.offset(c as isize) as i32
                                + 5 as i32)
                } {
                    current_block = 15354980847687936399; /* Invalid */
                    break;
                }
                br_safe.cache_avail -=
                    unsafe { *(*ds).pt.bitlen.offset(c as isize) as i32 };
                same = unsafe {
                    ((br_safe.cache_buffer >> br_safe.cache_avail - 5 as i32) as uint32_t
                        & cache_masks[5 as i32 as usize])
                        .wrapping_add(20 as i32 as u32)
                        as i32
                };
                if i + same > end {
                    return -(1 as i32);
                }
                br_safe.cache_avail -= 5 as i32;
                unsafe {
                    memset_safe(
                        (*d).bitlen.offset(i as isize) as *mut (),
                        0 as i32,
                        same as u64,
                    );
                }
                i += same
            }
            19 => {
                /* a few same lengths. */
                if unsafe {
                    !((*br).cache_avail
                        >= *(*ds).pt.bitlen.offset(c as isize) as i32
                            + 1 as i32
                            + (*ds).pt.max_bits
                        || lzx_br_fillup(strm, br) != 0
                        || (*br).cache_avail
                            >= *(*ds).pt.bitlen.offset(c as isize) as i32
                                + 1 as i32
                                + (*ds).pt.max_bits)
                } {
                    current_block = 15354980847687936399; /* Invalid */
                    break; /* Invalid */
                }
                unsafe {
                    (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as i32;
                    same = ((br_safe.cache_buffer >> br_safe.cache_avail - 1 as i32)
                        as uint32_t
                        & cache_masks[1 as i32 as usize])
                        .wrapping_add(4 as i32 as u32)
                        as i32;
                }
                if i + same > end {
                    return -(1 as i32);
                }
                br_safe.cache_avail -= 1 as i32;
                rbits = unsafe {
                    (br_safe.cache_buffer >> br_safe.cache_avail - ds_safe.pt.max_bits) as uint32_t
                        & cache_masks[(*ds).pt.max_bits as usize]
                };
                c = lzx_decode_huffman(&mut ds_safe.pt, rbits);
                unsafe {
                    (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as i32;
                    c = (*(*d).bitlen.offset(i as isize) as i32 - c + 17 as i32)
                        % 17 as i32;
                }
                if c < 0 as i32 {
                    return -(1 as i32);
                }
                j = 0 as i32;
                while j < same {
                    let fresh12 = i;
                    i = i + 1;
                    unsafe {
                        *(*d).bitlen.offset(fresh12 as isize) = c as u8;
                    }
                    j += 1
                }
                d_safe.freq[c as usize] += same
            }
            _ => {
                unsafe {
                    (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as i32;
                    c = (*(*d).bitlen.offset(i as isize) as i32 - c + 17 as i32)
                        % 17 as i32;
                }
                if c < 0 as i32 {
                    return -(1 as i32);
                }
                d_safe.freq[c as usize] += 1;
                let fresh13 = i;
                i = i + 1;
                unsafe { *(*d).bitlen.offset(fresh13 as isize) = c as u8 }
            }
        }
    }
    match current_block {
        5141539773904409130 => ret = 1 as i32,
        _ => {}
    }
    ds_safe.loop_0 = i;
    return ret;
}
unsafe fn lzx_huffman_init(
    mut hf: *mut huffman,
    mut len_size: size_t,
    mut tbl_bits: i32,
) -> i32 {
    let hf_safe = unsafe { &mut *hf };
    if hf_safe.bitlen.is_null() || hf_safe.len_size != len_size as i32 {
        free_safe(hf_safe.bitlen as *mut ());
        hf_safe.bitlen = calloc_safe(
            len_size,
            ::std::mem::size_of::<u8>() as u64,
        ) as *mut u8;
        if hf_safe.bitlen.is_null() {
            return -(30 as i32);
        }
        hf_safe.len_size = len_size as i32
    } else {
        memset_safe(
            hf_safe.bitlen as *mut (),
            0 as i32,
            len_size.wrapping_mul(::std::mem::size_of::<u8>() as u64),
        );
    }
    if hf_safe.tbl.is_null() {
        hf_safe.tbl = malloc_safe(
            ((1 as i32 as size_t) << tbl_bits)
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as u64),
        ) as *mut uint16_t;
        if hf_safe.tbl.is_null() {
            return -(30 as i32);
        }
        hf_safe.tbl_bits = tbl_bits
    }
    return 0 as i32;
}
unsafe fn lzx_huffman_free(mut hf: *mut huffman) {
    let hf_safe = unsafe { &mut *hf };
    free_safe(hf_safe.bitlen as *mut ());
    free_safe(hf_safe.tbl as *mut ());
}
/*
 * Make a huffman coding table.
 */
unsafe fn lzx_make_huffman_table(mut hf: *mut huffman) -> i32 {
    let mut tbl: *mut uint16_t = 0 as *mut uint16_t;
    let mut bitlen: *const u8 = 0 as *const u8;
    let mut bitptn: [i32; 17] = [0; 17];
    let mut weight: [i32; 17] = [0; 17];
    let mut i: i32 = 0;
    let mut maxbits: i32 = 0 as i32;
    let mut ptn: i32 = 0;
    let mut tbl_size: i32 = 0;
    let mut w: i32 = 0;
    let mut len_avail: i32 = 0;
    /*
     * Initialize bit patterns.
     */
    ptn = 0 as i32; /* Invalid */
    i = 1 as i32;
    w = (1 as i32) << 15 as i32;
    let hf_safe = unsafe { &mut *hf };
    while i <= 16 as i32 {
        bitptn[i as usize] = ptn;
        weight[i as usize] = w;
        if hf_safe.freq[i as usize] != 0 {
            ptn += hf_safe.freq[i as usize] * w;
            maxbits = i
        }
        i += 1;
        w >>= 1 as i32
    }
    if ptn & 0xffff as i32 != 0 as i32 || maxbits > hf_safe.tbl_bits {
        return 0 as i32;
    }
    hf_safe.max_bits = maxbits;
    /*
     * Cut out extra bits which we won't house in the table.
     * This preparation reduces the same calculation in the for-loop
     * making the table.
     */
    if maxbits < 16 as i32 {
        let mut ebits: i32 = 16 as i32 - maxbits;
        i = 1 as i32;
        while i <= maxbits {
            bitptn[i as usize] >>= ebits;
            weight[i as usize] >>= ebits;
            i += 1
        }
    }
    /*
     * Make the table.
     */
    tbl_size = (1 as i32) << hf_safe.tbl_bits;
    tbl = hf_safe.tbl;
    bitlen = hf_safe.bitlen;
    len_avail = hf_safe.len_size;
    hf_safe.tree_used = 0 as i32;
    i = 0 as i32;
    while i < len_avail {
        let mut p: *mut uint16_t = 0 as *mut uint16_t;
        let mut len: i32 = 0;
        let mut cnt: i32 = 0;
        if unsafe { !(*bitlen.offset(i as isize) as i32 == 0 as i32) } {
            /* Get a bit pattern */
            len = unsafe { *bitlen.offset(i as isize) as i32 };
            if len > tbl_size {
                return 0 as i32;
            }
            ptn = bitptn[len as usize];
            cnt = weight[len as usize];
            /* Calculate next bit pattern */
            bitptn[len as usize] = ptn + cnt; /* Invalid */
            if bitptn[len as usize] > tbl_size {
                return 0 as i32;
            }
            /* Update the table */
            p = unsafe { &mut *tbl.offset(ptn as isize) as *mut uint16_t };
            loop {
                cnt -= 1;
                if !(cnt >= 0 as i32) {
                    break;
                }
                unsafe { *p.offset(cnt as isize) = i as uint16_t }
            }
        }
        i += 1
    }
    return 1 as i32;
}
#[inline]
unsafe fn lzx_decode_huffman(
    mut hf: *mut huffman,
    mut rbits: u32,
) -> i32 {
    let mut c: i32 = 0;
    c = unsafe { *(*hf).tbl.offset(rbits as isize) as i32 };
    let hf_safe = unsafe { &mut *hf };
    if c < hf_safe.len_size {
        return c;
    }
    return 0 as i32;
}

#[no_mangle]
pub unsafe fn archive_test_cab_skip_sfx(mut _a: *mut archive) {
    let mut a: *mut archive_read = _a as *mut archive_read;
    cab_skip_sfx(a);
    let mut archive_read_filter: *mut archive_read_filter = 0 as *mut archive_read_filter;
    archive_read_filter = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<archive_read_filter>() as u64,
        )
    } as *mut archive_read_filter;
    (*archive_read_filter).fatal = 'a' as i8;
    (*a).filter = archive_read_filter as *mut archive_read_filter;
    cab_skip_sfx(a);
}

#[no_mangle]
pub unsafe fn archive_test_lzx_br_fixup() {
    let mut lzx_stream: *mut lzx_stream = 0 as *mut lzx_stream;
    lzx_stream = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<lzx_stream>() as u64,
        )
    } as *mut lzx_stream;
    let mut lzx_br: *mut lzx_br = 0 as *mut lzx_br;
    lzx_br = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<lzx_br>() as u64,
        )
    } as *mut lzx_br;
    (*lzx_stream).avail_in = 1 as int64_t;
    (*lzx_br).have_odd = '1' as i8;
    (*lzx_br).cache_avail = 1 as i32;
    lzx_br_fixup(lzx_stream, lzx_br);
}

#[no_mangle]
pub unsafe fn archive_test_lzx_read_blocks() {
    let mut strm: *mut lzx_stream = 0 as *mut lzx_stream;
    strm = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<lzx_stream>() as u64,
        )
    } as *mut lzx_stream;
    let mut lzx_dec: *mut lzx_dec = 0 as *mut lzx_dec;
    lzx_dec = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<lzx_dec>() as u64,
        )
    } as *mut lzx_dec;
    (*lzx_dec).br.cache_avail = 0 as i32;
    (*lzx_dec).state = ARCHIVE_CAB_DEFINED_PARAM.st_copy_uncomp1;
    (*lzx_dec).block_bytes_avail = 20 as size_t;
    (*strm).avail_out = 1 as int64_t;
    (*strm).avail_in = 0 as int64_t;
    (*strm).ds = lzx_dec as *mut lzx_dec;
    lzx_read_blocks(strm, 1);
    lzx_read_blocks(strm, 0);
    (*lzx_dec).block_bytes_avail = 4 as size_t;
    (*strm).avail_out = 2 as int64_t;
    (*strm).avail_in = 1 as int64_t;
    let mut p1: [u8; 4] = [
        '1' as i32 as u8,
        '2' as i32 as u8,
        '3' as i32 as u8,
        '4' as i32 as u8,
    ];
    let mut p2: [u8; 4] = [
        '1' as i32 as u8,
        '2' as i32 as u8,
        '3' as i32 as u8,
        '4' as i32 as u8,
    ];
    (*strm).next_out =
        &p1 as *const [u8; 4] as *mut [u8; 4] as *mut u8;
    (*strm).next_in =
        &p2 as *const [u8; 4] as *mut [u8; 4] as *const u8;
    let mut p: [u8; 4] = [
        '1' as i32 as u8,
        '2' as i32 as u8,
        '3' as i32 as u8,
        '4' as i32 as u8,
    ];
    (*lzx_dec).w_buff =
        &p as *const [u8; 4] as *mut [u8; 4] as *mut u8;
    (*lzx_dec).w_pos = 1 as i32;
    (*lzx_dec).w_size = 4 as i32;
    (*lzx_dec).w_mask = 1 as i32;
    (*strm).ds = lzx_dec as *mut lzx_dec;
    lzx_read_blocks(strm, 1);
    (*lzx_dec).state = ARCHIVE_CAB_DEFINED_PARAM.st_copy_uncomp2;
    (*lzx_dec).block_size = 1 as size_t;
    (*strm).avail_in = 0 as int64_t;
    (*strm).ds = lzx_dec as *mut lzx_dec;
    lzx_read_blocks(strm, 1);
    (*strm).avail_in = 1 as int64_t;
    (*strm).ds = lzx_dec as *mut lzx_dec;
    lzx_read_blocks(strm, 1);
    (*lzx_dec).block_size = 2 as size_t;
    (*strm).ds = lzx_dec as *mut lzx_dec;
    lzx_read_blocks(strm, 1);
    (*lzx_dec).state = ARCHIVE_CAB_DEFINED_PARAM.st_rd_r2;
    (*lzx_dec).br.cache_avail = 33 as i32;
    (*strm).ds = lzx_dec as *mut lzx_dec;
    lzx_read_blocks(strm, 1);
    (*lzx_dec).br.cache_avail = 17 as i32;
    (*strm).ds = lzx_dec as *mut lzx_dec;
    lzx_read_blocks(strm, 1);
    (*lzx_dec).br.cache_avail = 15 as i32;
    (*lzx_dec).rbytes_avail = 2 as i32;
    (*lzx_dec).br.have_odd = 'a' as i8;
    (*strm).ds = lzx_dec as *mut lzx_dec;
    lzx_read_blocks(strm, 1);
    (*lzx_dec).state = ARCHIVE_CAB_DEFINED_PARAM.st_rd_r0;
    lzx_read_blocks(strm, 1);
    (*lzx_dec).state = ARCHIVE_CAB_DEFINED_PARAM.st_rd_r1;
    lzx_read_blocks(strm, 1);
    (*lzx_dec).state = ARCHIVE_CAB_DEFINED_PARAM.st_rd_alignment;
    (*strm).ds = lzx_dec as *mut lzx_dec;
    lzx_read_blocks(strm, 1);
}

#[no_mangle]
pub unsafe fn archive_test_lzx_br_fillup() {
    let mut lzx_stream: *mut lzx_stream = 0 as *mut lzx_stream;
    lzx_stream = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<lzx_stream>() as u64,
        )
    } as *mut lzx_stream;
    let mut lzx_br: *mut lzx_br = 0 as *mut lzx_br;
    lzx_br = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<lzx_br>() as u64,
        )
    } as *mut lzx_br;
    (*lzx_br).cache_avail = 1 as i32;
    (*lzx_stream).avail_in = 1 as int64_t;
}

#[no_mangle]
pub unsafe fn archive_test_archive_read_support_format_cab() {
    let mut archive_read: *mut archive_read = 0 as *mut archive_read;
    archive_read = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<archive_read>() as u64,
        )
    } as *mut archive_read;
    (*archive_read).archive.magic = ARCHIVE_AR_DEFINED_PARAM.archive_read_magic;
    (*archive_read).archive.state = ARCHIVE_AR_DEFINED_PARAM.archive_state_new;
    archive_read_support_format_cab(&mut (*archive_read).archive as *mut archive);
}

#[no_mangle]
pub unsafe fn archive_test_archive_read_format_cab_options(mut _a: *mut archive) {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut cab: *mut cab = 0 as *mut cab;
    cab = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cab>() as u64,
        )
    } as *mut cab;
    (*(*a).format).data = cab as *mut ();
    archive_read_format_cab_options(
        a,
        b"hdrcharset\x00" as *const u8 as *const i8,
        b"h\x00" as *const u8 as *const i8,
    );
}

#[no_mangle]
pub unsafe fn archive_test_cab_read_data(mut _a: *mut archive) {
    let mut size: size_t = 0;
    let mut size2: *mut size_t = &size as *const size_t as *mut size_t;
    let mut offset: int64_t = 0;
    let mut offset2: *mut int64_t = &offset as *const int64_t as *mut int64_t;
    let mut buff: *mut () = 0 as *const () as *mut ();
    let mut buff2: *mut *const () = unsafe {
        &buff as *const *mut () as *mut *mut () as *mut *const ()
    };
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut cab: *mut cab = 0 as *mut cab;
    cab = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cab>() as u64,
        )
    } as *mut cab;
    (*cab).entry_bytes_remaining = 0 as int64_t;
    (*(*a).format).data = cab as *mut ();
    cab_read_data(a, buff2, size2, offset2);
}

#[no_mangle]
pub unsafe fn archive_test_cab_consume_cfdata(mut _a: *mut archive) {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut cab: *mut cab = 0 as *mut cab;
    cab = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cab>() as u64,
        )
    } as *mut cab;
    let mut cffolder: *mut cffolder = 0 as *mut cffolder;
    cffolder = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cffolder>() as u64,
        )
    } as *mut cffolder;
    let mut cffile: *mut cffile = 0 as *mut cffile;
    cffile = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cffile>() as u64,
        )
    } as *mut cffile;
    (*cffile).folder = 0xFFFD;
    (*cab).entry_cffile = cffile;
    (*cffolder).comptype = 0x0000;
    (*cab).entry_cffolder = cffolder;
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    cfdata = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cfdata>() as u64,
        )
    } as *mut cfdata;
    (*cab).entry_cfdata = cfdata;
    (*cfdata).unconsumed = 0;
    (*cfdata).compressed_size = 1;
    (*cfdata).uncompressed_bytes_remaining = 10;
    (*(*a).format).data = cab as *mut ();
    cab_consume_cfdata(a, 20);
    (*cfdata).uncompressed_bytes_remaining = 40;
    cab_consume_cfdata(a, 20);
}

#[no_mangle]
pub unsafe fn archive_test_archive_read_format_cab_read_data(mut _a: *mut archive) {
    let mut size: size_t = 0;
    let mut size2: *mut size_t = &size as *const size_t as *mut size_t;
    let mut offset: int64_t = 0;
    let mut offset2: *mut int64_t = &offset as *const int64_t as *mut int64_t;
    let mut buff: *mut () = 0 as *const () as *mut ();
    let mut buff2: *mut *const () = unsafe {
        &buff as *const *mut () as *mut *mut () as *mut *const ()
    };
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut cab: *mut cab = 0 as *mut cab;
    cab = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cab>() as u64,
        )
    } as *mut cab;
    (*(*a).format).data = cab as *mut ();
    let mut cffile: *mut cffile = 0 as *mut cffile;
    cffile = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cffile>() as u64,
        )
    } as *mut cffile;
    (*cffile).folder = 0xFFFF;
    (*cab).entry_cffile = cffile;
    archive_read_format_cab_read_data(a, buff2, size2, offset2);
}

#[no_mangle]
pub unsafe fn archive_test_cab_next_cfdata(mut _a: *mut archive) {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut cab: *mut cab = 0 as *mut cab;
    cab = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cab>() as u64,
        )
    } as *mut cab;
    (*(*a).format).data = cab as *mut ();
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    cfdata = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cfdata>() as u64,
        )
    } as *mut cfdata;
    (*cab).entry_cfdata = cfdata;
    let mut cffolder: *mut cffolder = 0 as *mut cffolder;
    cffolder = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cffolder>() as u64,
        )
    } as *mut cffolder;
    (*cffolder).cfdata_index = 1;
    (*cffolder).cfdata_count = 1;
    (*cab).entry_cffolder = cffolder;
    cab_next_cfdata(a);
}

#[no_mangle]
pub unsafe fn archive_test_cab_checksum_update(mut _a: *mut archive) {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut cab: *mut cab = 0 as *mut cab;
    cab = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cab>() as u64,
        )
    } as *mut cab;
    (*(*a).format).data = cab as *mut ();
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    cfdata = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cfdata>() as u64,
        )
    } as *mut cfdata;
    (*cab).entry_cfdata = cfdata;
    (*cfdata).sum = 1;
    let mut cffolder: *mut cffolder = 0 as *mut cffolder;
    cffolder = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cffolder>() as u64,
        )
    } as *mut cffolder;
    (*cffolder).cfdata_index = 1;
    (*cffolder).cfdata_count = 1;
    (*cab).entry_cffolder = cffolder;
    cab_next_cfdata(a);
    let mut p: *mut i8 = b"hdrcharset\x00" as *const u8 as *mut i8;
    (*cfdata).sum_ptr = p as *mut ();
    (*cfdata).sum_extra_avail = 3;
    cab_checksum_update(a, 4);
}

#[no_mangle]
unsafe fn archive_test_cab_checksum_cfdata(
    mut p: *const (),
    mut bytes: size_t,
    mut seed: uint32_t,
) {
    cab_checksum_cfdata(p, bytes, seed);
}

#[no_mangle]
unsafe fn archive_test_lzx_huffman_init(
    mut len_size: size_t,
    mut tbl_bits: i32,
) {
    let mut huffman: *mut huffman = 0 as *mut huffman;
    huffman = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<huffman>() as u64,
        )
    } as *mut huffman;
    (*huffman).len_size = 1;
    let mut bitlen: *mut u8 = b"abc\x00" as *const u8 as *mut u8;
    (*huffman).bitlen = bitlen as *mut u8;
    lzx_huffman_init(huffman, len_size, tbl_bits);
}

#[no_mangle]
pub unsafe fn archive_test_cab_read_ahead_cfdata_none(mut _a: *mut archive) {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut cab: *mut cab = 0 as *mut cab;
    cab = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cab>() as u64,
        )
    } as *mut cab;
    (*(*a).format).data = cab as *mut ();
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    cfdata = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<cfdata>() as u64,
        )
    } as *mut cfdata;
    (*cab).entry_cfdata = cfdata;
    (*(*a).format).data = cab as *mut ();
    let mut archive_read_filter: *mut archive_read_filter = 0 as *mut archive_read_filter;
    archive_read_filter = unsafe {
        calloc_safe(
            1 as i32 as u64,
            ::std::mem::size_of::<archive_read_filter>() as u64,
        )
    } as *mut archive_read_filter;
    (*archive_read_filter).fatal = 'a' as i8;
    (*a).filter = archive_read_filter as *mut archive_read_filter;
    let mut availp: ssize_t = 1;
    let mut avail: *mut ssize_t = &availp as *const ssize_t as *mut ssize_t;
    cab_read_ahead_cfdata_none(a, avail);
}
