//! Contains ABI definitions that allow Libcw to be called from The C
//! Programming Language
//!
//! # Warning
//!
//! This module is only available if the feature `tokio` is disabled
use std::ffi::CStr;
use crate::{Parser, Stats as LibStats};
use std::fs::File;
use std::io::BufReader;
use std::os::raw::{c_char, c_uchar};
use std::os::raw::c_ulong;
use crate::config::{Encoding, LineBreak};


/// ABI representation of [Stats](crate::Stats)
///
/// # Example
///
/// ```c
/// Stats * stats = new_stats();
/// printf("%i",stats -> lines);
/// ```
#[repr(C)]
#[derive(Default)]
pub struct Stats {
    lines: c_ulong,
    words: c_ulong,
    characters: c_ulong,
    bytes: c_ulong,
    length: c_ulong,
}

impl From<LibStats> for Stats {
    fn from(s: LibStats) -> Self {
        Stats {
            lines: s.lines().unwrap_or_default() as c_ulong,
            words: s.words().unwrap_or_default() as c_ulong,
            characters: s.characters().unwrap_or_default() as c_ulong,
            bytes: s.bytes().unwrap_or_default() as c_ulong,
            length: s.length().unwrap_or_default() as c_ulong,
        }
    }
}
/// Creates a new Stats instance and returns its pointer
///
/// ```c
/// Stats * stats = new_stats();
/// printf("%i",stats -> lines);
/// ```
///
/// # Unsafe
///
/// This method uses unsafe pointers that leak memory (required by the ABI
/// interface). To free up a Stats instance, use
/// [destroy_stats](crate::c::destroy_stats)
#[no_mangle]
pub unsafe extern "C" fn new_stats()->*mut Stats {
    let stats = Box::new(Default::default());
    Box::into_raw(stats)
}
/// Destroys a Stats instance and frees its memory
///
/// ```c
/// Stats * stats = new_stats();
/// destroy_stats(&stats);
/// // stats == NULL
/// ```
///
/// # Warning
///
/// The received pointer will point to `NULL`
#[no_mangle]
pub unsafe extern "C" fn destroy_stats(stats:*mut *mut Stats){
    if (*stats).is_null() {return;}
    let _ = Box::from_raw(*stats);
    let null = std::ptr::null_mut();
    *stats = null;
}

/// Creates a new Parser instance and returns its pointer. For more information
/// read [Parser::new](crate::Parser::new)
///
/// ```c
/// Parser * parser = new_parser(UTF8,LF,true,true,true,true,true);
/// ```
///
/// # Unsafe
///
/// This method uses unsafe pointers that leak memory (required by the ABI
/// interface). To free up a Stats instance, use
/// [destroy_parser](crate::c::destroy_parser)
#[no_mangle]
pub unsafe extern "C" fn new_parser(
    encoding:Encoding,
    linebreak:LineBreak,
    lines:bool,
    words:bool,
    chars:bool,
    bytes:bool,
    max_length:bool,
) -> *mut Parser {
    let parser = Parser::new(encoding, linebreak, lines, words, chars, bytes, max_length);
    let allocated_parser = Box::new(parser);
    Box::into_raw(allocated_parser)
}
/// Destroys a Parser instance and frees its memory
///
/// ```c
/// Parser * parser = new_parser(UTF8,LF,true,true,true,true,true);
/// destroy_stats(&parser);
/// // parser == NULL
/// ```
///
/// # Warning
///
/// The received pointer will point to `NULL`
#[no_mangle]
pub unsafe extern "C" fn destroy_parser(parser: *mut *mut Parser) {
    if (*parser).is_null() {return;}
    let _ = Box::from_raw(*parser);
    let null:*mut Parser = std::ptr::null_mut();
    *parser = null;
}

/// Runs the parser over a file
///
/// # Params
/// - parser: A valid parser instance
/// - path: A valid path to a file
/// - out: A pointer to a valid stats instance
///
/// # Successful codes
///
/// - Code 0: The file was correctly parsed
///
/// # Error codes
///
/// - Code -1: Parser is null
/// - Code -2: Stats is null
/// - Code -3: The received string is not a valid Rust String Slice (see [str](std::str))
/// - Code -4: The file couldn't be opened
/// - Code -5: The parser couldn't read the file
#[no_mangle]
pub unsafe extern "C" fn process_file(parser: *const Parser, path:*const c_char,out:*mut Stats) -> c_char {
    if parser.is_null() {return -1};
    if out.is_null() {return -2};

    let cstr = CStr::from_ptr(path);
    let result = cstr.to_str();
    if result.is_err() {return -3};

    let str = result.unwrap();
    let result = File::open(str);
    if result.is_err() {return -4};

    let file = result.unwrap();
    let reader = BufReader::new(file);
    let result = (*parser).process(reader);
    if result.is_err() {return -5};

    *out = result.unwrap().into();
    0
}

/// Runs the parser over an array slice
///
/// # Params
/// - parser: A valid parser instance
/// - path: A pointer to the where the array slice starts
/// - size: The size of the array slice
/// - out: A pointer to a valid stats instance
///
/// # Successful codes
///
/// - Code 0: The slice was correctly parsed
///
/// # Error codes
///
/// - Code -1: Parser is null
/// - Code -2: Stats is null
/// - Code -5: The parser couldn't read the slice
#[no_mangle]
pub unsafe extern "C" fn process_slice(parser:*const Parser, ptr:*const c_uchar,size:c_ulong,out:*mut Stats) -> c_char {
    if parser.is_null() {return -1};
    if out.is_null() {return -2};

    let slice = std::slice::from_raw_parts(ptr,size as usize);
    let result = (*parser).process(slice);
    if result.is_err() {return -5};
    *out = result.unwrap().into();
    0
}
/// Runs the parser over a string
///
/// # Params
/// - parser: A valid parser instance
/// - ptr: A valid string
/// - out: A pointer to a valid stats instance
///
/// # Successful codes
///
/// - Code 0: The file was correctly parsed
///
/// # Error codes
///
/// - Code -1: Parser is null
/// - Code -2: Stats is null
/// - Code -5: The parser couldn't read the string
#[no_mangle]
pub unsafe extern "C" fn process_string(parser:*const Parser,ptr:*const c_char,out:*mut Stats) -> c_char {
    if parser.is_null() {return -1};
    if out.is_null() {return -2};

    let c_str = CStr::from_ptr(ptr);

    let result = (*parser).process(c_str.to_bytes());
    if result.is_err() {return -5};
    *out = result.unwrap().into();
    0
}

