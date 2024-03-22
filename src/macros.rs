#![allow(unused_macros, unused_imports)]

macro_rules! prt {
    ($var: expr) => {
        println!("{:2} {:2} {:p} {:<15?} '{}'", $var.len(), $var.capacity(), &$var, $var.as_ptr(), $var);
    };
}
pub(crate) use prt;

macro_rules! prt_str {
    ($var: expr) => {
        println!("{:2}    {:p} {:<15?} '{}'", $var.len(), &$var, $var.as_ptr(), $var);
    };
}
pub(crate) use prt_str;
