use crate::gen_all;

gen_all!(
    "\x1B\x5B={}h", &mode,
    [(mode, u8)]; 
    set_mode_val, 
    "Sets the mode for the given value.";

    "\x1B\x5B={}l", &mode,
    [(mode, u8)];
    reset_mode_val,
    "Resets the mode for the given value.";
);

gen_all!(
    (
        "\x1B\x5B0h",
        set_monochrome_40x25,
        "Sets the text to 40x25 monochrome.",
    );

    (
        "\x1B\x5B0l",
        reset_monochrome_40x25,
        "Sets the text from 40x25 monochrome.",
    );

    (
        "\x1B\x5B1h",
        set_color_40x25,
        "Sets the text to 40x25 colored.",
    );

    (
        "\x1B\x5B1l",
        reset_color_40x25,
        "Resets the text from 40x25 colored.",
    );

    (
        "\x1B\x5B2h",
        set_monochrome_80x25,
        "Sets the text to 80x25 monochrome.",
    );

    (
        "\x1B\x5B2l",
        reset_monochrome_80x25,
        "Resets the text from 80x25 monochrome.",
    );

    (
        "\x1B\x5B3h", 
        set_color_80x25,
        "Sets the text to 80x25 colored.",
    );

    (
        "\x1B\x5B3l", 
        reset_color_80x25,
        "Resets the text from 80x25 colored.",
    );

    (
        "\x1B\x5B4h",
        set_colorx4_320x200,
        "Sets the text to 320x200 4-color graphics.",
    );

    (
        "\x1B\x5B4l",
        reset_colorx4_320x200,
        "Resets the text from 320x200 4-color graphics.",
    );

    (
        "\x1B\x5B5h",
        set_monochrome_320x200,
        "Sets the text to 320x200 monochrome.",
    );

    (
        "\x1B\x5B5l",
        reset_monochrome_320x200,
        "Resets the text from 320x200 monochrome.",
    );

    (
        "\x1B\x5B6h",
        set_monochrome_640x200,
        "Sets the text to 640x200 monochrome.",
    );

    (
        "\x1B\x5B6l",
        reset_monochrome_640x200,
        "Resets the text from 640x200 monochrome.",
    );

    (
        "\x1B\x5B7h",
        set_line_wrapping,
        "Enables line wrapping",
    );

    (
        "\x1B\x5B7l",
        reset_line_wrapping,
        "Disables line wrapping",
    );

    (
        "\x1B\x5B13h",
        set_color_320x200,
        "Sets the text to 320x200 colored.",
    );

    (
        "\x1B\x5B13l",
        reset_color_320x200,
        "Resets the text from 320x200 colored.",
    );

    (
        "\x1B\x5B14h",
        set_colorx16_640x200,
        "Sets the text to 640x200 16-color colored.",
    );

    (
        "\x1B\x5B14l",
        reset_colorx16_640x200,
        "Resets the text from 640x200 16-color colored.",
    );

    (
        "\x1B\x5B15h",
        set_monochromex2_640x350,
        "Sets the text to 640x350 2-color monochrome.",
    );

    (
        "\x1B\x5B15l",
        reset_monochromex2_640x350,
        "Resets the text from 640x350 2-color monochrome.",
    );
    (
        "\x1B\x5B16h",
        set_colorx16_640x250,
        "Sets the text to 640x250 16-color colored.",
    );

    (
        "\x1B\x5B17l",
        reset_monochromex2_640x480,
        "Resets the text from 640x480 2-color monochrome.",
    );

    (
        "\x1B\x5B18h",
        set_colorx16_640x480,
        "Sets the text to 640x480 16-color graphics.",
    );

    (
        "\x1B\x5B18l",
        reset_colorx16_640x480,
        "Resets the text from 640x480 16-color graphics.",
    );

    (
        "\x1B\x5B19h",
        set_colorx256_320x200,
        "Sets the text to 320x200 256-color graphics.",
    );

    (
        "\x1B\x5B19l",
        reset_colorx256_320x200,
        "Resets the text from 320x200 256-color graphics.",
    );
);

