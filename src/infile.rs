//
// Copyright 2018 Tamas Blummer
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//!
//! # in file store
//!
//! Implements persistent store

use error::BCSError;
use bcdb::PageFile;
use logfile::LogFile;
use keyfile::KeyFile;
use datafile::DataFile;
use bcdb::{BCDBFactory, BCDB};
use types::Offset;
use page::Page;
use rolled::RolledFile;

use std::sync::{Mutex,Arc};

/// Implements persistent storage
pub struct InFile {
    file: RolledFile
}

impl InFile {
    /// create a new DB in memory for tests
    pub fn new (file: RolledFile) -> InFile {
        InFile {file: file}
    }
}

impl BCDBFactory for InFile {
    fn new_db (name: &str) -> Result<BCDB, BCSError> {
        let log = Arc::new(Mutex::new(LogFile::new(Box::new(
            RolledFile::new(name.to_string(), "lg".to_string(), true)?))));
        let table = KeyFile::new(Box::new(InFile::new(
            RolledFile::new(name.to_string(), "tb".to_string(), false)?
        )), log);
        let data = DataFile::new(Box::new(RolledFile::new(name.to_string(), "bc".to_string(), true)?))?;

        BCDB::new(table, data)
    }
}

impl PageFile for InFile {
    fn flush(&mut self) -> Result<(), BCSError> {
        self.file.flush()
    }

    fn len(&self) -> Result<u64, BCSError> {
        self.file.len()
    }

    fn truncate(&mut self, new_len: u64) -> Result<(), BCSError> {
        self.file.truncate(new_len)
    }

    fn sync(&self) -> Result<(), BCSError> {
        self.file.sync()
    }

    fn read_page(&self, offset: Offset) -> Result<Page, BCSError> {
        self.file.read_page(offset)
    }

    fn append_page(&mut self, page: Page) -> Result<(), BCSError> {
        self.file.append_page(page)
    }

    fn write_page(&mut self, page: Page) -> Result<(), BCSError> {
        self.file.write_page(page)
    }
}