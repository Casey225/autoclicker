use std::process::Command;
use std::process;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::env;

const SETTING_FILE_NAME : &str = "ac_settings.ron";

#[derive(Deserialize)]
struct Settings {
    delay: f32,

    dev_id: String,
    key: String
}

fn main() {
    
    // Appending current directory to SETTING_FILE_NAME
    let path = format!["{}/{}", env::current_dir().expect(" ").into_os_string().into_string().expect(" "), SETTING_FILE_NAME];

    // Opening file at appended path
    let f = File::open(&path).expect(format!["Failed opening {}", SETTING_FILE_NAME]);

    // Attempting to deserialize .ron file to a Settings struct
    let settings: Settings = match from_reader(f) {
        Ok(s) => s,
        Err(_e) => {
            eprintln!("Invalid {} syntax.", SETTING_FILE_NAME);
            process::exit(1);
        }
    };

    loop {
        // Querying mouse side button with evtest
        let output = Command::new("sudo")
            .arg("evtest")
            .arg("--query")
            .arg(format!["/dev/input/by-id/{}", &settings.dev_id])
            .arg("EV_KEY")
            .arg(&settings.key)
            .status()
            .expect("Could not complete device query. ");

        // Key is pressed
        if !output.success() {
            // Clicking once with xdotool
            // Ignoring result
            match Command::new("xdotool")
            .arg("click")
            .arg("1")
            .spawn() {
                _ => {}
            };

            // Sleeping for delay
            Command::new("sleep")
            .arg(format!["{}", settings.delay])
            .spawn()
            .expect("Sleep failed at - ");
        }
    };
}
