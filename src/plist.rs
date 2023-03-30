use std::*;
use std::io::Write;



pub fn prefix(file: &mut fs::File) -> Result<(), Box<dyn error::Error>> {
  write!(file, "
    <?xml version=\"1.0\" encoding=\"UTF-8\"?>
    <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
    <plist version=\"1.0\">
    <dict>
  ")?;
  Ok(())
}


pub fn required_args(file: &mut fs::File, label: &str, dir: &path::Path) -> Result<(), Box<dyn error::Error>> {
  write!(file, "
      <key>Label</key>
      <string>{label}</string>
      <key>ProgramArguments</key>
      <array>
          <string>{dir}/run</string>
      </array>
      <key>RunAtLoad</key>
      <true/>
      <key>StandardOutPath</key>
      <string>/tmp/{label}.log</string>
      <key>StandardErrorPath</key>
      <string>/tmp/{label}.log</string>
  ", label=label, dir=dir.display())?;
  Ok(())
}

pub fn start_interval(file: &mut fs::File, interval: u64) -> Result<(), Box<dyn error::Error>>  {
  write!(file, "
      <key>StartInterval</key>
      <integer>{interval}</integer>
  ", interval=interval)?;
  Ok(())
}



pub fn suffix(file: &mut fs::File) -> Result<(), Box<dyn error::Error>>  {
  write!(file, "
    </dict>
    </plist>
  ")?;
  Ok(())
}