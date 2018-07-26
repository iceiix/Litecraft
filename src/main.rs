// The MIT License (MIT)
// Copyright © 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation
// files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(box_syntax)]
#![deny(unused_must_use)]
#![deny(unused_imports)]

#[macro_use]
extern crate glium;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate cgmath;
extern crate image;
extern crate pretty_env_logger;
extern crate serde;
extern crate serde_yaml;
extern crate threadpool;
extern crate zip;

mod core;
mod gfx;
mod scenes;
mod tests;

use core::constants::*;
use gfx::canvas::Canvas;

use std::path::Path;

fn main() {
    println!("{}", ASCII_ART);
    println!(
        "Starting Litecraft {} for Minecraft {}...",
        LITECRAFT_VERSION, MINECRAFT_VERSION
    );

    pretty_env_logger::init();

    if !Path::new("resources").exists() {
        panic!(
            "Resources path doesn't exist, please check that you have all required resources. Check \
             Litecraft's README.md for more details."
        );
    }

    Canvas::start();
}
