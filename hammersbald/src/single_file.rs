use crate::error::Error;
use crate::page::{Page, PAGE_SIZE};
use crate::paged_file::PagedFile;
use crate::pref::PRef;

use std::cmp::max;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Mutex;

pub struct SingleFile {
	file: Mutex<File>,
	base: u64,
	len: u64,
	file_size: u64,
}

impl SingleFile {
	pub fn new(mut file: File, base: u64, file_size: u64) -> Result<SingleFile, Error> {
		let len = file.seek(SeekFrom::End(0))?;
		Ok(SingleFile {
			file: Mutex::new(file),
			base,
			len,
			file_size,
		})
	}
}

impl PagedFile for SingleFile {
	fn read_page(&self, pref: PRef) -> Result<Option<Page>, Error> {
		let pos = pref.as_u64();
		if pos < self.base || pos >= self.base + self.file_size {
			return Err(Error::Corrupted("read from wrong file".to_string()));
		}
		let pos = pos - self.base;
		if pos < self.len {
			let mut file = self.file.lock().unwrap();
			file.seek(SeekFrom::Start(pos))?;
			let mut buffer = [0u8; PAGE_SIZE];

			let len = if self.len as usize % PAGE_SIZE > 0 {
				self.len as usize % PAGE_SIZE
			} else {
				PAGE_SIZE as usize
			};
			file.read_exact(&mut buffer[..len])?;
			return Ok(Some(Page::from_buf(buffer)));
		}
		Ok(None)
	}

	// The length of the current single file (a multiple of PAGE_SIZE)
	fn len(&self) -> Result<u64, Error> {
		Ok(self.len)
	}

	fn truncate(&mut self, new_len: u64) -> Result<(), Error> {
		self.len = new_len;
		Ok(self.file.lock().unwrap().set_len(new_len)?)
	}

	fn sync(&self) -> Result<(), Error> {
		Ok(self.file.lock().unwrap().sync_data()?)
	}

	fn shutdown(&mut self) -> Result<(), Error> {
		Ok(())
	}

	fn update_page(&mut self, page: Page) -> Result<u64, Error> {
		let page_pos = page.pref().as_u64();
		if page_pos < self.base || page_pos >= self.base + self.file_size {
			return Err(Error::Corrupted("write to wrong file".to_string()));
		}
		let pos = page_pos - self.base;

		let mut file = self.file.lock().unwrap();
		file.seek(SeekFrom::Start(pos))?;
		file.write_all(&page.into_buf())?;
		self.len = max(self.len, pos + PAGE_SIZE as u64);
		Ok(self.len)
	}

	fn flush(&mut self) -> Result<(), Error> {
		Ok(self.file.lock().unwrap().flush()?)
	}
}

#[cfg(test)]
mod tests {
	use crate::page::{Page, PAGE_SIZE};
	use crate::paged_file::PagedFile;
	use crate::pref::PRef;
	use crate::single_file::SingleFile;
	use std::fs;
	use std::fs::OpenOptions;

	#[test]
	fn test_single_file() {
		let file_name = "testdb/single-test.bc";
		fs::remove_file(file_name).unwrap_or_default();

		let mut options = OpenOptions::new();
		options.read(true).write(true).create(true);
		let file = options.open(file_name).unwrap();
		let mut single_file = SingleFile::new(file, 0, 100000).unwrap();

		let page_one_pref = PRef::from(0);
		let mut page_one = Page::new_page_with_position(page_one_pref);
		page_one.write_u64(0, 1);
		single_file.update_page(page_one.clone()).unwrap();

		let page_two_pref = page_one_pref.next_page();
		let mut page_two = Page::new_page_with_position(page_two_pref);
		page_two.write_u64(0, 2);
		single_file.update_page(page_two.clone()).unwrap();

		single_file.sync().unwrap();
		single_file.flush().unwrap();

		assert_eq!(PAGE_SIZE as u64 * 2, single_file.len);

		page_one.write_u64(0, 3);
		single_file.update_page(page_one.clone()).unwrap();

		assert_eq!(PAGE_SIZE as u64 * 2, single_file.len);

		let page_result = single_file.read_page(page_one_pref).unwrap().unwrap();
		assert_eq!(3, page_result.read_u64(0))
	}
}