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

use std::collections::HashMap;

pub fn decode_urlencoded(buffer: &[u8]) -> HashMap<String, Vec<u8>> {
    let mut output: HashMap<String, Vec<u8>> = HashMap::with_capacity(16);
    
    return output;
}
