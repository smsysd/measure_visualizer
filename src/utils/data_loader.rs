use std::{fs::{File, OpenOptions}, path::PathBuf, io::BufReader};
use bevy::prelude::Resource;
use rmp_serde as rmps;
use serde::{Deserialize, Serialize};
use calamine::{open_workbook, Xlsx, Reader};

use super::{in_delta_i64, in_delta_f64};

const XLSX_SHEET_BG: &str = "bg";
const XLSX_SHEET_PHOTO: &str = "photo";
const XLSX_SHEET_TEMP: &str = "temp";
const XLSX_SHEET_FLOW: &str = "flow";
const XLSX_LATITUDE_INDEX: usize = 0;
const XLSX_LONGITUDE_INDEX: usize = 1;
const XLSX_DEEP_INDEX: usize = 2;
const XLSX_DATETIME_INDEX: usize = 3;
const XLSX_SPEC_INDEX: usize = 5;

enum Extension {
	Msgpack,
	Xlsx
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Point {
	pub latitude: f64,
	pub longitude: f64,
	pub deep: f64
}

impl Default for Point {
	fn default() -> Self {
		Self {
			deep: 0.,
			latitude: 0.,
			longitude: 0.
		}
	}
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Temp {
	pub point: Point,
	pub timestamp: i64,
	pub val: f64
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Photo {
	pub point: Point,
	pub timestamp: i64,
	pub solar: f64,
	pub transparency: Vec<(f64, f64)>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Flow {
	pub point: Point,
	pub timestamp: i64,
	pub speed: f64,
	pub dir: f64
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BackgroundImage {
	pub image_path: String,
	pub scale: f64,
	pub rotate: f64
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Background {
	pub image: Option<BackgroundImage>,
	pub border: Vec<Point>
}

impl Default for Background {
	fn default() -> Self {
		Self {
			border: Vec::new(),
			image: None
		}
	}
}

pub struct Query {
	pub timestamp: i64,
	pub timestamp_d: i64,
	pub deep: f64,
	pub temp_deep_d: f64,
	pub photo_deep_d: f64,
	pub flow_deep_d: f64
}

pub struct QueryResult {
	pub photo: Vec<Photo>,
	pub temp: Vec<Temp>,
	pub flow: Vec<Flow>	
}

pub struct Ranges {
	pub deep_max: f64,
	pub deep_min: f64,
	pub timestamp_max: i64,
	pub timestamp_min: i64
}

impl Default for Ranges {
	fn default() -> Self {
		Self {
			deep_max: 0.0,
			deep_min: 0.0,
			timestamp_max: 0,
			timestamp_min: 0
		}
	}
}

#[derive(Deserialize, Serialize, Resource)]
pub struct Data {
	pub bg: Background,
	pub photo: Vec<Photo>,
	pub temp: Vec<Temp>,
	pub flow: Vec<Flow>
}

impl Default for Data {
	fn default() -> Self {
		Self {
			bg: Background::default(),
			photo: Vec::new(),
			temp: Vec::new(),
			flow: Vec::new()
		}
	}
}

impl Data {
	pub fn clear(&mut self) {
		self.bg = Background::default();
		self.photo.clear();
		self.temp.clear();
		self.flow.clear();
	}

	pub fn add(&mut self, data_add: Data) -> (usize, usize, usize) {
		let old_photos = self.photo.len();
		let old_temps = self.temp.len();
		let old_flows = self.flow.len();
		for p in data_add.photo {
			self.photo.push(p);
		}
		for p in data_add.temp {
			self.temp.push(p);
		}
		for p in data_add.flow {
			self.flow.push(p);
		}
		let pthoto_add = self.photo.len() - old_photos;
		let temp_add = self.temp.len() - old_temps;
		let flow_add = self.flow.len() - old_flows;
		(pthoto_add, temp_add, flow_add)
	}

	pub fn query_2d(&self, q: &Query) -> QueryResult {
		QueryResult {
			photo: {
				let mut data = Vec::new();
				for p in &self.photo {
					if in_delta_i64(p.timestamp, q.timestamp, q.timestamp_d) {
						if in_delta_f64(p.point.deep, q.deep, q.photo_deep_d) {
							data.push(p.clone());
						}
					}
				}
				data
			},
			temp: {
				let mut data = Vec::new();
				for p in &self.temp {
					if in_delta_i64(p.timestamp, q.timestamp, q.timestamp_d) {
						if in_delta_f64(p.point.deep, q.deep, q.temp_deep_d) {
							data.push(p.clone());
						}
					}
				}
				data
			},
			flow: {
				let mut data = Vec::new();
				for p in &self.flow {
					if in_delta_i64(p.timestamp, q.timestamp, q.timestamp_d) {
						if in_delta_f64(p.point.deep, q.deep, q.flow_deep_d) {
							data.push(p.clone());
						}
					}
				}
				data
			}
		}
	}

	pub fn ranges(&self) -> Ranges {
		let mut dmax = f64::MIN;
		let mut dmin = f64::MAX;
		let mut tsmax = i64::MIN;
		let mut tsmin = i64::MAX;
		let mut is_deep_exists = false;
		let mut is_timestamp_exists = false;
		for p in &self.bg.border {
			is_deep_exists = true;
			if p.deep > dmax {dmax = p.deep}
			if p.deep < dmin {dmin = p.deep}
		}
		for p in &self.photo {
			is_deep_exists = true;
			is_timestamp_exists = true;
			if p.point.deep > dmax {dmax = p.point.deep}
			if p.point.deep < dmin {dmin = p.point.deep}
			if p.timestamp > tsmax {tsmax = p.timestamp}
			if p.timestamp < tsmin {tsmin = p.timestamp}
		}
		for p in &self.temp {
			is_deep_exists = true;
			is_timestamp_exists = true;
			if p.point.deep > dmax {dmax = p.point.deep}
			if p.point.deep < dmin {dmin = p.point.deep}
			if p.timestamp > tsmax {tsmax = p.timestamp}
			if p.timestamp < tsmin {tsmin = p.timestamp}
		}
		for p in &self.flow {
			is_deep_exists = true;
			is_timestamp_exists = true;
			if p.point.deep > dmax {dmax = p.point.deep}
			if p.point.deep < dmin {dmin = p.point.deep}
			if p.timestamp > tsmax {tsmax = p.timestamp}
			if p.timestamp < tsmin {tsmin = p.timestamp}
		}
		if !is_deep_exists {
			dmax = 0.0;
			dmin = 0.0;
		}
		if !is_timestamp_exists {
			tsmax = 0;
			tsmin = 0;
		}
		Ranges {
			deep_max: dmax,
			deep_min: dmin,
			timestamp_max: tsmax,
			timestamp_min: tsmin
		}
	}
}


pub fn load_data(path: &PathBuf) -> Result<Data, String> {
	println!("Load data from {:?}", path);
	let extension = match path.extension() {
		Some(name) => {
			let name = name.to_str().unwrap();
			if name == "xlsx" {
				Extension::Xlsx
			} else
			if name == "dat" {
				Extension::Msgpack
			} else {
				return Err(format!("Extension '{}' of selected file not supported, supported list: [dat, xlsx]", name));
			}
		},
		None => Extension::Msgpack
	};
	match extension {
		Extension::Msgpack => {
			let file = match File::open(path) {
				Ok(f) => f,
				Err(e) => return Err(format!("Fail to open dat file: {}", e.to_string()))
			};
			match rmps::decode::from_read(file) {
				Ok(data) => Ok(data),
				Err(e) => Err(format!("Decode error: {}", e.to_string()))
			}
		},
		Extension::Xlsx => {
			let mut excel: Xlsx<_> = match open_workbook(path) {
				Ok(data) => data,
				Err(e) => return Err(format!("Fail to open xlsx file: {}", e.to_string()))
			};
			let bgs = xlsx_open_sheet(&mut excel, XLSX_SHEET_BG)?;
			let photos = xlsx_open_sheet(&mut excel, XLSX_SHEET_PHOTO)?;
			let temps = xlsx_open_sheet(&mut excel, XLSX_SHEET_TEMP)?;
			let flows = xlsx_open_sheet(&mut excel, XLSX_SHEET_FLOW)?;

			let bg = match xlsx_load_bg(&bgs) {
				Ok(data) => data,
				Err(e) => return Err(format!("Fail to load bg: {}", e))
			};
			let photo = match xlsx_load_photo(&photos) {
				Ok(data) => data,
				Err(e) => return Err(format!("Fail to load photo: {}", e))
			};
			let temp = match xlsx_load_temp(&temps) {
				Ok(data) => data,
				Err(e) => return Err(format!("Fail to load temp: {}", e))
			};
			let flow = match xlsx_load_flow(&flows) {
				Ok(data) => data,
				Err(e) => return Err(format!("Fail to load flow: {}", e))
			};

			Ok(Data {
				bg: bg,
				photo: photo,
				temp: temp,
				flow: flow
			})
		}
	}
}

fn xlsx_load_bg(bgs: &calamine::Range<calamine::DataType>) -> Result<Background, String> {
	Ok(Background {
		image: xlsx_get_image(&bgs),
		border: {
			let mut data = Vec::new();
			for i in 5..bgs.rows().len() {
				data.push(xlsx_get_point(&bgs, i)?);
			}
			data
		}
	})
}

fn xlsx_load_photo(photos: &calamine::Range<calamine::DataType>) -> Result<Vec<Photo>, String> {
	let mut photo = Vec::new();
	for i in 1..photos.rows().len() {
		photo.push(Photo {
			point: xlsx_get_point(&photos, i)?,
			timestamp: xlsx_get_timestamp(&photos, i, XLSX_DATETIME_INDEX)?,
			solar: xlsx_get_f64(&photos, i, XLSX_SPEC_INDEX)?,
			transparency: {
				let mut data = Vec::new();
				let mut pos = XLSX_SPEC_INDEX + 1;
				let ncells = xlsx_row_len(photos, i);
				println!("parse photo row {}, ncells: {}", i, ncells);
				loop {
					let wl = xlsx_get_f64(&photos, i, pos)?;
					let val = xlsx_get_f64(&photos, i, pos+1)?;
					println!("photoval at row {}: ({}, {})", i, wl, val);
					data.push((wl, val));
					pos += 2;
					if pos >= ncells {
						break;
					}
				}
				data
			}
		});
	}
	Ok(photo)
}

fn xlsx_load_temp(temps: &calamine::Range<calamine::DataType>) -> Result<Vec<Temp>, String> {
	let mut temp = Vec::new();
	for i in 1..temps.rows().len() {
		temp.push(Temp {
			point: xlsx_get_point(&temps, i)?,
			timestamp: xlsx_get_timestamp(&temps, i, XLSX_DATETIME_INDEX)?,
			val: xlsx_get_f64(&temps, i, XLSX_SPEC_INDEX)?
		});
	}
	Ok(temp)
}

fn xlsx_load_flow(flows: &calamine::Range<calamine::DataType>) -> Result<Vec<Flow>, String> {
	let mut flow = Vec::new();
	for i in 1..flows.rows().len() {
		flow.push(Flow {
			point: xlsx_get_point(&flows, i)?,
			timestamp: xlsx_get_timestamp(&flows, i, XLSX_DATETIME_INDEX)?,
			speed: xlsx_get_f64(&flows, i, XLSX_SPEC_INDEX)?,
			dir: xlsx_get_f64(&flows, i, XLSX_SPEC_INDEX+1)?
		});
	}
	Ok(flow)
}

fn xlsx_get_point(sheet: &calamine::Range<calamine::DataType>, r: usize) -> Result<Point, String> {
	Ok(Point {
		latitude: xlsx_get_f64(sheet, r, XLSX_LATITUDE_INDEX)?,
		longitude: xlsx_get_f64(sheet, r, XLSX_LONGITUDE_INDEX)?,
		deep: xlsx_get_f64(sheet, r, XLSX_DEEP_INDEX)?,
	})
}

fn xlsx_get_image(sheet: &calamine::Range<calamine::DataType>) -> Option<BackgroundImage>{
	match xlsx_get_str(sheet, 1, 0) {
		Ok(path) => match xlsx_get_f64(sheet, 1, 1) {
			Ok(scale) => match xlsx_get_f64(sheet, 1, 2) {
				Ok(rotate) => Some(BackgroundImage {
					image_path: path,
					scale: scale,
					rotate: rotate
				}),
				_ => None
			},
			_ => None
		},
		_ => None
	}
}

fn xlsx_get_f64(sheet: &calamine::Range<calamine::DataType>, r: usize, c: usize) -> Result<f64, String> {
	match sheet.get((r, c)) {
		Some(val) => match val.get_float() {
			Some(val) => Ok(val),
			None => Err(format!("Fail to get value from ({}, {}) - expected float, found '{:?}'", r, c, val))
		},
		None => Err(format!("Fail to get value from ({}, {}) - none", r, c))
	}
}

fn xlsx_get_str(sheet: &calamine::Range<calamine::DataType>, r: usize, c: usize) -> Result<String, String> {
	match sheet.get((r, c)) {
		Some(val) => match val.get_string() {
			Some(val) => Ok(String::from(val)),
			None => Err(format!("Fail to get value from ({}, {}) - expected string, found '{:?}'", r, c, val))
		},
		None => Err(format!("Fail to get value from ({}, {}) - none", r, c))
	}
}

fn xlsx_get_timestamp(sheet: &calamine::Range<calamine::DataType>, r: usize, c: usize) -> Result<i64, String> {
	match sheet.get((r, c)) {
		Some(val) => match val {
			calamine::DataType::DateTime(exldt) => Ok(((exldt - 25569.0) * 86400.0) as i64),
			oth => Err(format!("Fail to get value from ({}, {}) - expected string, found '{:?}'", r, c, oth))
		},
		None => Err(format!("Fail to get value from ({}, {}) - none", r, c))
	}
}

fn xlsx_open_sheet(excel: &mut Xlsx<BufReader<File>>, name: &str) -> Result<calamine::Range<calamine::DataType>, String> {
	match excel.worksheet_range(name) {
		Some(res) => match res {
			Ok(data) => Ok(data),
			Err(e) => Err(format!("Fail to read '{}': {}", name, e.to_string()))
		},
		None => Err(format!("Sheet '{}' not found in book", name))
	}
}

fn xlsx_row_len(sheet: &calamine::Range<calamine::DataType>, r: usize) -> usize {
	let ncells = sheet.cells().len();
	let mut cnt = 0;
	for i in 0..ncells {
		let val = match sheet.get((r, i)) {
			Some(val) => val,
			None => return 0
		};
		match val {
			calamine::DataType::Empty => break,
			_ => cnt += 1
		}
	}
	cnt
}

pub fn save_data(path: &PathBuf, data: &Data) -> Result<(), String> {
	let mut open_opt = OpenOptions::new();
	open_opt.append(false);
	open_opt.write(true);
	open_opt.truncate(true);
	open_opt.create(true);
	open_opt.create_new(false);
	let mut file = match open_opt.open(path) {
		Ok(f) => f,
		Err(e) => return Err(format!("Fail open file to write: {}", e.to_string()))
	};
	rmps::encode::write(&mut file, data).unwrap();
	println!("Data saved at {:?}", path);
	Ok(())
}