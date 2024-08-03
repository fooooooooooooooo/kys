fn main() {
  #[cfg(target_os = "linux")]
  sysrq_trigger();
  bsod::bsod();
}

#[cfg(target_os = "linux")]
fn sysrq_trigger() {
  use std::fs::File;
  use std::io::Write;
  if let Ok(mut f) = File::create("/proc/sysrq-trigger") {
    let _ = f.write_all(b"l");
  }
}
