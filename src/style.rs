use crate::gen_all;
use std::io;

pub fn set_modes<T: AsRef<str>>(
    mut stdout: &mut io::Stdout,
    codes: impl AsRef<[T]>,
) -> Result<(), io::Error> {
    let mut seq = String::from("\x1B\x5B");

    let col_ref = codes.as_ref();
    let len = col_ref.len();

    for code in 0..(len - 1) {
        seq.push_str(col_ref[code].as_ref());
        seq.push(';');
    }

    seq.push_str(col_ref[len].as_ref());
    seq.push('m');

    crate::template(
        &mut stdout,
        seq,
    )?;

    return Ok(());
}

gen_all!(
    (
        "\x1B\x5B0m",
        reset_all_modes,
        "Resets all of the styles and colors.",
    ); 

    (
        "\x1B\x5B1m",
        set_bold_mode,
        "Sets bold mode for the whole screen.",
    );

    (
        "\x1B\x5B22m",
        reset_bold_and_faint_mode,
        "Resets bold and faint mode for the whole screen.",
    );

    (
        "\x1B\x5B2m",
        set_faint_mode,
        "Sets faint mode for the whole screen.",
    );

    (
        "\x1B\x5B3m",
        set_italic_mode,
        "Sets italic mode for the whole screen.",
    );

    (
        "\x1B\x5B23m", 
        reset_italic_mode,
        "Resets italic mode for the whole screen.",
    );

    (
        "\x1B\x5B4m",
        set_underline_mode,
        "Sets underline mode for the whole screen.",
    );

    (
        "\x1B\x5B24m",
        reset_underline_mode,
        "Resets underline mode for the whole screen.",
    );

    (
        "\x1B\x5B5m",
        set_blinking_mode,
        "Sets blinking mode for the whole screen.",
    );

    (
        "\x1B\x5B25m",
        reset_blinking_mode,
        "Resets blinking mode for the whole screen.",
    );

    (
        "\x1B\x5B7m",
        set_inverse_mode,
        "Sets inverse mode for the whole screen.",
    );

    (
        "\x1B\x5B27m",
        reset_inverse_mode,
        "Resets inverse mode for the whole screen.",
    );

    (
        "\x1B\x5B8m",
        set_hidden_mode,
        "Sets hidden mode for the whole screen.",
    );

    (
        "\x1B\x5B28m",
        reset_hidden_mode,
        "Resets hidden mode for the whole screen.",
    );

    (
        "\x1B\x5B9m",
        set_strikethrough_mode,
        "Sets strikethrough mode for the whole screen.",
    );

    (
        "\x1B\x5B29m",
        reset_strikethrough_mode,
        "Resets strikethrough mode for the whole screen.",
    );
);
