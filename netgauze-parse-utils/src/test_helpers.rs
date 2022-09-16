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

//! Various functions used in testing the correctness or
//! serializing/deserializing wire protocols

use crate::{ReadablePDU, Span};
use netgauze_locate::BinarySpan;
use nom::IResult;
use std::fmt::Debug;

/// Helper method to combine multiple vectors into one
pub fn combine(v: Vec<&[u8]>) -> Vec<u8> {
    v.iter()
        .flat_map(|x| x.iter())
        .cloned()
        .collect::<Vec<u8>>()
}

/// Fancier assert to for more meaningful error messages
pub fn test_parsed_completely<'a, T, E>(input: &'a [u8], expected: &T) -> T
where
    T: ReadablePDU<'a, E> + PartialEq + Debug,
    E: Debug,
{
    let parsed = <T as ReadablePDU<E>>::from_wire(Span::new(input));
    assert!(parsed.is_ok(), "Message failed parsing, while expecting it to pass.\n\tExpected : {:?}\n\tParsed msg: {:?}", expected, parsed);
    let (span, value) = parsed.unwrap();
    assert_eq!(&value, expected);
    assert_eq!(
        span.fragment().len(),
        0,
        "Not all the input is consumed by the parser, didn't consume: {:?}",
        span
    );
    value
}

/// Fancier assert to for more meaningful error messages
pub fn test_parse_error<'a, 'b, 'c, T, E>(input: &'a [u8], expected_err: &E)
where
    T: ReadablePDU<'a, E> + Debug,
    E: Debug + Eq,
{
    let parsed: IResult<BinarySpan<&[u8]>, T, E> =
        <T as ReadablePDU<E>>::from_wire(Span::new(input));
    assert!(
        parsed.is_err(),
        "Message was parsed, while expecting it to fail.\n\tExpected : {:?}\n\tParsed msg: {:?}",
        expected_err,
        parsed
    );

    if let Err(nom::Err::Error(parsed_error)) = parsed {
        assert_eq!(&parsed_error, expected_err);
    } else {
        panic!(
            "Expected the test to fail with Err(nom::Err:Err(x)) but it didn't. Got {:?} instead",
            parsed
        );
    }
}