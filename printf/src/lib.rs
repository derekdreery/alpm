//! This crate provides a method to convert printf-style calls to a rust formatter
extern crate libc;
extern crate va_list;
#[macro_use] extern crate nom;

use std::io::Write;
use std::ffi::CStr;
use std::fmt;

use libc as c;
use va_list::VaList;

mod parser;
use parser::*;

/// Take a printf c-string and variadic array, and write equiv. out to the formatter
///
/// # Safety
/// This function is UB if the va_list doesn't match the format (c printf syntax)
///
/// There must be no panics in this function, so quite often errors are deliberately ignored
pub unsafe fn printf<W: Write>(mut w: &mut W,
                               format: *const c::c_char,
                               mut args: VaList)
{
    let format_str = CStr::from_ptr(format).to_bytes();
    let format = parse_format_str(format_str);

    let format = match format {
        nom::IResult::Done(_, out) => out,
        _ => { return; }
    };

    for tok in format {
        match tok {
            Token::Literal(val) => w.write_all(&[val]).unwrap_or(()), // can't handle error
            Token::Placeholder {
                right_align: right_align,
                preceding_plus: preceding_plus,
                pad_sign: pad_sign,
                prefix: prefix,
                pad_zero: pad_zero,
                width: width,
                precision: precision,
                modifier: modifier,
                token_type: token_type,
            } => {
                let width = width.map(|width| match width {
                    Width::Static(w) => w,
                    Width::Dynamic => args.get::<c::c_int>() as usize,
                });
                let precision = precision.map(|width| match width {
                    Width::Static(w) => w,
                    Width::Dynamic => args.get::<c::c_int>() as usize,
                });
                let num: Vec<u8> = match (token_type, modifier) {
                    (TokenType::SignedInteger, Modifier::Char)
                        => get_formatted::<c::c_char>(&mut args).into_bytes(),
                    (TokenType::SignedInteger, Modifier::Short)
                        => get_formatted::<c::c_short>(&mut args).into_bytes(),
                    (TokenType::SignedInteger, Modifier::Long)
                        => get_formatted::<c::c_long>(&mut args).into_bytes(),
                    (TokenType::SignedInteger, Modifier::LongLong)
                        => get_formatted::<c::c_longlong>(&mut args).into_bytes(),
                    (TokenType::SignedInteger, Modifier::IntMax)
                        => get_formatted::<c::intmax_t>(&mut args).into_bytes(),
                    (TokenType::SignedInteger, Modifier::Size)
                        => get_formatted::<c::size_t>(&mut args).into_bytes(),
                    (TokenType::SignedInteger, Modifier::PtrDiff)
                        => get_formatted::<c::ptrdiff_t>(&mut args).into_bytes(),
                    (TokenType::SignedInteger, _)
                        => get_formatted::<c::c_int>(&mut args).into_bytes(),

                    (TokenType::UnsignedInteger, Modifier::Char)
                        => get_formatted::<c::c_uchar>(&mut args).into_bytes(),
                    (TokenType::UnsignedOctal, Modifier::Char)
                        => get_formatted_octal::<c::c_uchar>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHex, Modifier::Char)
                        => get_formatted_hex::<c::c_uchar>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHexUpper, Modifier::Char)
                        => get_formatted_hexupper::<c::c_uchar>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedInteger, Modifier::Short)
                        => get_formatted::<c::c_ushort>(&mut args).into_bytes(),
                    (TokenType::UnsignedOctal, Modifier::Short)
                        => get_formatted_octal::<c::c_ushort>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHex, Modifier::Short)
                        => get_formatted_hex::<c::c_ushort>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHexUpper, Modifier::Short)
                        => get_formatted_hexupper::<c::c_ushort>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedInteger, Modifier::Long)
                        => get_formatted::<c::c_ulong>(&mut args).into_bytes(),
                    (TokenType::UnsignedOctal, Modifier::Long)
                        => get_formatted_octal::<c::c_ulong>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHex, Modifier::Long)
                        => get_formatted_hex::<c::c_ulong>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHexUpper, Modifier::Long)
                        => get_formatted_hexupper::<c::c_ulong>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedInteger, Modifier::LongLong)
                        => get_formatted::<c::c_ulonglong>(&mut args).into_bytes(),
                    (TokenType::UnsignedOctal, Modifier::LongLong)
                        => get_formatted_octal::<c::c_ulonglong>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHex, Modifier::LongLong)
                        => get_formatted_hex::<c::c_ulonglong>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHexUpper, Modifier::LongLong)
                        => get_formatted_hexupper::<c::c_ulonglong>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedInteger, Modifier::IntMax)
                        => get_formatted::<c::uintmax_t>(&mut args).into_bytes(),
                    (TokenType::UnsignedOctal, Modifier::IntMax)
                        => get_formatted_octal::<c::uintmax_t>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHex, Modifier::IntMax)
                        => get_formatted_hex::<c::uintmax_t>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHexUpper, Modifier::IntMax)
                        => get_formatted_hexupper::<c::uintmax_t>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedInteger, Modifier::Size)
                        => get_formatted::<c::size_t>(&mut args).into_bytes(),
                    (TokenType::UnsignedOctal, Modifier::Size)
                        => get_formatted_octal::<c::size_t>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHex, Modifier::Size)
                        => get_formatted_hex::<c::size_t>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHexUpper, Modifier::Size)
                        => get_formatted_hexupper::<c::size_t>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedInteger, Modifier::PtrDiff)
                        => get_formatted::<c::ptrdiff_t>(&mut args).into_bytes(),
                    (TokenType::UnsignedOctal, Modifier::PtrDiff)
                        => get_formatted_octal::<c::ptrdiff_t>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHex, Modifier::PtrDiff)
                        => get_formatted_hex::<c::ptrdiff_t>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHexUpper, Modifier::PtrDiff)
                        => get_formatted_hexupper::<c::ptrdiff_t>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedInteger, _)
                        => get_formatted::<c::c_uint>(&mut args).into_bytes(),
                    (TokenType::UnsignedOctal, _)
                        => get_formatted_octal::<c::c_uint>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHex, _)
                        => get_formatted_hex::<c::c_uint>(&mut args, prefix).into_bytes(),
                    (TokenType::UnsignedHexUpper, _)
                        => get_formatted_hexupper::<c::c_uint>(&mut args, prefix).into_bytes(),

                    // TODO c_longdouble is not supported yet
                    (TokenType::Float, _) =>
                        get_formatted_double::<c::c_double>(&mut args, precision).into_bytes(),
                    (TokenType::FloatUpper, _) =>
                        get_formatted_double::<c::c_double>(&mut args, precision).into_bytes(),
                    (TokenType::Scientific, _) =>
                        get_formatted_double::<c::c_double>(&mut args, precision).into_bytes(),
                    (TokenType::ScientificUpper, _) =>
                        get_formatted_double::<c::c_double>(&mut args, precision).into_bytes(),
                    (TokenType::ShortestFloat, _) =>
                        get_formatted_double::<c::c_double>(&mut args, precision).into_bytes(),
                    (TokenType::ShortestFloatUpper, _) =>
                        get_formatted_double::<c::c_double>(&mut args, precision).into_bytes(),
                    (TokenType::HexFloat, _) =>
                        get_formatted_double::<c::c_double>(&mut args, precision).into_bytes(),
                    (TokenType::HexFloatUpper, _) =>
                        get_formatted_double::<c::c_double>(&mut args, precision).into_bytes(),

                    (TokenType::Character, _) => vec![args.get::<i8>() as u8],
                    (TokenType::String, _) => vec![],
                    (TokenType::PointerAddress, _) => vec![],
                    (TokenType::StoreChars, _) => vec![],
                    (TokenType::PercentLiteral, _) => vec![],
                };
                write_stuff_before_number(&mut w,
                                          token_type,
                                          num.len(),
                                          num.get(0).map(|v| *v) == Some(b'-'),
                                          width,
                                          right_align,
                                          preceding_plus,
                                          pad_sign,
                                          prefix,
                                          pad_zero);
                write(&mut w, &num[..]);
            },
            Token::Error => {
                // give up
                return;
            }
        }
    }
    //write!(w, "{:?}", format)
}

