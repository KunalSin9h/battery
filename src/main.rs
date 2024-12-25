use gpui::{div, prelude::*, rgb, App, AppContext, SharedString, ViewContext, WindowOptions};
use std::{
    thread,
    time,
    fs::read_to_string
};

/**
Battery Supply Information is in file "$HOME/sys/class/power_supply/BAT0/uevent"

It includes (each per line):

    POWER_SUPPLY_NAME=BAT0
    POWER_SUPPLY_TYPE=Battery
    POWER_SUPPLY_STATUS=Discharging
    POWER_SUPPLY_PRESENT=1
    POWER_SUPPLY_TECHNOLOGY=Li-ion
    POWER_SUPPLY_CYCLE_COUNT=0
    POWER_SUPPLY_VOLTAGE_MIN_DESIGN=14800000
    POWER_SUPPLY_VOLTAGE_NOW=15985000
    POWER_SUPPLY_CURRENT_NOW=346000
    POWER_SUPPLY_CHARGE_FULL_DESIGN=2800000
    POWER_SUPPLY_CHARGE_FULL=2365000
    POWER_SUPPLY_CHARGE_NOW=2008000
    POWER_SUPPLY_CAPACITY=84
    POWER_SUPPLY_CAPACITY_LEVEL=Normal
    POWER_SUPPLY_MODEL_NAME=DELL GR43747
    POWER_SUPPLY_MANUFACTURER=SMP
    POWER_SUPPLY_SERIAL_NUMBER=10252

Here POWER_SUPPLY_STATUS=Discharging means its currently not plugged-in
but when plugged-in we will have status change to Charging
*/
const BATTERY_INFO_SOURCE_FILE: &str = "/sys/class/power_supply/BAT0/uevent";

struct WarningWindow {
    text: SharedString,
}

impl Render for WarningWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffe3))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0x000000))
            .child(format!("🪫 Battery Low!, {}!", &self.text))
    }
}


fn main() {
    // Continuously check for battery status
    loop {
        match read_to_string(BATTERY_INFO_SOURCE_FILE) {
            Ok(info) => check_and_take_action(info),
            Err(err) => {
                println!("Error while getting battery info: {}", err);
            }
        }
        thread::sleep(time::Duration::from_secs(10));
    }
}

fn check_and_take_action(data: String) {
    if let Some(battery_percentage) = data.lines()
        .find(|line| line.starts_with("POWER_SUPPLY_CAPACITY="))
        .and_then(|line| line.split('=').nth(1))
        .and_then(|value| value.parse::<u16>().ok()) {


        if battery_percentage < 90 {
            App::new().run(|cx: &mut AppContext| {
                cx.open_window(WindowOptions::default(), |cx| {
                    cx.new_view(|_cx| WarningWindow {
                        text: "Plugin it right now".into(),
                    })
                })
                    .unwrap();
            });
        }
    } else {
        println!("Failed to get battery percentage");
    }
}
