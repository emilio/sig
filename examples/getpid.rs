// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sig
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate sig;

fn main() {
    let from: i32 = getpid!();

    println!("{}", from);
}
