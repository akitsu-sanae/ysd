# What is ysd?
Ysd is a text editor made with Rust lang.  
'ysd' is '安定' in Japanese Kanji.  

# How to build
1. install cargo (Rust's build system and package manager)
2. run `git clone https://github.com/akitsu-sanae/ysd`
3. run `cd ysd`
4. run `cargo build --release`
5. then, you can fild executable ysd in `./target/release/`

# How to use

* `ysd <filename>` ... open file
* in Move Mode (default)
    - j ... move left
    - l ... move right
    - i ... move up
    - k ... move down
    - a ... chage mode to Edit
    - F1 ... change mode to Edit
* in Edit Mode
    - Esc ... change mode to Move
    - otherwise ... insert charactor at current cursor.

# Copyright
Copyright (C) 2016 akitsu sanae.  
Distributed under the Boost Software License, Version 1.0. 
(See accompanying file LICENSE_1_0 or copy at http://www.boost/org/LICENSE_1_0.txt)  


