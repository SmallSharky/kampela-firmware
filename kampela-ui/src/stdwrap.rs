

#[cfg(not(feature="std"))]
use alloc::{format, string::String, vec::Vec};


#[cfg(feature="std")]
pub trait Std {
    type Vec = alloc::vec::Vec;
    type String = alloc::string::String;
}





#[cfg(feature="std")]
use std::{format, string::String, vec::Vec};

#[cfg(feature="std")]
pub trait Std {
    type Vec = std::Vec;
    type String = std::String;
}