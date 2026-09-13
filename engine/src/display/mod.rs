// HACK : I'll use it later

use crate::errors::set_error;
use display_info::DisplayInfo;

pub struct Display {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    is_primary: bool,
}

impl Display {
    pub fn new(id: u32, width: u32, height: u32, is_primary: bool) -> Self {
        Self {
            id,
            width,
            height,
            is_primary,
        }
    }
}

pub fn check_displays() -> Vec<Display> {
    let mut displays: Vec<Display> = vec![];
    match DisplayInfo::all() {
        Ok(display_infos) => {
            for display in display_infos {
                displays.push(Display::new(
                    display.id,
                    display.width,
                    display.height,
                    display.is_primary,
                ));
            }
        }
        Err(_e) => set_error(0.11),
    }
    return displays;
}

pub fn get_primary_display(displays: &Vec<Display>) -> &Display {
    if !displays.is_empty() {
        for display in displays {
            if display.is_primary {
                return display;
            } else {
                if display.id == displays.last().unwrap().id {
                    return display;
                }
            }
        }
        return &displays[0];
    } else {
        set_error(0.12);
        panic!(""); // NOTE : Impossible to reach
    }
}
