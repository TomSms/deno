use deno_dir::CodeFetchOutput;

use futures::sync::mpsc;

lazy_static!() {
  static ref (TX, RX) = mpsc::<CodeFetchOutput>::channel();
  static ref INSTANCE = Option<Arc<CompilerIsolate>> = None;
}

static START: Once = Once::new();


fn compiler_main() {
  let snapshot = snapshot::deno_snapshot();
  let worker = Worker::new(snapshot);
  tokio_util::init(|| {
    worker
      .execute("deno_main.js", "compilerMain();")
      .unwrap_or_else(print_err_and_exit);
    worker.event_loop().unwrap_or_else(print_err_and_exit);
  });
}

/// Returns a modified version of the input with the maybe_output_code filled
/// in.
/// Called from runtime thread.
fn compile(&self, out0: CodeFetchOutput) -> Result<CodeFetchOutput, JSError> {
  // Lazily start the isolate.
  START.call_once(|| {
    // Start new thread for the compiler.
    thread::spawn(compiler_main);
  });

  tokio::spawn(TX.send(out));

  let r = rx.recv().wait();
}



impl CompilerIsolate {
  fn get_instance() -> Arc<CompilerIsolate> {
    // 
  }

}
