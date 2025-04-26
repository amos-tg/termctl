use crate::gen_all;

#[cfg(test)]
mod test;

gen_all!(
    (
        "\x1B\x5B\x4A",
        in_display,
        "ESC[J: erase in display, erases from cursor until end of screen",
    );

    (
        "\x1B\x5B\x30\x4A",
        entire_screen_from_cursor,
        "ESC[0J: erase from cursor until end of screen",
    );

    (
        "\x1B\x5B\x31\x4A",
        cursor_to_beginning_of_screen,
        "ESC[1J: erase from cursor to beginning of screen",
    );

    (
        "\x1B\x5B\x32\x4A",
        entire_screen,
        "ESC[2J: erases the entire screen",
    );

    (
        "\x1B\x5B\x33\x4A",
        saved_lines,
        "ESC[3J: erase saved lines",
    );

    (
        "\x1B\x5B\x4B",
        in_line,
        "ESC[K: erase in line (same as erase from cursor to end of line)",
    );

    (
        "\x1B\x5B\x30\x4B",
        cursor_to_end_line,
        "ESC[0K: erase from cursor to end of line",
    );

    (
        "\x1B\x5B\x31\x4B",
        line_start_to_cursor,
        "ESC[1K: erase start of line to the cursor",
    );

    (
        "\x1B\x5B\x32\x4B",    
        entire_line,
        "ESC[2K: erase the entire line without moving the cursor",
    );
);
