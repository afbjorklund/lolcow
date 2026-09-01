// Copyright (C) 2020 jaseg <github@jaseg.net>

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

const CODES: [u8; 30] = [
    39, 38, 44, 43, 49, 48, 84, 83, 119, 118, 154, 148, 184, 178, 214, 208, 209, 203, 204, 198,
    199, 163, 164, 128, 129, 93, 99, 63, 69, 33,
];
const CODES16: [u8; 12] = [31, 33, 32, 36, 34, 35, 95, 94, 96, 92, 93, 91];
const CODES_GRADIENT: [u8; 511] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242,
    241, 240, 239, 238, 237, 236, 235, 234, 233, 232, 231, 230, 229, 228, 227, 226, 225, 224, 223,
    222, 221, 220, 219, 218, 217, 216, 215, 214, 213, 212, 211, 210, 209, 208, 207, 206, 205, 204,
    203, 202, 201, 200, 199, 198, 197, 196, 195, 194, 193, 192, 191, 190, 189, 188, 187, 186, 185,
    184, 183, 182, 181, 180, 179, 178, 177, 176, 175, 174, 173, 172, 171, 170, 169, 168, 167, 166,
    165, 164, 163, 162, 161, 160, 159, 158, 157, 156, 155, 154, 153, 152, 151, 150, 149, 148, 147,
    146, 145, 144, 143, 142, 141, 140, 139, 138, 137, 136, 135, 134, 133, 132, 131, 130, 129, 128,
    127, 126, 125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114, 113, 112, 111, 110, 109,
    108, 107, 106, 105, 104, 103, 102, 101, 100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88,
    87, 86, 85, 84, 83, 82, 81, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71, 70, 69, 68, 67, 66, 65, 64,
    63, 62, 61, 60, 59, 58, 57, 56, 55, 54, 53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40,
    39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16,
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
];

#[derive(PartialEq, Clone, Copy)]
enum EscapeState {
    None,
    EscBegin,
    EscString,
    EscCsi,
    EscStringTerm,
    EscCsiTerm,
    EscTerm,
}

fn find_escape_sequences(c: char, state: EscapeState) -> EscapeState {
    match state {
        EscapeState::None | EscapeState::EscCsiTerm => {
            if c == '\x1b' {
                EscapeState::EscBegin
            } else {
                EscapeState::None
            }
        }
        EscapeState::EscBegin => {
            if c == '[' {
                EscapeState::EscCsi
            } else if c == 'P' || c == ']' || c == 'X' || c == '^' || c == '_' {
                EscapeState::EscString
            } else {
                EscapeState::EscTerm
            }
        }
        EscapeState::EscCsi => {
            if (0x40..=0x7e).contains(&(c as u32)) {
                EscapeState::EscCsiTerm
            } else {
                state
            }
        }
        EscapeState::EscString => {
            if c == '\x07' {
                EscapeState::None
            } else if c == '\x1b' {
                EscapeState::EscStringTerm
            } else {
                state
            }
        }
        EscapeState::EscStringTerm => {
            if c == '\\' {
                EscapeState::None
            } else {
                EscapeState::EscString
            }
        }
        EscapeState::EscTerm => EscapeState::None,
    }
}

fn wchar_width(c: char) -> i32 {
    if c.is_control() {
        0
    } else {
        1
    }
}

fn rand() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static RAND_STATE: AtomicU32 = AtomicU32::new(1);
    let state = RAND_STATE.load(Ordering::SeqCst);
    let new_state = (state * 1103515245 + 12345) & 0x7fffffff;
    RAND_STATE.store(new_state, Ordering::SeqCst);
    new_state
}

fn srand(seed: u32) {
    use std::sync::atomic::{AtomicU32, Ordering};
    static RAND_STATE: AtomicU32 = AtomicU32::new(1);
    RAND_STATE.store(seed, Ordering::SeqCst);
}

const RAND_MAX: u32 = 0x7fffffff;

fn interpolate_rgb(start: &Rgb, end: &Rgb, f: f64) -> Rgb {
    Rgb {
        r: (start.r as f64 + (end.r as f64 - start.r as f64) * f).round() as u8,
        g: (start.g as f64 + (end.g as f64 - start.g as f64) * f).round() as u8,
        b: (start.b as f64 + (end.b as f64 - start.b as f64) * f).round() as u8,
    }
}

