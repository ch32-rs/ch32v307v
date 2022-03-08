// Copyright 2022 Ahmed Charles <me@ahmedcharles.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // Put the memory definitions somewhere the linker can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    let boards: Vec<_> = env::vars()
        .filter_map(|(key, _value)| {
            if key.starts_with("CARGO_FEATURE_BOARD") {
                Some(key[20..].to_ascii_lowercase()) // Strip 'CARGO_FEATURE_BOARD_'
            } else {
                None
            }
        })
        .collect();

    if boards.is_empty() {
        panic!("No board features selected");
    }
    if boards.len() > 1 {
        panic!("More than one board feature selected: {:?}", boards);
    }

    let board = boards.first().unwrap();

    match board.as_str() {
        "ch32v307v_r1" => {
            fs::copy("memory-ch32v307v-r1.x", out_dir.join("ch32v307v-memory.x")).unwrap();
            println!("cargo:rerun-if-changed=memory-ch32v307v-r1.x");
        }

        other => panic!("Unknown board: {}", other),
    }

    fs::copy("ch32v307v-link.x", out_dir.join("ch32v307v-link.x")).unwrap();
}
