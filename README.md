# bond [![bond](https://docs.rs/bond/badge.svg?style=svg)](https://docs.rs/bond) [![CircleCI](https://circleci.com/gh/ekump/bond.svg?style=svg)](https://circleci.com/gh/ekump/bond)
A Rust crate for fast fixed-income calculations

I have primarily created this library for my own edification. My goals are to learn how to create a usable Rust Crate with good documentation, become more adept at Rust programming, and to relearn some fixed-income math that I've mostly forgotten. If people find this crate useful I may devote more attention to it.

My initial plan was to use the BigDecimal type instead of floats (or perhaps support both). However, it looks like there are a fair amount of math functions missing from the BigDecimal library necessary to perform calculations for fixed-income instruments. I may pause development of this library while I investigate implementing those math functions. 
