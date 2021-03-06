#![crate_id(name="printenv", vers="1.0.0", author="Seldaek")]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Jordi Boggiano <j.boggiano@seld.be>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/* last synced with: printenv (GNU coreutils) 8.13 */

#![feature(macro_rules)]

extern crate getopts;
extern crate libc;

use std::os;
use std::io::print;

#[path = "../common/util.rs"]
mod util;

static NAME: &'static str = "printenv";

#[allow(dead_code)]
fn main() { os::set_exit_status(uumain(os::args())); }

pub fn uumain(args: Vec<String>) -> int {
    let program = args.get(0).clone();
    let opts = [
        getopts::optflag("0", "null", "end each output line with 0 byte rather than newline"),
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version", "output version information and exit"),
    ];
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => {
            crash!(1, "Invalid options\n{}", f.to_err_msg())
        }
    };
    if matches.opt_present("help") {
        println!("printenv 1.0.0");
        println!("");
        println!("Usage:");
        println!("  {0:s} [VARIABLE]... [OPTION]...", program);
        println!("");
        print(getopts::usage("Prints the given environment VARIABLE(s), otherwise prints them all.", opts).as_slice());
        return 0;
    }
    if matches.opt_present("version") {
        println!("printenv 1.0.0");
        return 0;
    }
    let mut separator = "\n";
    if matches.opt_present("null") {
        separator = "\x00";
    };

    exec(matches.free, separator);

    0
}

pub fn exec(args: Vec<String>, separator: &str) {
    if args.is_empty() {
        let vars = os::env();
        for (env_var, value) in vars.move_iter() {
            print!("{0:s}={1:s}", env_var, value);
            print(separator);
        }
        return;
    }

    for env_var in args.iter() {
        match os::getenv(env_var.as_slice()) {
            Some(var) => {
                print(var.as_slice());
                print(separator);
            }
            _ => ()
        }
    }
}
