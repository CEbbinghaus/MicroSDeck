use std::{
	fs::{self, read_to_string, DirEntry},
	io,
};

pub const DEFAULT_MOUNT: &str = "mmcblk0p1";
pub const LIBRARY_FOLDER_FILE: &str = "libraryfolder.vdf";

use crate::err::Error;

pub fn is_card_inserted() -> bool {
	std::fs::metadata("/sys/block/mmcblk0").is_ok()
}

// Based on https://www.cameramemoryspeed.com/sd-memory-card-faq/reading-sd-card-cid-serial-psn-internal-numbers/
pub fn get_card_cid() -> Option<String> {
	read_to_string("/sys/block/mmcblk0/device/cid")
		.map(|v| v.trim().to_string())
		.ok()
}

pub fn has_libraryfolder(mount: &Option<String>) -> bool {
	std::fs::metadata(format!(
		"/run/media/{}/{}",
		mount.clone().unwrap_or(DEFAULT_MOUNT.into()),
		LIBRARY_FOLDER_FILE
	))
	.is_ok()
}

pub fn read_libraryfolder(mount: &Option<String>) -> io::Result<String> {
	std::fs::read_to_string(format!(
		"/run/media/{}/{}",
		mount.clone().unwrap_or(DEFAULT_MOUNT.into()),
		LIBRARY_FOLDER_FILE
	))
}

pub fn get_steam_acf_files(
	mount: &Option<String>,
) -> Result<impl Iterator<Item = DirEntry>, Error> {
	Ok(fs::read_dir(format!(
		"/run/media/{}/steamapps/",
		mount.clone().unwrap_or(DEFAULT_MOUNT.into())
	))?
	.filter_map(Result::ok)
	.filter(|f| f.path().extension().unwrap_or_default().eq("acf")))
}
