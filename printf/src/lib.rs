//! This crate provides a method to convert printf-style calls to a rust formatter
extern crate libc;
extern crate va_list;
#[macro_use] extern crate nom;

use std::io::Write;
use std::ffi::CStr;
use std::fmt;

use libc::{c_char, c_short, c_int, c_long, c_longlong, intmax_t, size_t, ptrdiff_t};
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
                               format: *const c_char,
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
            } => match token_type {
                TokenType::SignedInteger => {
                    let width = width.map(|width| match width {
                        Width::Static(w) => w,
                        Width::Dynamic => args.get::<c_int>() as usize,
                    });
                    let num = match modifier {
                        Modifier::Char => get_formatted::<c_char>(&mut args, width),
                        Modifier::Short => get_formatted::<c_short>(&mut args, width),
                        Modifier::Long => get_formatted::<c_long>(&mut args, width),
                        Modifier::LongLong => get_formatted::<c_longlong>(&mut args, width),
                        Modifier::IntMax => get_formatted::<intmax_t>(&mut args, width),
                        Modifier::Size => get_formatted::<size_t>(&mut args, width),
                        Modifier::PtrDiff => get_formatted::<ptrdiff_t>(&mut args, width),
                        _ => get_formatted::<c_int>(&mut args, width),
                    }.into_bytes();
                    write_stuff_before_number(&mut w,
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
                TokenType::UnsignedInteger => {},
                TokenType::UnsignedOctal => {},
                TokenType::UnsignedHex => {},
                TokenType::UnsignedHexUpper => {},
                TokenType::Float => {},
                TokenType::FloatUpper => {},
                TokenType::Scientific => {},
                TokenType::ScientificUpper => {},
                TokenType::ShortestFloat => {},
                TokenType::ShortestFloatUpper => {},
                TokenType::HexFloat => {},
                TokenType::HexFloatUpper => {},
                TokenType::Character => {
                    let arg = args.get::<i8>();
                    write(&mut w, &[arg as u8]);
                },
                TokenType::String => {},
                TokenType::PointerAddress => {},
                TokenType::StoreChars => { },
                TokenType::PercentLiteral => {},
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
unsafe fn get_formatted<T>(args: &mut VaList, width: Option<usize>) -> String
    where T: va_list::VaPrimitive + fmt::Display
{
    let t = args.get::<T>();
    if let Some(width) = width {
        format!("{:.*}", width, t)
    } else {
        format!("{}", t)
    }
}
