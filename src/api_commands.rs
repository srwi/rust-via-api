#![allow(non_camel_case_types)]

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy)]
pub enum ApiCommand {
    GET_PROTOCOL_VERSION = 0x01,
    GET_KEYBOARD_VALUE = 0x02,
    SET_KEYBOARD_VALUE = 0x03,
    DYNAMIC_KEYMAP_GET_KEYCODE = 0x04,
    DYNAMIC_KEYMAP_SET_KEYCODE = 0x05,
    //  DYNAMIC_KEYMAP_CLEAR_ALL = 0x06,
    CUSTOM_MENU_SET_VALUE = 0x07,
    CUSTOM_MENU_GET_VALUE = 0x08,
    CUSTOM_MENU_SAVE = 0x09,

    EEPROM_RESET = 0x0a,
    BOOTLOADER_JUMP = 0x0b,
    DYNAMIC_KEYMAP_MACRO_GET_COUNT = 0x0c,
    DYNAMIC_KEYMAP_MACRO_GET_BUFFER_SIZE = 0x0d,
    DYNAMIC_KEYMAP_MACRO_GET_BUFFER = 0x0e,
    DYNAMIC_KEYMAP_MACRO_SET_BUFFER = 0x0f,
    DYNAMIC_KEYMAP_MACRO_RESET = 0x10,
    DYNAMIC_KEYMAP_GET_LAYER_COUNT = 0x11,
    DYNAMIC_KEYMAP_GET_BUFFER = 0x12,
    DYNAMIC_KEYMAP_SET_BUFFER = 0x13,
    DYNAMIC_KEYMAP_GET_ENCODER = 0x14,
    DYNAMIC_KEYMAP_SET_ENCODER = 0x15,
    // DEPRECATED:
    // BACKLIGHT_CONFIG_SET_VALUE = 0x07,
    // BACKLIGHT_CONFIG_GET_VALUE = 0x08,
    // BACKLIGHT_CONFIG_SAVE = 0x09,
}
