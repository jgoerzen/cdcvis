/*

Copyright (c) 2019 John Goerzen

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::env;
use std::ffi::OsString;
use std::error::Error;

mod parser;
mod analysis;
mod sankeygen;

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
fn main() {
    let file_path = get_first_arg().expect("need args").into_string().expect("conversion issue");
    let mut rdr = parser::parse_init_file(file_path).expect("Couldn't init parser");
    let vr = parser::parse(&mut rdr);
    let mut t = analysis::mktree(vr);
    analysis::treecalcs(&mut t);
    analysis::coalescepct(&mut t, 0.03, 0.5);
    /* for debugging
    let mut level = 0;
    for item in t.traverse() {
        match item {
            rctree::NodeEdge::Start(x) => {
                level += 1;
                println!("{}{:?}", " ".repeat(level * 5), x.borrow()); }
            _ => { level -= 1; },
        }
    }
    println!("Titles OK: {}", analysis::aretitlesok(&t));
    */
    for item in sankeygen::sankeymatic(&t) {
        println!("{}", item);
    }
}

