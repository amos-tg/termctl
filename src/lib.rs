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
pub mod color;
pub mod graphics;
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
                    $seq,
                    $args,
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
                    $seq,
                    $args,
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

pub trait MacroFmt<T, U = ()> {
    const PAT: &str = "{}";

    fn macro_fmt(
        lit: &'static str,
        fmtted: Self,
    ) -> String;
}

impl<T, U> MacroFmt<T, U> for T where T: AsRef<[U]>, U: ToString {
    fn macro_fmt(
        lit: &'static str,
        fmtted: Self,
    ) -> String {
        let mut ret = lit.to_string(); 

        debug_assert_eq!(
            ret.matches(Self::PAT)
                .collect::<Vec<&str>>()
                .len(),
            fmtted.as_ref().len(),
            "Error: Unequal formatting args and format specifiers",
        );

        for fmt in fmtted.as_ref() {
            ret = ret.replacen(Self::PAT, fmt.to_string().as_str(), 1);
        }

        return ret;
    }
}

impl<T> MacroFmt<T> for &T 
where 
    T: ToString, 
{

    fn macro_fmt(
        lit: &'static str,
        fmtted: Self,
    ) -> String {
        let ret = lit.to_string();

        debug_assert_eq!(
            ret.matches(Self::PAT)
                .collect::<Vec<&str>>()
                .len(),
            1, 
            "Error: Incorrect number of formatting args or specifiers",
        );

        return ret.replacen(Self::PAT, fmtted.to_string().as_str(), 1);
    }
}
