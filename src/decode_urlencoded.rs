// /home/robert/src/testcgi/src/decode_urlencoded.rs
// Copyright (C) 2025 Robert R. Russell
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, version 3.
//
// This program is distributed in the hope that it will be usefull, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

use std::str::Chars;
use std::string::String;

/// This is the detection mask for a four byte long UTF-8 character.
const FOUR_BYTE_DETECT: u32 = 0x000000F0;

/// This is the data mask for the fist of a four byte long UTF-8 character.
const FOUR_BYTE_DATA: u32 = 0x00000007;

/// This is the detection mask for a three byte long UTF-8 character.
const THREE_BYTE_DETECT: u32 = 0x000000E0;

/// This is the data mask for the first of a three byte long UTF-8 character.
const THREE_BYTE_DATA: u32 = 0x0000000F;

/// This is the detection mask for two byte long UTF-8 character.
const TWO_BYTE_DETECT: u32 = 0x000000C0;

/// This is the data mask for the first of a two byte long UTF-8 character.
const TWO_BYTE_DATA: u32 = 0x0000001F;

/// This is the detection mask for the following bytes in a multibyte UTF-8
/// character.
const FOLLOWING_BYTE_DETECT: u32 = 0x00000080;

/// This is the data mask for the following bytes in a multibyte UTF-8
/// character.
const FOLLOWING_BYTE_DATA: u32 = 0x0000003F;

/// Shift the existing data left by this many bytes left for every following
/// byte you add.
const FOLLOWING_BYTE_SHIFT: u8 = 6;

