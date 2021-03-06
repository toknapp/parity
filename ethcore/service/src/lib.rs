// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

extern crate ansi_term;
extern crate ethcore;
extern crate ethcore_io as io;
extern crate ethcore_private_tx;
extern crate ethcore_sync as sync;
extern crate kvdb;
extern crate stop_guard;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate tempdir;

mod error;
mod service;

#[cfg(test)]
extern crate kvdb_rocksdb;

pub use error::{Error, ErrorKind};
pub use service::{ClientService, PrivateTxService};
