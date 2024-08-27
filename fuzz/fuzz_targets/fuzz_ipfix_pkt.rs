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

#![no_main]
use libfuzzer_sys::fuzz_target;
use netgauze_flow_pkt::ipfix::IpfixPacket;
use netgauze_parse_utils::{ReadablePduWithOneInput, Span};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

fuzz_target!(|data: &[u8]| {
    let mut buf = data;
    let mut templates_map = HashMap::new();
    while let Ok((retbuf, _msg)) = IpfixPacket::from_wire(Span::new(buf), templates_map.clone()) {
        buf = retbuf.fragment();
    }
});
