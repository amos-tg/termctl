use crate::{DynErr, err};
use std::io;

#[cfg(test)]
fn in_display_test() -> DynErr<()> {
    println!("test\ncontent");
    let mut stdout = io::stdout();
    super::in_display(&mut stdout)?;

    return Ok(());
}
