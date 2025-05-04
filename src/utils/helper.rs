use crate::constants;
use crate::utils::api_error::api_error;
use std::{error::Error, thread, path::PathBuf, string::String};
use std::fs::{self, OpenOptions, File};
use std::io::{self, Write};
use windows::{core::*, Win32::UI::WindowsAndMessaging::*};
extern crate ini;
use ini::Ini;
use std::collections::HashMap;
use sysinfo::System;
use std::time::Duration;

/// This function will return the name of the installed modlist by reading the symbolic link in the DLC folder.
/// It will return an empty string if none.
pub fn get_installed_modlist_name() -> core::result::Result<String, Box<dyn Error>> {

    let link = std::env::current_dir()
        .unwrap()
        .join(constants::WITCHER_GAME_ROOT)
        .join("dlc");

    if link.is_symlink() {
        let path: PathBuf = fs::read_link(&link)?;

        let s = path.to_str().expect("missing");

        let u = s.split(r"\").count();

        let v: Vec<&str> = s.split(r"\").collect();

        let ret = v[u - 2];

        Ok(String::from(ret))
    } else {
        Ok(String::from(""))
    }
}

/// This function will check installed modlist with the expected name.
pub fn is_installed(name: &String) -> bool {
    if name == get_installed_modlist_name().unwrap().as_str() {
      return true;
    }
    return false;
  }

/// This function will check if the Mod Manager home directory exists.  
pub fn check_mod_mgr_env() -> bool {
    #[cfg(debug_assertions)]
    true;

    let home = dirs::document_dir()
    .ok_or(api_error(
      "Internal server error: could not find the Documents directory",
    )).unwrap();

    if home.join(constants::MODMANAGER_PATH).is_dir() {
        return true;
    }
    
    return false;    
}

/// This function will check if the Script Merger executable exists in the specified path.
pub fn check_script_mgr_env() -> bool {
    #[cfg(debug_assertions)]
    true;

    let scriptmerger_path = std::env::current_dir()
    .unwrap()
    .join(constants::SCRIPTMERGER_PATH);

    let scriptmerger_exe = scriptmerger_path
        .join(constants::SCRIPTMERGER_EXE_NAME);
    
    if scriptmerger_path.is_dir() && scriptmerger_exe.is_file() {
        return true;
    }

    return false;
}

/// Function check if create a symbolic link is possible
/// This function will check if the current user has the necessary permissions to create symbolic links.
pub fn test_symlink() {
    let path = std::env::current_dir().unwrap();
    let link_path = path.join("link_to_dir");
    
    symlink::symlink_dir(&path, &link_path).ok();
    if link_path.exists() && link_path.is_symlink() {
      symlink::remove_symlink_dir(&link_path).ok();
    }
    else {
        unsafe {
            MessageBoxW(None, w!("Can not create Symbolic Links. Run program as Administrator, or turn \"Developer Mode\" on."), w!("Error"), MB_ICONERROR);
            std::process::exit(0);
        }
    }
}

/// Function to merge INI files
/// This function will read the content of the source file and append it to the destination file.
pub fn merge_inputs_settings(source: PathBuf, destination: PathBuf) -> io::Result<()> {
    // Read the content of the source file
    let content = fs::read(&source)?;
    println!("merging files from {} to {}", source.display(), destination.display());
    // Open the destination file in append mode
    let mut dest_file = OpenOptions::new()
        .append(true)
        .open(&destination)?;

    // Write the content to the destination file
    dest_file.write_all(&content)?;

    Ok(())
}

/// Function to remove duplicate entries from the INI file
/// This function will read the INI file, remove duplicates, and write the unique entries back to the file.
pub fn truncate_input_settings( target: PathBuf) {
    if !fs::metadata(&target).is_ok() {
        return;
    }
        
    let conf = Ini::load_from_file(&target).unwrap();
    let mut new_conf = Ini::new();
    
    // Create a HashMap to track duplicates
    let mut seen = HashMap::new();
    
    // Iterate through sections and keys
    for (sec, prop) in conf.iter() {
        for (key, value) in prop.iter() {
            // Create a unique key for each entry
            let unique_key = format!("{}_{}_{}", sec.as_ref().unwrap_or(&"").to_string().trim(), key.to_string().trim(), value.to_string().trim());
            
            // Ommit duplicates line if necessary
            if !seen.contains_key(&unique_key) {
                
                // Add the single key/value into the new INI object
                new_conf.with_section(Some(sec.clone().unwrap_or_else(|| "")))
                    .add(key, value);

                // Add the not seen unique key to the HashMap
                seen.insert(unique_key, String::new());
            }
        }
    }
    
    // Write the new INI content back to a file
    let mut file = File::create(target).ok().unwrap();
    new_conf.write_to(&mut file).ok().unwrap();
    
    
}

/// Function to kill all Witcher 3 & tools related processes
pub fn kill_apps() {
    let mut s = System::new_all();
    let mut t = System::new_all();
    s.refresh_all();

    for (_pid, process) in s.processes() {

        if process.name() == "witcher3.exe" {
            process.kill();
            loop {
                t.refresh_all();
                if t.process(process.pid()).is_none() {                    
                    break;
                }
                thread::sleep(Duration::from_millis(500));
            }     
        }
        
        if process.name() == "TheWitcher3ModManager.exe" {
            process.kill();
            loop {
                t.refresh_all();
                if t.process(process.pid()).is_none() {                    
                    break;
                }
                thread::sleep(Duration::from_millis(500));
            }                
        }

        if process.name() == "WitcherScriptMerger.exe" {
            process.kill();
            loop {
                t.refresh_all();
                if t.process(process.pid()).is_none() {                    
                    break;
                }
                thread::sleep(Duration::from_millis(500));
            }     
        }        
    }
}

/// Function to copy files only from one directory to another
pub fn copy_dir_files(src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if !ty.is_dir() {            
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}