pub fn lolcat_write(input: &str, out: &mut dyn Write) {
    let color_output = true;
    let random = false;
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    let start_color: i32 = 0;
    let rgb = false;
    let ansi16 = false;
    let invert = false;
    let gradient = false;
    let freq_h: f64 = 0.23;
    let freq_v: f64 = 0.1;

    let rgb_start = Rgb { r: 255, g: 0, b: 0 };
    let rgb_end = Rgb {
        r: 0,
        g: 255,
        b: 255,
    };

    if random {
        srand(seed);
    }

    let mut offx = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as f64
        % 300.0;
    offx /= 300.0;

    let mut l: i32 = 0;
    let mut i: i32 = 0;
    let mut cc: i32 = -1;

    let mut escape_state = EscapeState::None;

    for c in input.chars() {
        if color_output {
            escape_state = find_escape_sequences(c, escape_state);

            if escape_state == EscapeState::EscCsiTerm {
                let _ = out.write_all(&[c as u8]);
            }

            if escape_state == EscapeState::None || escape_state == EscapeState::EscCsiTerm {
                if c == '\n' {
                    l += 1;
                    i = 0;
                    cc = -1;
                    let width = if invert { 49 } else { 0 };
                    let _ = out.write_all(format!("\x1b[{}m", width).as_bytes());
                } else {
                    if escape_state == EscapeState::None {
                        i += wchar_width(c) as i32;
                    }

                    if rgb {
                        let rand_offset = if random { rand() as f64 } else { 0.0 };
                        let theta: f64 = i as f64 * freq_h / 5.0
                            + l as f64 * freq_v
                            + (offx + 2.0 * (rand_offset + start_color as f64) / (RAND_MAX as f64))
                                * std::f64::consts::PI;

                        if gradient {
                            let mut theta = theta / (2.0 * std::f64::consts::PI);
                            theta = theta - theta.floor();
                            if theta > 1.0 {
                                theta = 2.0 - theta;
                            }
                            let rgb_c = interpolate_rgb(&rgb_start, &rgb_end, theta as f64);
                            let _ = out.write_all(
                                format!(
                                    "\x1b[{};2;{};{};{}m",
                                    if invert { 48 } else { 38 },
                                    rgb_c.r,
                                    rgb_c.g,
                                    rgb_c.b
                                )
                                .as_bytes(),
                            );
                        } else {
                            let offset = 0.1;
                            let rgb_c = Rgb {
                                r: ((offset + (1.0 - offset) * (0.5 + 0.5 * (theta + 0.0).sin()))
                                    * 255.0)
                                    .round() as u8,
                                g: ((offset
                                    + (1.0 - offset)
                                        * (0.5
                                            + 0.5
                                                * (theta + 2.0 * std::f64::consts::PI / 3.0)
                                                    .sin()))
                                    * 255.0)
                                    .round() as u8,
                                b: ((offset
                                    + (1.0 - offset)
                                        * (0.5
                                            + 0.5
                                                * (theta + 4.0 * std::f64::consts::PI / 3.0)
                                                    .sin()))
                                    * 255.0)
                                    .round() as u8,
                            };
                            let _ = out.write_all(
                                format!(
                                    "\x1b[{};2;{};{};{}m",
                                    if invert { 48 } else { 38 },
                                    rgb_c.r,
                                    rgb_c.g,
                                    rgb_c.b
                                )
                                .as_bytes(),
                            );
                        }
                    } else if ansi16 {
                        let rand_offset = if random { rand() as usize } else { 0 };
                        let ncc = (offx * CODES16.len() as f64
                            + (i as f64 * freq_h as f64 + l as f64 * freq_v as f64))
                            as usize;
                        if cc != ncc as i32 || escape_state == EscapeState::EscCsiTerm {
                            cc = ncc as i32;
                            let idx =
                                (rand_offset + start_color as usize + cc as usize) % CODES16.len();
                            let _ = out.write_all(
                                format!("\x1b[{};{}m", if invert { 10 } else { 0 }, CODES16[idx])
                                    .as_bytes(),
                            );
                        }
                    } else {
                        if gradient {
                            let rand_offset = if random { rand() as usize } else { 0 };
                            let ncc = (offx * CODES_GRADIENT.len() as f64
                                + (i as f64 * freq_h as f64 + l as f64 * freq_v as f64))
                                as usize;
                            if cc != ncc as i32 || escape_state == EscapeState::EscCsiTerm {
                                cc = ncc as i32;
                                let lookup = (rand_offset + start_color as usize + cc as usize)
                                    % (2 * CODES_GRADIENT.len());
                                let lookup = if lookup >= CODES_GRADIENT.len() {
                                    2 * CODES_GRADIENT.len() - 1 - lookup
                                } else {
                                    lookup
                                };
                                let _ = out.write_all(
                                    format!(
                                        "\x1b[{};5;{}m",
                                        if invert { 48 } else { 38 },
                                        CODES_GRADIENT[lookup]
                                    )
                                    .as_bytes(),
                                );
                            }
                        } else {
                            let rand_offset = if random { rand() as usize } else { 0 };
                            let ncc = (offx * CODES.len() as f64
                                + (i as f64 * freq_h as f64 + l as f64 * freq_v as f64))
                                as usize;
                            if cc != ncc as i32 || escape_state == EscapeState::EscCsiTerm {
                                cc = ncc as i32;
                                let idx = (rand_offset + start_color as usize + cc as usize)
                                    % CODES.len();
                                let _ = out.write_all(
                                    format!(
                                        "\x1b[{};5;{}m",
                                        if invert { 48 } else { 38 },
                                        CODES[idx]
                                    )
                                    .as_bytes(),
                                );
                            }
                        }
                    }
                }
            }
        }

        if escape_state != EscapeState::EscCsiTerm {
            let _ = out.write_all(&[c as u8]);
        }
    }

    if color_output {
        let _ = out.write_all(b"\x1b[0m");
    }
}
