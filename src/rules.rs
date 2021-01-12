//! From https://doc.rust-lang.org/nomicon/lifetime-elision.html
//!
//! Elision rules are as follows:
//!
//! 1. each elided lifetime in input position becomes a distinct
//!    lifetime parameter.
//!
//! 2. If there is exactly one input lifetime position (elided or
//!    not), that lifetime is assigned to all elided output lifetimes.
//!
//! 3. If there are multiple input lifetime positions, but one of them
//!    is &self or &mut self, the lifetime of self is assigned to all
//!    elided output lifetimes.
//!
//! 4. Otherwise, it is an error to elide an output lifetime.

fn f1_elided(s: &'_ str) {
    println!("{}", s)
}

// fn f2_elided(i: usize, s: &str) {
//     if i > 42 {
//         println!("{}", s)
//     }
// }

fn f3_elided<'a>(s: &'a str, i: usize) -> (&'a str, &'a str) {
    unimplemented!()
}

// // illegal
// fn f4_elided() -> &str {
//     &"foo"
// }

// fn f4_bis() -> &'static str {
//     "foo"
// }

// illegal
fn f5_elided<'a>(s: &'a str, t: &'a str) -> &'a str {
    if s.len() > t.len() {
        s
    } else {
        t
    }
}

fn f5_elided_bis<'a, 'b: 'a>(s: &'a str, t: &'b str) -> &'a str {
    if s.len() > t.len() {
        s
    } else {
        t
    }
}

pub struct Foo {
    numbers: Vec<u32>,
}

impl Foo {
    fn m1_elided(&self) -> &Vec<u32> {
        &self.numbers
    }

    fn m2_elided<'a>(&self, args: &'a [u32]) -> &'a [u32] {
        args
    }

    fn m3_elided(&self, _args: &[u32]) -> &[u32] {
        self.numbers.as_slice()
    }
}


pub struct Bar<'a> {
    i: &'a usize
}

impl<'a> Bar<'a> {
    fn m1_elided(&self) -> &usize {
        self.i
    }

    fn m2_elided(&self) -> &'a usize {
        self.i
    }
}
