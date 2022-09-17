// Copyright (C) 2022-present The NetGauze Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Serializer for BGP Path Attributes

use crate::{
    path_attribute::{Origin, PathAttribute},
    serde::serializer::update::BGPUpdateMessageWritingError,
};
use byteorder::{NetworkEndian, WriteBytesExt};
use netgauze_parse_utils::{WritablePDU, WritablePDUWithOneInput};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PathAttributeWritingError {
    StdIOError(String),
    OriginError(OriginWritingError),
}

impl From<std::io::Error> for PathAttributeWritingError {
    fn from(err: std::io::Error) -> Self {
        PathAttributeWritingError::StdIOError(err.to_string())
    }
}

impl From<PathAttributeWritingError> for BGPUpdateMessageWritingError {
    fn from(value: PathAttributeWritingError) -> Self {
        BGPUpdateMessageWritingError::PathAttributeError(value)
    }
}

impl WritablePDU<PathAttributeWritingError> for PathAttribute {
    const BASE_LENGTH: usize = 0;

    fn len(&self) -> usize {
        todo!()
    }

    fn write<T: std::io::Write>(&self, _writer: &mut T) -> Result<(), PathAttributeWritingError> {
        todo!()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum OriginWritingError {
    StdIOError(String),
}

impl From<std::io::Error> for OriginWritingError {
    fn from(err: std::io::Error) -> Self {
        OriginWritingError::StdIOError(err.to_string())
    }
}

impl From<OriginWritingError> for PathAttributeWritingError {
    fn from(value: OriginWritingError) -> Self {
        PathAttributeWritingError::OriginError(value)
    }
}

#[inline]
fn write_length<T: Sized + WritablePDUWithOneInput<bool, E>, E, W: std::io::Write>(
    attribute: &T,
    extended_length: bool,
    writer: &mut W,
) -> Result<(), E>
where
    E: From<std::io::Error>,
{
    let len = attribute.len(extended_length) - 1;
    if extended_length || len > u8::MAX.into() {
        writer.write_u16::<NetworkEndian>((len - 1) as u16)?;
    } else {
        writer.write_u8(len as u8)?;
    }
    Ok(())
}

impl WritablePDUWithOneInput<bool, OriginWritingError> for Origin {
    // One octet length (if extended is not enabled) and second for the origin value
    const BASE_LENGTH: usize = 2;

    fn len(&self, extended_length: bool) -> usize {
        if extended_length {
            Self::BASE_LENGTH + 1
        } else {
            Self::BASE_LENGTH
        }
    }

    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        extended_length: bool,
    ) -> Result<(), OriginWritingError> {
        write_length(self, extended_length, writer)?;
        writer.write_u8((*self) as u8)?;
        Ok(())
    }
}