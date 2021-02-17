use std::process::Command;
use std::process;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::env;

#[derive(Deserialize)]
struct Settings {
    delay: f32,

    dev_id: String,
    key: String
}

fn main() {
    
    // Reading input arguments from settings file
    let path = format!["{}/ac_settings.ron", env!("CARGO_MANIFEST_DIR")];

    let f = File::open(&path).expect("Failed opening file");

    let settings: Settings = match from_reader(f) {
        Ok(s) => s,
        Err(_e) => {
            eprintln!("Could not open file: settings.ron");
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
