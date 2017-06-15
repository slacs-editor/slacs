//! This is the utilities module for Xt.

// This file is part of Xt.

// This is the Xt text editor; it edits text.
// Copyright (C) 2016-2017  The Xt Developers

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

/// Custom types module.
pub mod types;

/// UUID module.
pub mod uuid;

/// Return version of Xt.
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Return name of Xt. (Duh!)
pub fn get_pkg_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
