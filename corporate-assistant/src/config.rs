pub mod configuration {
    use dirs::home_dir;
    use serde::Deserialize;
    use std::fs::create_dir;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::path::PathBuf;
    use toml;

    pub struct CAConfig {
        config_dir: PathBuf,
    }

    #[derive(Deserialize)]
    pub struct OrganizationConfig {
        pub restaurants: Option<Vec<String>>,
        pub recognition: Option<Vec<String>>,
        pub holidays: Option<Vec<String>>,
        pub proxy: Option<Vec<String>>,
        pub jira: Option<Vec<String>>,
    }

    impl CAConfig {
        pub fn new() -> Self {
            // Check if directory exists
            // and create if needed
            let mut config_dir: PathBuf;
            let hd = home_dir();
            match hd {
                Some(hd) => {
                    config_dir = hd.clone();
                    config_dir.push(".corporate_assistant");

                    if config_dir.exists() != true {
                        create_dir(&config_dir).expect(&format!(
                            "Unable to create directory: {}",
                            config_dir.to_str().unwrap()
                        ));
                    }
                }
                None => panic!("Error: home directory not found!"),
            }
            Self {
                config_dir: config_dir,
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
                let mut file = File::create(script_name).expect(&format!(
                    "Unable to create default custom script: {}",
                    script_name.to_str().unwrap()
                ));
                file.write_all(default_content.as_bytes())
                    .expect("Failure in writting custom script");
            }
            script_name.to_path_buf()
        }

        // TODO(jczaja) Make next two methods unified into one
        pub fn get_repos_config(&self) -> PathBuf {
            let mut config_name = PathBuf::from(&self.config_dir);
            config_name.push("paddle.toml"); //TODO: make it given from commandline
            let config_name = Path::new(&config_name);
            if config_name.exists() == false {
                let default_content = "
                [github]
                user = \"<your github id>\"
                token = \"<your github token>\"
                repos = [\"Paddle\"]
                url = \"https://api.github.com/graphql\"

                [jira]
                user = \"<username to JIRA>\"
                project = \"PADDLEQ\"
                url = [\"<URL of JIRA server>\"]
                ";
                let mut file = File::create(config_name).expect(&format!(
                    "Unable to create default repos config: {}",
                    config_name.to_str().unwrap()
                ));
                file.write_all(default_content.as_bytes())
                    .expect("Failure in writting custom script");
            }
            config_name.to_path_buf()
        }

        // Generic based on
        pub fn get_organization_config(&self, organization_config_name: &str) -> PathBuf {
            // if no config then fill with default values
            let mut config_name = PathBuf::from(&self.config_dir);
            config_name.push(organization_config_name);
            let config_name = Path::new(&config_name);
            if config_name.exists() == false {
                let default_content = " 
                restaurants = [\"<URL of first nearby resturant>\", \"<URL of second nearby resturant>\", \"<yet another URL of some reachable canteen>\"]
                recognition = [\"<URL to website with recognition>\"]
                holidays = [\"<URL to website with holidays request form>\"]
                proxy = [\"<URL of proxy servers>\"]
                ";
                let mut file = File::create(config_name).expect(&format!(
                    "Unable to create default repos config: {}",
                    config_name.to_str().unwrap()
                ));
                file.write_all(default_content.as_bytes())
                    .expect("Failure in writting custom script");
            }
            config_name.to_path_buf()
        }

        pub fn get_mailer_config(&self) -> PathBuf {
            let mut config_name = PathBuf::from(&self.config_dir);
            config_name.push("email_client.toml");

            let config_name = Path::new(&config_name);

            if config_name.exists() == false {
                let default_content = "[email]
                    login = \"<your Intel username\"
                    password = \"<your Intel password\"
                    server = \"<an smtp server for email communication>\"
                    port = 0
                    from = \"<sender>\"
                    to = \"<receiver>\"
                ";

                let mut file = File::create(config_name).expect(&format!(
                    "Unable to create default email client config: {}",
                    config_name.to_str().unwrap()
                ));

                file.write_all(default_content.as_bytes())
                    .expect("Failure saving email client config file");
            }

            config_name.to_path_buf()
        }
    }

    pub fn parse_organization_config(config_name: &PathBuf) -> OrganizationConfig {
        let mut file = File::open(config_name).expect(&format!(
            "Error opening organization config {}\n",
            config_name.display()
        ));
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(&format!(
            "Error reading organization config {}\n",
            config_name.display()
        ));
        let config: OrganizationConfig = toml::from_str(&contents).unwrap();
        config
    }
}
