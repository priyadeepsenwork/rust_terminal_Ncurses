//* we can also use*/
use ncurses::*;
use std::cmp::*;

//? CONSTANTS
const REGULAR_PAIR: i16 = 1;
const HIGHLIGHT_PAIR: i16 = 2;

fn main() {
    //* start(initialize) ncurses, no-echo and no-cursor
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    //? Colors
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
    
    // program
    let mut quit = false;
    let todos = vec!["Write a Todo App", "Buy a bread", "Get a Job"];

    let mut todo_curr: usize = 0;

    while !quit {
        for (index, todo) in todos.iter().enumerate() {
            let pair = {
                if todo_curr == index {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };

            attron(COLOR_PAIR(pair));
            mv(index as i32, 1);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }

        //refresh here (as per me)
        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' => {
                if todo_curr > 0 {
                    todo_curr -= 1;
                }
            },
            's' => {
                todo_curr = min(todo_curr + 1, todos.len()  - 1);
            },
            _ => {}
        }
    }

 

    //* end ncurses window
    endwin();
}
