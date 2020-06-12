extern crate serial;

use std::env;
use std::io;
use std::thread;
use std::time::{Duration, SystemTime};

use debouncr::{debounce_5, Debouncer, Edge};
use serial::prelude::*;
use xdotool::command::options;
use xdotool::mouse::{click_down, click_up, Button};
use xdotool::OptionVec;
fn main() {
	for arg in env::args_os().skip(1) {
		let mut port = serial::open(&arg).unwrap();
		interact(&mut port).unwrap();
	}
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
	port.reconfigure(&|settings| {
		settings.set_baud_rate(serial::Baud115200)?;
		settings.set_char_size(serial::Bits8);
		settings.set_parity(serial::ParityNone);
		settings.set_stop_bits(serial::Stop1);
		settings.set_flow_control(serial::FlowNone);
		Ok(())
	})?;

	port.set_timeout(Duration::from_millis(1000))?;
	let mut click_combo = 0;
	let mut debouncer = debounce_5();
	loop {
		let mut last = 0u8;
		let mut buf = vec![];
		loop {
			last = read_byte(port)?;
			if last == 13 || last == 10 {
				break;
			} else {
				buf.push(last);
			}
		}
		let nstr = String::from_utf8(buf).unwrap();
		if nstr.len() == 0 {
			continue;
		}
		let num: u8 = nstr.parse().unwrap();
		// print!("{}", num);
		let maybedge = debouncer.update(num < 10);
		match maybedge {
			Some(edge) => {
				println!("{:?}", edge);
				match edge {
					Edge::Rising => {
						click_down(Button::Left, OptionVec::<options::ClickOption>::new());
					}
					Edge::Falling => {
						click_up(Button::Left, OptionVec::<options::ClickOption>::new());
					}
				}
			}
			None => {}
		}
	}

	Ok(())
}

fn read_byte(port: &mut SerialPort) -> io::Result<u8> {
	let mut buf = vec![0; 1];
	port.read(&mut buf)?;

	Ok(*buf.get(0).unwrap())
}
/*				thread::spawn(move || {
					click(Button::Left, OptionVec::<options::ClickOption>::new());
				});
*/
