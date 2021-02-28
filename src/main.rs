extern crate ncurses;

use ncurses::*;
use std::{thread, time};

fn main() {
    initscr();
    keypad(stdscr(), true);
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut pressed_char = 0;
    let mut x = 0;
    let mut y = 0;
    let time_to_sleep = time::Duration::from_millis(1000);

    loop {
        clear();

        mvaddstr(x, y, "0");
        // thread::sleep(time_to_sleep);
        // x+=1;
        // y+=1;
    }



    addstr("\nFinished");
    getch();
    endwin();
}
