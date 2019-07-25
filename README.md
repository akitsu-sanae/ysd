# What is ysd?
Ysd is a text editor made with Rust lang.  
'ysd' is '安定' in Japanese Kanji.  

# How to build
1. install cargo (Rust's build system and package manager)
2. run `git clone https://github.com/akitsu-sanae/ysd`
3. run `cd ysd`
4. run `cargo build --release`
5. then, you can find executable ysd in `./target/release/`

# How to use

* `ysd <filename>` ... open file
* in Command Mode (default)
    - j ... move left
    - l ... move right
    - i ... move up
    - k ... move down
    - `:edit` ... change to Edit Mode
    - `:save-as <filename>` ... save current buffer as `<filename>`
    - `:toggle-line-number` ... enable/disable showing line numbers
    - `:quit` ... quit ysd
* in Edit Mode
    - Esc ... change to Command Mode
    - otherwise ... insert charactor at current cursor position.

