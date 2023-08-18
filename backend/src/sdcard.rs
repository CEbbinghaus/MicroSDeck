use std::{path::Path, fs::read_to_string};

// Based on https://www.cameramemoryspeed.com/sd-memory-card-faq/reading-sd-card-cid-serial-psn-internal-numbers/

pub fn is_card_inserted() -> bool {
    std::fs::metadata("/sys/block/mmcblk0").is_ok()
}

pub fn get_card_cid() -> Option<String> {
    read_to_string("/sys/block/mmcblk0/device/cid").ok()
}
