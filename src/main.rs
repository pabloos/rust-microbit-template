#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::Timer,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    loop { }
}
