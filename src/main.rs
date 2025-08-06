extern crate ncurses;

//* we can also use*/
use ncurses::*;

fn main() {
    //* start(initialize) ncurses
    initscr();

    let mut quit = false;
    let todos = vec!["Write a Todo App", "Buy a bread", "Get a Job", "Fix bugs"];

    //* EVENT LOOP */
    while !quit {
        for (row, todo) in todos.iter().enumerate() {
            mv(row as i32, 0);
            addstr(*todo);
        }

        //refresh here (as per me)
        refresh();

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
