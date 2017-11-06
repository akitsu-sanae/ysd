/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

pub struct Config {
    pub line_number_visible: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            line_number_visible: false,
        }
    }
}


