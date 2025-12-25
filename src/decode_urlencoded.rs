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
        let temp_string: String = String::with_capacity(8);
        match (first, second) {
            (Some(f), Some(s)) => {
                // We hope to have a character in hexadecimal format.
                if !(f.is_ascii_hexdigit() && s.is_ascii_hexdigit()) {
                    return None;
                }
                let mut y: String = String::from(f);
                y.push(s);
                let y: u32 = y.parse().unwrap();
                match char::from_u32(y) {
                    Some(z) => {
                        output.push(z);
                    }
                    None => {
                        // Someone percent encoded something that isn't ASCII.
                        // To send UTF-8 
                        return None;
                    }
                }
                todo!();
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
