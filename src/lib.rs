/*!
# Trim in-place

This crate is used for extending `String` in order to do in-place trimming.

## Usage

```rust
use trim_in_place::TrimInPlace;

let mut s = String::from(" 1234 abcd  ");

s.trim_in_place();

assert_eq!("1234 abcd", s);
```

## Benchmark

```bash
cargo bench
```
*/

#![no_std]

extern crate alloc;

use alloc::string::String;
use core::intrinsics::copy;

mod sealed {
    pub enum ConcretePattern<'a> {
        Func(&'a mut dyn FnMut(char) -> bool),
        Str(&'a str),
        Char(char),
    }

    pub trait Pattern {
        fn as_pattern_enum(&mut self) -> ConcretePattern<'_>;
    }

    impl Pattern for char {
        fn as_pattern_enum(&mut self) -> ConcretePattern<'_> {
            ConcretePattern::Char(*self)
        }
    }

    impl Pattern for str {
        fn as_pattern_enum(&mut self) -> ConcretePattern<'_> {
            ConcretePattern::Str(self)
        }
    }

    impl<F: FnMut(char) -> bool> Pattern for F {
        fn as_pattern_enum(&mut self) -> ConcretePattern<'_> {
            ConcretePattern::Func(self)
        }
    }
}

use sealed::Pattern;

macro_rules! deconstruct_pattern {
    ($func:path, $self:ident, $arg:ident) => {
        match sealed::Pattern::as_pattern_enum(&mut $arg) {
            sealed::ConcretePattern::Func(val) => $func($self, val),
            sealed::ConcretePattern::Char(val) => $func($self, val),
            sealed::ConcretePattern::Str(val) => $func($self, val),
        }
    };
}

pub trait TrimInPlace {
    fn trim_in_place(&mut self) -> &str;
    fn trim_start_in_place(&mut self) -> &str;
    fn trim_end_in_place(&mut self) -> &str;

    // TODO trim_matches with Pattern
    fn trim_matches_in_place(&mut self, pat: char) -> &str;
    fn trim_start_matches_in_place(&mut self, pat: impl Pattern) -> &str;
    fn trim_end_matches_in_place(&mut self, pat: impl Pattern) -> &str;
}

impl TrimInPlace for String {
    #[inline]
    fn trim_in_place(&mut self) -> &str {
        let trimmed_str = self.trim();

        let trimmed_str_start_pointer = trimmed_str.as_ptr();
        let trimmed_str_length = trimmed_str.len();

        unsafe {
            let v = self.as_mut_vec();

            copy(trimmed_str_start_pointer, v.as_mut_ptr(), trimmed_str_length);

            v.set_len(trimmed_str_length);
        }

        self.as_str()
    }

    #[inline]
    fn trim_start_in_place(&mut self) -> &str {
        let trimmed_str = self.trim_start();

        let trimmed_str_start_pointer = trimmed_str.as_ptr();
        let trimmed_str_length = trimmed_str.len();

        unsafe {
            let v = self.as_mut_vec();

            copy(trimmed_str_start_pointer, v.as_mut_ptr(), trimmed_str_length);

            v.set_len(trimmed_str_length);
        }

        self.as_str()
    }

    #[inline]
    fn trim_end_in_place(&mut self) -> &str {
        let trimmed_str_length = self.trim_end().len();

        unsafe {
            self.as_mut_vec().set_len(trimmed_str_length);
        }

        self.as_str()
    }

    #[inline]
    fn trim_matches_in_place(&mut self, pat: char) -> &str {
        let trimmed_str = self.trim_matches(pat);

        let trimmed_str_start_pointer = trimmed_str.as_ptr();
        let trimmed_str_length = trimmed_str.len();

        unsafe {
            let v = self.as_mut_vec();

            copy(trimmed_str_start_pointer, v.as_mut_ptr(), trimmed_str_length);

            v.set_len(trimmed_str_length);
        }

        self.as_str()
    }

    #[inline]
    fn trim_start_matches_in_place(&mut self, mut pat: impl Pattern) -> &str {
        let trimmed_str = deconstruct_pattern!(str::trim_start_matches, self, pat);

        let trimmed_str_start_pointer = trimmed_str.as_ptr();
        let trimmed_str_length = trimmed_str.len();

        unsafe {
            let v = self.as_mut_vec();

            copy(trimmed_str_start_pointer, v.as_mut_ptr(), trimmed_str_length);

            v.set_len(trimmed_str_length);
        }

        self.as_str()
    }

    #[inline]
    fn trim_end_matches_in_place(&mut self, mut pat: impl Pattern) -> &str {
        let trimmed_str_length = deconstruct_pattern!(str::trim_end_matches, self, pat).len();

        unsafe {
            self.as_mut_vec().set_len(trimmed_str_length);
        }

        self.as_str()
    }
}