pub fn decode_urlencoded(input: String) -> Option<String> {
    let mut output: String = String::with_capacity(input.len());
    let mut characters: Chars = input.chars();
    while let Some(x) = characters.next() {
        if x == '+' {
            output.push(' ');
            continue;
        }
        if x != '%' {
            output.push(x);
            continue;
        }
        let first: Option<char> = characters.next();
        let second: Option<char> = characters.next();
        let mut temp_number: u32;
        match (first, second) {
            (Some(f), Some(s)) => {
                // We hope to have a character in hexadecimal format.
                if !(f.is_ascii_hexdigit() && s.is_ascii_hexdigit()) {
                    return None;
                }
                match f.to_digit(16) {
                    None => { return None; }
                    Some(d) => {
                        temp_number = d;
                        temp_number = temp_number << 4;
                    }
                }
                match s.to_digit(16) {
                    None => { return None; }
                    Some(d) => {
                        temp_number = temp_number + d;
                    }
                }

                match char::from_u32(temp_number) {
                    Some(c) => {
                        output.push(c);
                        continue;
                    }
                    None => {}
                }

                if (temp_number & FOUR_BYTE_DETECT) == FOUR_BYTE_DETECT {
                    temp_number = temp_number & FOUR_BYTE_DATA;
                    let mut chars: [Option<char>; 9] = [None; 9];
                    for i in 0..chars.len() {
                        chars[i] = characters.next();
                    }
                    let mut bytes: [u32; 9] = [0; 9];
                    for i in 0..chars.len() {
                        match chars[i] {
                            // Early end of data.
                            None => { return None; }
                            Some(c) => {
                                if (i % 3) == 0 {
                                    if c != '%' {
                                        // There should be more percent
                                        // encoded bytes.
                                        // There are not.
                                        return None;
                                    }
                                } else {
                                    match c.to_digit(16) {
                                        None => {
                                            // Percent encoded strings are
                                            // restricted to two ASCII hex
                                            // digits after the percent sign.
                                            return None;
                                        }
                                        Some(d) => {
                                            bytes[i] = d;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // Six more characters have been read and converted to
                    // nibbles.
                    for i in [1,4,7] {
                        bytes[i] = bytes[i] << 4;
                        bytes[i] = bytes[i] + bytes[i+1];
                    }
                    if (bytes[1] & bytes[4] & bytes[7] & FOLLOWING_BYTE_DETECT)
                        != FOLLOWING_BYTE_DETECT {
                            // One of the following bytes is not a valid
                            // follow multibyte character.
                            return None;
                        }
                    //Okay we have finally validated everything.
                    for i in [1,4,7] {
                        temp_number = temp_number << FOLLOWING_BYTE_SHIFT;
                        temp_number = temp_number + (bytes[i] &
                            FOLLOWING_BYTE_DATA);
                    }
                    match char::from_u32(temp_number) {
                        // All that work and the character is not valid.
                        None => { return None; }
                        Some(c) => { output.push(c); }
                    }
                    continue;
                }

                if (temp_number & THREE_BYTE_DETECT) == THREE_BYTE_DETECT {
                    temp_number = temp_number & THREE_BYTE_DATA;
                    let mut chars: [Option<char>; 6] = [None; 6];
                    for i in 0..chars.len() {
                        chars[i] = characters.next();
                    }
                    let mut bytes: [u32; 6] = [0; 6];
                    for i in 0.. chars.len() {
                        match chars[i] {
                            // Early end of data.
                            None => { return None; }
                            Some(c) => {
                                if (i % 3) == 0 {
                                    if c != '%' {
                                        // There should be more percent
                                        // encoded bytes.
                                        // There are not.
                                        return None;
                                    }
                                } else {
                                    match c.to_digit(16) {
                                        None => {
                                            // Percent encoded strings are
                                            // restricted to two ASCII hex
                                            // digits after the percent sign.
                                            return None;
                                        }
                                        Some(d) => {
                                            bytes[i] = d;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // Four more characters have been read and converted to
                    // nibbles.
                    for i in [1,4] {
                        bytes[i] = bytes[i] << 4;
                        bytes[i] = bytes[i] + bytes[i+1];
                    }
                    if (bytes[1] & bytes[4] & FOLLOWING_BYTE_DETECT)
                        != FOLLOWING_BYTE_DETECT {
                            // One of the following bytes is not a valid
                            // follow multibyte character.
                            return None;
                        }
                    //Okay we have finally validated everything.
                    for i in [1,4] {
                        temp_number = temp_number << FOLLOWING_BYTE_SHIFT;
                        temp_number = temp_number + (bytes[i] &
                            FOLLOWING_BYTE_DATA);
                    }
                    match char::from_u32(temp_number) {
                        // All that work and the character is not valid.
                        None => { return None; }
                        Some(c) => { output.push(c); }
                    }
                    continue;
                }
                if (temp_number & TWO_BYTE_DETECT) == TWO_BYTE_DETECT {
                    temp_number = temp_number & TWO_BYTE_DATA;
                    let mut chars: [Option<char>; 9] = [None; 9];
                    for i in 0..chars.len() {
                        chars[i] = characters.next();
                    }
                    let mut bytes: [u32; 3] = [0; 3];
                    for i in 0.. chars.len() {
                        match chars[i] {
                            // Early end of data.
                            None => { return None; }
                            Some(c) => {
                                if (i % 3) == 0 {
                                    if c != '%' {
                                        // There should be more percent
                                        // encoded bytes.
                                        // There are not.
                                        return None;
                                    }
                                } else {
                                    match c.to_digit(16) {
                                        None => {
                                            // Percent encoded strings are
                                            // restricted to two ASCII hex
                                            // digits after the percent sign.
                                            return None;
                                        }
                                        Some(d) => {
                                            bytes[i] = d;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // Three more characters have been read and converted to
                    // nibbles.
                    for i in [1] {
                        bytes[i] = bytes[i] << 4;
                        bytes[i] = bytes[i] + bytes[i+1];
                    }
                    if (bytes[1] & FOLLOWING_BYTE_DETECT)
                        != FOLLOWING_BYTE_DETECT {
                            // One of the following bytes is not a valid
                            // follow multibyte character.
                            return None;
                        }
                    //Okay we have finally validated everything.
                    for i in [1] {
                        temp_number = temp_number << FOLLOWING_BYTE_SHIFT;
                        temp_number = temp_number + (bytes[i] &
                            FOLLOWING_BYTE_DATA);
                    }
                    match char::from_u32(temp_number) {
                        // All that work and the character is not valid.
                        None => { return None; }
                        Some(c) => { output.push(c); }
                    }
                }
            }
            (Some(_), None) => {
                // The input data is incomplete or improperly formatted.
                return None;
            }
            (None, _) => {
                // We hit the end of input.
            }
        }
    }
    return Some(output);
}
