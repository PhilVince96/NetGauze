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

#![allow(clippy::upper_case_acronyms)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]
#![deny(unreachable_pub)]
#![deny(unused_allocation)]
#![deny(unused_assignments)]
#![deny(unused_comparisons)]
#![deny(clippy::clone_on_ref_ptr)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::missing_const_for_fn)]

pub mod ie;
pub mod ipfix;
pub mod netflow;
#[cfg(feature = "serde")]
pub mod wire;

use crate::ie::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlowInfo {
    NetFlowV9(netflow::NetFlowV9Packet),
    IPFIX(ipfix::IpfixPacket),
}

/// Errors when crafting a new Set
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum FieldSpecifierError {
    /// Specified field length was out of the range defined by the registry
    InvalidLength(u16, InformationElementId),
}

/// Field Specifier
///
/// ```text
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |E|  Information Element ident. |        Field Length           |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                      Enterprise Number                        |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FieldSpecifier {
    element_id: InformationElementId,
    length: u16,
}

impl FieldSpecifier {
    pub fn new(element_id: InformationElementId, length: u16) -> Result<Self, FieldSpecifierError> {
        if let Some(range) = element_id.length_range() {
            if !range.contains(&length) {
                return Err(FieldSpecifierError::InvalidLength(length, element_id));
            }
        };
        Ok(Self { element_id, length })
    }

    pub const fn element_id(&self) -> InformationElementId {
        self.element_id
    }

    pub const fn length(&self) -> u16 {
        self.length
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum DataSetIdError {
    InvalidId(u16),
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSetId(u16);

/// Values 256 and above are used for Data Sets
pub(crate) const DATA_SET_MIN_ID: u16 = 256;

impl DataSetId {
    pub const fn new(id: u16) -> Result<Self, DataSetIdError> {
        if id < DATA_SET_MIN_ID {
            Err(DataSetIdError::InvalidId(id))
        } else {
            Ok(Self(id))
        }
    }

    #[inline]
    pub const fn id(&self) -> u16 {
        self.0
    }
}

impl Deref for DataSetId {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
