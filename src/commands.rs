use std::{*,
  io::Write,
};

use crate::{*,
  util::*
};

fn get_service_directory(exe: &str) -> Result<(String, path::PathBuf, path::PathBuf), Box<dyn error::Error>> {

  // directories and labels
  let exe_name = path::Path::new(&exe)
    .file_name()
    .unwrap()
    .to_str()
    .unwrap();
  let label = get_service_label(match path::Path::new(&exe).file_name() {
    None => err!("{} ends in ..", exe),
    Some(name) => match name.to_str() {
      None => err!("{} not valid utf-8", name.to_string_lossy()),
      Some(name) => name,
    },
  });
  let home_path_str = env::var("HOME")?;
  let home_path = path::Path::new(&home_path_str);
  let launch_agent_dir = home_path.join("Library").join("LaunchAgents");
  let plist_path = launch_agent_dir.join(format!("{}.plist", &label));
  let deps_path = launch_agent_dir.join("bundles").join(&label);
  

  Ok((label, plist_path, deps_path))
}

fn get_service_label(exe: &str) -> String {
  format!("{}.{}", constants::DOMAIN, exe)
}



pub fn install(
  bundle: String,
  start_interval: Option<u64>
) -> Result<(), Box<dyn error::Error>> {

  // get the directories and label
  let (label, plist_path, deps_path) = get_service_directory(&bundle)?;
  println!("Label: {}", label);
  println!("Plist path: {}", plist_path.display());

  // remove directories if they exist
  if plist_path.exists() {
    println!("Removing the plist at {}", plist_path.display());
    fs::remove_file(&plist_path)?;
  }
  if deps_path.exists() {
    println!("Removing the bundle at {}", deps_path.display());
    fs::remove_dir_all(&deps_path)?;
  }
  
  
  // copy the bundle
  let old_bundle_path = path::Path::new(&bundle);
  if !old_bundle_path.join("run.sh").exists() {
    err!("{} does not have a run.sh file", old_bundle_path.display());
  }
  let bundle_path = deps_path.join("bundle");
  println!("Creating directory {}", deps_path.display());
  fs::create_dir_all(&bundle_path)?;
  println!("Copying the bundle from {} to {}", old_bundle_path.display(), bundle_path.display());
  let mut bundle_file_paths = collections::VecDeque::<path::PathBuf>::from([old_bundle_path.into()]);
  while let Some(bundle_file_path) = bundle_file_paths.pop_front() {
    println!("Copying path {}", bundle_file_path.display());
    let bundle_file_type = bundle_file_path.metadata()?.file_type();
    if bundle_file_type.is_dir() {
      for path in fs::read_dir(&bundle_file_path)? {
        bundle_file_paths.push_back(path?.path());
      }
    }
    else if bundle_file_type.is_file() {
      let new_path = bundle_path.join(bundle_file_path.strip_prefix(&old_bundle_path)?);
      println!("Copying {} to {}", bundle_file_path.display(), new_path.display());
      fs::copy(&bundle_file_path, &new_path)?;
    }
    else {
      err!("File type {:?} of {} is not supported", bundle_file_type, bundle_file_path.display())
    }
  }

  // update permissions on the run.sh file
  let entry_path = bundle_path.join("run.sh");
  println!("Updating permissions on run.sh");
  if exec!("chmod", "+x", &entry_path) != 0 {
    err!("Failed to update permissions on {}", entry_path.display());
  };


  // create the plist file
  println!("Creating plist file at {}", plist_path.display());
  let mut plist_file = fs::File::create(&plist_path)?;
  plist::prefix(&mut plist_file)?;
  plist::required_args(&mut plist_file, &label, &deps_path)?;
  if let Some(interval) = start_interval {
    plist::start_interval(&mut plist_file, interval)?;
  }
  plist::suffix(&mut plist_file)?;

  // create the entry file
  let run_c_path = deps_path.join("run.c");
  println!("Creating the entry c file at {}", run_c_path.display());
  let mut run_c_file = fs::File::create(&run_c_path)?;
  write!(run_c_file, "
    #include <stdlib.h>
    #include <unistd.h>
    #include <stdio.h>

    int main(int argc, char** argv) {{
      int code = chdir(\"{}\");
      printf(\"chdir returned %d\\n\", code);
      return execl(\"./run.sh\",NULL);
    }}
  ", bundle_path.display())?;
  let run_o_path = deps_path.join("run");
  if exec!("gcc", "-o", &run_o_path, &run_c_path) != 0 {
    err!("Failed to compile {}", run_c_path.display());
  }


  // register the service
  println!("Registering the service");
  exec!("launchctl", "unload", &plist_path);
  if exec!("launchctl", "load", &plist_path) != 0 {
    err!("Failed to register the service");
  };

  Ok(())
}