/// Responsible for writing the text before the actual number. Needs to know the length of
/// chars of the actual number
fn write_stuff_before_number<W: Write>(mut w: &mut W,
                                       token_type: TokenType,
                                       len: usize,
                                       is_negative: bool,
                                       width: Option<usize>,
                                       right_align: bool,
                                       preceding_plus: bool,
                                       pad_sign: bool,
                                       prefix: bool,
                                       pad_zero: bool)
{
    if is_negative {
        write(&mut w, b"-");
    } else {
        if preceding_plus {
            write(&mut w, b"+");
        } else if pad_sign {
            write(&mut w, b" ");
        } else {
            // do nothing
        }
    }
    if let Some(width) = width {
        if width > len {
            for _ in 0..(width - len) {
                if pad_zero {
                    write(&mut w, b"0");
                } else {
                    write(&mut w, b" ");
                }
            }
        }
    }
}

/// helper function to write byte slice
#[inline(always)]
fn write<W: Write>(mut w: &mut W, i: &[u8]) {
    w.write_all(&i[..]).unwrap_or(());
}

/// Get a value formatted as a string with the given number of decimal points
unsafe fn get_formatted<T>(args: &mut VaList) -> String
    where T: va_list::VaPrimitive + fmt::Display
{
    let t = args.get::<T>();
    format!("{:}", t)
}

unsafe fn get_formatted_octal<T>(args: &mut VaList, prefix: bool) -> String
    where T: va_list::VaPrimitive + fmt::Octal
{
    let t = args.get::<T>();

    if prefix {
        // rust adds 'o' between '0' and num, c doesn't, so can't use '#' in format str
        let mut out = format!("{:o}", t);
        out.insert(0, '0');
        out
    } else {
        format!("{:o}", t)
    }
}

unsafe fn get_formatted_hex<T>(args: &mut VaList, prefix: bool) -> String
    where T: va_list::VaPrimitive + fmt::LowerHex
{
    let t = args.get::<T>();

    match prefix {
        true => format!("{:#x}", t),
        false => format!("{:x}", t),
    }
}

unsafe fn get_formatted_hexupper<T>(args: &mut VaList, prefix: bool) -> String
    where T: va_list::VaPrimitive + fmt::UpperHex
{
    let t = args.get::<T>();

    match prefix {
        true => format!("{:#X}", t),
        false => format!("{:X}", t),
    }
}

unsafe fn get_formatted_double<T>(args: &mut VaList, precision: Option<usize>) -> String
    where T: va_list::VaPrimitive + fmt::Display
{
    let t = args.get::<T>();

    match precision {
        Some(width) => format!("{:.*}", width, t),
        None => format!("{:}", t),
    }
}
