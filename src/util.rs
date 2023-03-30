
// #[macro_export]
macro_rules! exec {
  ($arg1:expr,$($args:expr),*) => {{
    let output = process::Command::new($arg1)
        $(
          .arg($args)
        )*
        .output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    output.status.code().unwrap_or(1)
  }};
}

pub(crate) use exec;

// #[macro_export]
macro_rules! err {
  ($($arg:tt)*) => {{
      println!("Error: {}", format!($($arg)*));
      process::exit(1);
  }};
}
pub(crate) use err;