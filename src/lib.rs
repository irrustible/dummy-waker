// Copyright (c) 2021 James Laver
// 
// Licensed under Apache License, Version 2.0
// (https://www.apache.org/licenses/LICENSE-2.0), with LLVM Exceptions
// (https://spdx.org/licenses/LLVM-exception.html).
//
// Unless you explicitly state otherwise, any contribution
// intentionally submitted for inclusion in the work by you, as
// defined in the Apache-2.0 license, shal l be licensed as above,
// without any additional terms or conditions.
#![no_std]
use core::task::{RawWaker, RawWakerVTable, Waker};

const VTABLE: RawWakerVTable =
    RawWakerVTable::new(
        |data: *const ()| RawWaker::new(data, &VTABLE),
        |_data: *const ()| (),
        |_data: *const ()| (),
        |_data: *const ()| (),
    );
    
const RAW: RawWaker = RawWaker::new(
    (&VTABLE as *const RawWakerVTable).cast(),
    &VTABLE
);

/// Returns a Waker that does absolutely nothing.
pub fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(RAW) }
}
