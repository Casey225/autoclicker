# Autoclicker

Tool that binds a key to click the left mouse button repeatedly with a specified delay.
Linux only.

# Installation

Dependancies: `cargo` (for compilation only), `evtest`, and `xdotool`

Download the source and compile it with

`cargo build --release`

Copy `target/release/autoclicker` and `ac_settings.ron` into a directory of your choice.

**THESE FILES MUST BE IN THE SAME DIRECTORY**

# Keybind setup
Find the device you would like to bind your enable key too in `/dev/input/by-id/`.

The device id will be either `usb-KEYBOARD-NAME-event-kbd` or `usb-MOUSE-NAME-event-mouse`.

Then run

`sudo evtest --grab ENTER_DEVICE_ID_HERE`

Click the button/key you would like to bind, then make note of the key id. It should look like KEY_XXXX or BTN_XXXX.

Edit the `ac_settings.ron` file with your specific device-id and key-id.

# Execution
Run with `sudo ./autoclicker`
