#![allow(dead_code)]
#![allow(unused_macros)]
#![no_main]

use std::{
    fmt,
    io::{Write, Stdout},
    io,
    str,
};

pub mod erase;
pub mod cursor;
/// this module (style.rs) contains a non macro defined fn.
pub mod style;
pub mod screen;

type DynErr<T> = Result<T, Box<dyn std::error::Error>>; 

#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        return(anyhow::anyhow!(expr).into());
    };
}

/// Object used for printing multiple escape sequences
pub struct MultiSeq(String);

impl MultiSeq {
    /// generates a storage struct for multi escape code sequences.
    pub fn new() -> Self {
        return Self(String::new()) 
    } 

    fn add(&mut self, seq: impl AsRef<str>) {
        self.0.push_str(seq.as_ref());     
    }

    /// prints the multi escape code sequence.
    pub fn print(&self, stdout: &mut Stdout) -> io::Result<()> {
        print!("{}", self.0);
        stdout.flush()?;
        return Ok(());
    } 
}

/// function used for printing singular escape sequences.
#[inline(always)]
fn template(
    stdout: &mut Stdout,
    msg: impl fmt::Display
) -> io::Result<()> {
    print!("{msg}");
    stdout.flush()?;    
    return Ok(()); 
}

#[macro_export]
macro_rules! gen_all {
    ($(
        $seq:literal, $args:expr, 
        [$(($arg_name:pat_param, $arg_type:ty)),*];
        $fn_name:ident,
        $doc:literal
    );*;) => {$(
        impl crate::MultiSeq {
            #[doc=concat!("Added to multi sequence struct:", $doc)]
            pub fn $fn_name(
                &mut self,
                $($arg_name: $arg_type),*
            ) {
                self.add(crate::MacroFmt::macro_fmt(
                    $args,
                    $seq,
                ))
            }
        }

        #[doc=$doc]
        pub fn $fn_name(
            mut stdout: &mut std::io::Stdout,
            $($arg_name: $arg_type),*
        ) -> std::io::Result<()> {
            Ok(crate::template( 
                &mut stdout,
                crate::MacroFmt::macro_fmt(
                    $args,
                    $seq,
                ),
            )?) 
        }
    )*};

    ($(($seq:literal, $name:ident, $doc:literal,));*;) => {$(
        impl crate::MultiSeq {
            #[doc=concat!("Added to multi sequence struct:", $doc)]
            pub fn $name(&mut self) {
                self.add($seq)
            }
        }

        #[doc=$doc]
        pub fn $name(mut stdout: &mut std::io::Stdout) -> std::io::Result<()> {
            Ok(crate::template( 
                &mut stdout,
                $seq,
            )?) 
        }
    )*};  
}

pub trait MacroFmt {
    const PAT: &str = "{}";

    fn macro_fmt(
        &self,
        lit: &'static str,
    ) -> String;
}

pub struct StrArray<'a, const N: usize>([&'a str; N]);

impl<'a, const N: usize> StrArray<'a, N> {
    #[inline(always)]
    pub fn new(str_array: [&'a str; N]) -> Self {
        return StrArray(str_array);
    }
}

impl<'a, const N: usize> AsRef<[&'a str]> for StrArray<'a, N> {
    fn as_ref(&self) -> &[&'a str] {
        return &self.0[0..];
    } 
}

impl<const N: usize> MacroFmt for StrArray<'_, N> {
    fn macro_fmt(
        &self,
        lit: &'static str,
    ) -> String {
        let mut ret = lit.to_string(); 

        debug_assert_eq!(
            ret.matches(Self::PAT)
                .collect::<Vec<&str>>()
                .len(),
            self.as_ref().len(),
            "Error: Unequal formatting args and format specifiers",
        );

        for fmt in self.as_ref() {
            ret = ret.replacen(Self::PAT, fmt, 1);
        }

        return ret;
    }
}

impl MacroFmt for &str {
    fn macro_fmt(
        &self,
        lit: &'static str,
    ) -> String {
        let ret = lit.to_string();

        debug_assert_eq!(
            ret.matches(Self::PAT)
                .collect::<Vec<&str>>()
                .len(),
            1, 
            "Error: Incorrect number of formatting args or specifiers",
        );

        return ret.replacen(Self::PAT, self, 1);
    }
}
