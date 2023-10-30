use std::fs::{self, read_to_string, DirEntry};

use crate::err::Error;

pub const STEAM_LIB_FILE: &'static str = "/run/media/mmcblk0p1/libraryfolder.vdf";
pub const STEAM_LIB_FOLDER: &'static str = "/run/media/mmcblk0p1/steamapps/";

pub fn is_card_inserted() -> bool {
	std::fs::metadata("/sys/block/mmcblk0").is_ok()
}

// Based on https://www.cameramemoryspeed.com/sd-memory-card-faq/reading-sd-card-cid-serial-psn-internal-numbers/
pub fn get_card_cid() -> Option<String> {
	read_to_string("/sys/block/mmcblk0/device/cid")
		.map(|v| v.trim().to_string())
		.ok()
}

pub fn is_card_steam_formatted() -> bool {
	std::fs::metadata("/run/media/mmcblk0p1/libraryfolder.vdf").is_ok()
}

pub fn get_steam_acf_files() -> Result<impl Iterator<Item = DirEntry>, Error> {
	Ok(fs::read_dir(STEAM_LIB_FOLDER)?
		.into_iter()
		.filter_map(Result::ok)
		.filter(|f| f.path().extension().unwrap_or_default().eq("acf")))
}
