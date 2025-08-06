extern crate ncurses;

//* we can also use*/
// use ncurses::*

//better use this with autocomplete IDE
use ncurses::{addstr, endwin, getch, initscr, refresh};

fn main() {
    //* start ncurses
    initscr();
    
    //add string
    addstr("Hello, World!").unwrap();
    
    //refresh screen 
    refresh();

    let mut quit = false;

    while !quit {
        let key = getch();

        match key as u8 as char {
            'q' => quit = true,
            _ => {}
        }
    }
    
    //wait for a keypress
    //getch(); <--- Not required, exit when types 'q' instantly

    //* end ncurses window
    endwin();
}
