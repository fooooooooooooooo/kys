fn main() {
  #[cfg(target_os = "linux")]
  kill_linux();
  #[cfg(target_os = "macos")]
  kill_macos();
  #[cfg(target_os = "windows")]
  bsod::bsod();
}

#[cfg(target_os = "linux")]
fn kill_linux() {
  use std::fs::File;
  use std::io::Write;
  use std::process::Command;

  if let Ok(mut f) = File::create("/proc/sysrq-trigger") {
    let _ = f.write_all(b"l");
  }
  let _ = Command::new("shutdown").arg("-r").arg("now").output();
  let _ = Command::new("reboot").output();
}

#[cfg(target_os = "macos")]
fn kill_macos() {
  use std::process::Command;
  let _ = Command::new("dtrace")
    .arg("-w")
    .arg("-n")
    .arg("BEGIN{ panic();}")
    .output();
  let _ = Command::new("killall").arg("kernel_task").output();
  let _ = Command::new("shutdown").arg("-r").arg("now").output();
  let _ = Command::new("reboot").output();
}
