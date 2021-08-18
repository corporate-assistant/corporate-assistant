pub mod configuration {
    use dirs::home_dir;
    use std::fs::create_dir;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::path::PathBuf;

    pub struct CAConfig {
        config_dir : PathBuf,
    }

    impl CAConfig {
        pub fn new() -> Self {
            // Check if directory exists
            // and create if needed
            let mut config_dir : PathBuf;
            let hd = home_dir();
            match hd {
                Some(hd) => { 
                    config_dir = hd.clone();
                    config_dir.push(".corporate_assistant");

                    if config_dir.exists() != true {
                        create_dir(&config_dir).expect(&format!("Unable to create directory: {}", config_dir.to_str().unwrap())); 
                    }
                },  
                None => panic!("Error: home directory not found!"),
            }
            Self{
                config_dir : config_dir,
            }
        }

        // If no custom script found then make a default one
        pub fn get_custom_action_script(&self) -> PathBuf {
           let base_script_name = if cfg!(target_os = "windows") {
                "custom_script.bat"
           } else {
                "custom_script.sh"
           };
           let mut script_name = PathBuf::from(&self.config_dir);
           script_name.push(base_script_name);
           let script_name = Path::new(&script_name); 
           // No script then put default script value there
           if script_name.exists() == false {
            let default_content = if cfg!(target_os = "windows") {
                ""
            } else {
                "#!/bin/bash
                #gvim -S /home/jczaja/Paddle/cache.vim
                #gnome-terminal -e 'cd /home/jczaja/Paddle'
                #gnome-terminal -e 'sh -c \"ssh mylogin@myserver\"' #\"sudo su jczaja\"
                #gnome-terminal -- tmux"
            };
            let mut file = File::create(script_name).expect(&format!("Unable to create default custom script: {}", script_name.to_str().unwrap()));
            file.write_all(default_content.as_bytes()).expect("Failure in writting custom script");
           }
           script_name.to_path_buf()
        }
    }
}
