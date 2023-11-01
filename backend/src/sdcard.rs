use std::{
	fs::{self, read_to_string, DirEntry},
	io,
};

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
		"/run/media/{}/libraryfolder.vdf",
		mount.clone().unwrap_or("mmcblk0p1".into())
	))
	.is_ok()
}

pub fn read_libraryfolder(mount: &Option<String>) -> io::Result<String> {
	std::fs::read_to_string(format!(
		"/run/media/{}/libraryfolder.vdf",
		mount.clone().unwrap_or("mmcblk0p1".into())
	))
}

pub fn get_steam_acf_files(
	mount: &Option<String>,
) -> Result<impl Iterator<Item = DirEntry>, Error> {
	Ok(fs::read_dir(format!(
		"/run/media/{}/steamapps/",
		mount.clone().unwrap_or("mmcblk0p1".into())
	))?
	.into_iter()
	.filter_map(Result::ok)
	.filter(|f| f.path().extension().unwrap_or_default().eq("acf")))
}
