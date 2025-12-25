// src/decode_query_string.rs
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

use std::collections::HashMap;

/// The buffer needs to have the entire input.
///
/// Per RFC 3986 a query string can contain just about any any ASCII character.
/// However this implementation expects a query string that looks like:
///
/// - name=spiderman
/// - name=Wing+Gundam&pilot=Heero%20Yuy
/// - still_alive=&yes=
/// - just_a_flag=
///
/// An empty map means the entire query string could not be parsed.
/// This function will ignore input data that it cannot parse.
/// So, empty values in the output HashMap can either mean no input was provided
/// or no key or value could be decoded.
/// You need to sanity check the results based on your needs.
pub fn decode_query_string(buffer: &[u8]) -> HashMap<String, String> {
    let mut output: HashMap<String, String> = HashMap::with_capacity(16);
    let mut _temp_vec: Vec<u8> = Vec::with_capacity(256);

    for x in buffer.split(|a| *a == b'&') {
        if x.contains(&b'=') {
            let first;
            let second;
            {
                let mut y = x.split(|a| *a == b'=');
                first = y.next();
                second = y.next();
            }
            let mut key: String;
            let mut value: String;
            if first.is_some() {
                let first = first.unwrap();
                key = String::from_utf8_lossy(first).into_owned();
                key = String::from(key.trim());
            } else {
                // Skip this item. It is not valid.
                continue;
            }
            if second.is_some() {
                let second = second.unwrap();
                value = String::from_utf8_lossy(second).into_owned();
                value = String::from(value.trim());
            } else {
                value = String::new();
            }
            output.insert(key, value);
        } else {
            // We will skip empty or invalid items;
            continue;
        }
    }

    return output;
}
