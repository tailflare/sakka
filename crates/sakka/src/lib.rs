#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

extern crate alloc;

mod common;
mod encoding;
mod error;
mod rw;

#[cfg(feature = "derive")]
pub use sakka_derive::{Decode, Encode};

pub use self::{
    common::Endian,
    encoding::{Codec, Decode, Encode},
    error::Error,
    rw::{
        CollectionLength, ReadCollection, ReadOption, ReadPrimitive, Reader, WriteCollection,
        WriteOption, WritePrimitive, Writer,
    },
};
