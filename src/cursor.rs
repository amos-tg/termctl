use crate::gen_all;

gen_all!(
    (
        "\x1B\x5B\x48",
        home_position,
        "ESC[H: moves cursor to home position (0, 0)",
    );

    (
        "\x1B\x5B6n", 
        request_pos,
        "ESC[6n: requests cursor position, reports as ESC[#;#R",
    );

    (
        "\x1B M",
        up_one_line,
        "ESC M: moves cursor up one line, scrolling if needed",
    );

    (
        "\x1B 7",
        save_pos_dec,
        "ESC M: saves cursor position (DEC)",
    );

    (
        "\x1B 8",
        restore_saved_pos,
        "ESC 8: restores last saved cursor position (SCO)",
    );

    (
        "\x1B\x5Bs",
        save_pos_sco,
        "ESC[s: save cursor position (SCO)",
    );

    (
        "\x1B\x5Bu",
        restore_pos_sco,
        "ESC[u: restores last saved cursor position (SCO)",
    );
);

gen_all!(
    "\x1B\x5B\x7B{}\x7D\x3B\x7B{}\x7DH", &[line, column],
    [(line, u16), (column, u16)];
    to_line_n_column_n_h,
    "ESC\x5B\x7Bline\x7D\x3B\x7Bcolumn\x7DH: moves cursor to line n, and column; if this doesn't work try the f version.";

    "\x1B\x5B\x7B{}\x7D\x3B\x7B{}\x7Df", &[line, column],
    [(line, u16), (column, u16)];
    to_line_n_column_n_f,
    "ESC\x5B\x7Bline\x7D\x3B\x7Bcolumn\x7Df: moves cursor to line n, and column; if this doesn't work try the h version.";

    "\x1B\x5B{}A", &lines,
    [(lines, u16)];
    up_n_lines,
    "ESC[#linesA: moves cursor up n #lines";

    "\x1B\x5B{}B", &lines,
    [(lines, u16)];
    down_n_lines,
    "ESC[#linesB: moves cursor down n #lines";

    "\x1B\x5B{}C", &columns,
    [(columns, u16)];
    right_n_columns,
    "ESC[#columnsC: moves cursor right n #columns";

    "\x1B\x5B{}D", &columns,
    [(columns, u16)];
    left_n_columns,
    "ESC[#columnsD: moves cursor left n #columns";

    "\x1B\x5B{}E", &lines,
    [(lines, u16)];
    beg_of_next_n_down,
    "ESC[#linesE: moves cursor to the beginning of next line, n #lines down";

    "\x1B\x5B{}F", &lines,
    [(lines, u16)];
    beg_of_prev_n_up,
    "ESC[#linesF: moves cursor to the beginning of previous line n #lines up";

    "\x1B\x5B{}G", &column,
    [(column, u16)];
    column_n,
    "ESC[#columnG: moves cursor to #column n";

);
