use std::process::Command;

fn main() {
  let mut cmd = Command::new("git");
  cmd.arg("submodule");
  cmd.args(["update", "--init", "--depth", "1", "--recursive"]);
  println!("{:?}", cmd);

  let echo = Command::new("echo").args(["hello"]).output().unwrap();
  println!(
    "\x1b[31mINFO:{}\x1b[0m",
    String::from_utf8_lossy(&echo.stdout)
  );
  println!(
    "\x1b[32mERROR:{}\x1b[0m",
    String::from_utf8_lossy(&echo.stderr)
  );
}
