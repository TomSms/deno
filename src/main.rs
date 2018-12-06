// Copyright 2018 the Deno authors. All rights reserved. MIT license.
extern crate dirs;
extern crate flatbuffers;
extern crate getopts;
extern crate http;
extern crate hyper;
extern crate hyper_rustls;
extern crate libc;
extern crate rand;
extern crate remove_dir_all;
extern crate ring;
extern crate rustyline;
extern crate serde_json;
extern crate source_map_mappings;
extern crate tempfile;
extern crate tokio;
extern crate tokio_executor;
extern crate tokio_fs;
extern crate tokio_io;
extern crate tokio_process;
extern crate tokio_threadpool;
extern crate url;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate futures;

pub mod deno_dir;
pub mod errors;
pub mod flags;
mod fs;
mod http_body;
mod http_util;
pub mod isolate;
pub mod js_errors;
pub mod libdeno;
pub mod msg;
pub mod msg_util;
pub mod ops;
pub mod permissions;
mod repl;
pub mod resources;
pub mod snapshot;
mod tokio_util;
mod tokio_write;
pub mod version;

#[cfg(unix)]
mod eager_unix;

use std::env;
use std::sync::Arc;

static LOGGER: Logger = Logger;

struct Logger;

impl log::Log for Logger {
  fn enabled(&self, metadata: &log::Metadata) -> bool {
    metadata.level() <= log::max_level()
  }

  fn log(&self, record: &log::Record) {
    if self.enabled(record.metadata()) {
      println!("{} RS - {}", record.level(), record.args());
    }
  }
  fn flush(&self) {}
}

fn print_err_and_exit(err: js_errors::JSError) {
  // TODO Currently tests depend on exception going to stdout. It should go
  // to stderr. https://github.com/denoland/deno/issues/964
  println!("{}", err.to_string());
  std::process::exit(1);
}

#[cfg(feature = "deno2")]
fn main2(state: Arc<isolate::IsolateState>) {
  let snapshot = snapshot::deno_snapshot2();

  let state2 = state.clone();

  let isolate = isolate::Isolate::new(snapshot, state, ops::dispatch);
  tokio_util::init(|| {
    isolate
      .execute("main2.js", "deno2Bootstrap()")
			.unwrap_or_else(print_err_and_exit);

    if state2.argv.len() > 1 {
      let cwd_path = std::env::current_dir().unwrap();
      let cwd = fs::normalize_path(cwd_path.as_ref()) + "/";
      let cwd = cwd.as_ref();

      let out = state2.dir.code_fetch(&state2.argv[1], &cwd).unwrap();
      println!("fetching input {}", out.filename);

      isolate
        .execute(&out.filename, &out.source_code)
      	.unwrap_or_else(print_err_and_exit);
    }

    isolate.event_loop().unwrap_or_else(print_err_and_exit);
  });
}

#[cfg(not(feature = "deno2"))]
fn main1(state: Arc<isolate::IsolateState>) {
  let snapshot = snapshot::deno_snapshot();
  let isolate = isolate::Isolate::new(snapshot, state, ops::dispatch);
  tokio_util::init(|| {
    isolate
      .execute("deno_main.js", "denoMain();")
      .unwrap_or_else(print_err_and_exit);
    isolate.event_loop().unwrap_or_else(print_err_and_exit);
  });
}

fn main() {
  // Rust does not die on panic by default. And -Cpanic=abort is broken.
  // https://github.com/rust-lang/cargo/issues/2738
  // Therefore this hack.
  std::panic::set_hook(Box::new(|panic_info| {
    eprintln!("{}", panic_info.to_string());
    std::process::abort();
  }));

  log::set_logger(&LOGGER).unwrap();
  let args = env::args().collect();
  let (flags, rest_argv, usage_string) =
    flags::set_flags(args).unwrap_or_else(|err| {
      eprintln!("{}", err);
      std::process::exit(1)
    });

  if flags.help {
    println!("{}", &usage_string);
    std::process::exit(0);
  }

  log::set_max_level(if flags.log_debug {
    log::LevelFilter::Debug
  } else {
    log::LevelFilter::Info
  });

  let state = Arc::new(isolate::IsolateState::new(flags, rest_argv));

	#[cfg(feature = "deno2")]
	main2(state);
	#[cfg(not(feature = "deno2"))]
	main1(state);
}
