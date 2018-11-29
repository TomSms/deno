// Copyright 2018 the Deno authors. All rights reserved. MIT license.
use libdeno::deno_buf;

#[cfg(not(features = "deno2"))]
pub fn deno_snapshot() -> deno_buf {
  let data =
    include_bytes!(concat!(env!("GN_OUT_DIR"), "/gen/snapshot_deno.bin"));

  unsafe { deno_buf::from_raw_parts(data.as_ptr(), data.len()) }
}

#[cfg(feature = "deno2")]
pub fn deno_snapshot2() -> deno_buf {
  let data =
    include_bytes!(concat!(env!("GN_OUT_DIR"), "/gen/snapshot_deno2.bin"));

  unsafe { deno_buf::from_raw_parts(data.as_ptr(), data.len()) }
}
