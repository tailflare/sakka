#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

extern crate alloc;

mod codec;
mod common;
mod error;
mod rw;

#[cfg(feature = "derive")]
pub use sakka_derive::{Decode, Encode};

pub use self::{
    codec::{Decode, Encode},
    common::Endian,
    error::Error,
    rw::{
        CollectionLength, ReadCollection, ReadOption, ReadPrimitive, Reader, WriteCollection,
        WriteOption, WritePrimitive, Writer,
    },
};
