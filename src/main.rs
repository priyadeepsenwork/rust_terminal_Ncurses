//* we can also use*/
use ncurses::*;
use std::{ cmp::*, ops::Index };

//? CONSTANTS
const REGULAR_PAIR: i16 = 1;
const HIGHLIGHT_PAIR: i16 = 2;

type Id = usize;

//?creating structure
#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize){
        self.row = row;
        self.col = col;
    }
    fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Nested lists are not allowed!");
        self.list_curr = Some(id);
    }

    fn list_element(&mut self, label: &str, id: Id) {
        // let pair = {
        //     if todo_curr == index {
        //         HIGHLIGHT_PAIR
        //     } else {
        //         REGULAR_PAIR
        //     }
        // };

        // //highlighting the todo with alt-colors
        // attron(COLOR_PAIR(pair));
        // mv(index as i32, 1);e(&mu
        // addstr(*todo);
        // attroff(COLOR_PAIR(pair));
        todo!();
    }

    fn label(&mut self, text: &str, pair: i16) {
        todo!();
    }

    fn end(&mut self){
        todo!();
    }

    fn end_list(&mut self) {
        todo!();
    }
}

fn main() {
    //* start(initialize) ncurses, no-echo and no-cursor
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    //* */ Colors
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    //? driver program
    let mut quit = false;
    let todos: Vec<String> = vec![
        "Learn Linux".to_string(), 
        "Buy a bread".to_string(), 
        "Get a Job".to_string(),
        ];
    let mut todo_curr: usize = 0;

    //*  todos that are completed or 'done'
    let dones = Vec::<String>::new(); //definition for 'done' todos
    let done_curr: usize = 0;

    //set default ui
    let mut ui = Ui::default();

    //? Todo While Loop =======================*/

    while !quit {

        //? UI element --- / 
        ui.begin();
        {
            //* iteration for 'CURRENT' todos */
            ui.begin_list(todo_curr); //?begin from 0
            for (index, todo) in todos.iter().enumerate() {
                ui.list_element(todo,index);
            }
            ui.end_list();

            //separator
            ui.label("---------------------------");

            //* Todos iteration for 'COMPLETED' todos */
            ui.begin_list(done_curr);
            for (index, done) in dones.iter().enumerate() {
                ui.list_element(done,index);
            }
            ui.end_list();

            
        }
        ui.end(); //? End UI Element

        //*refresh here (as per me)
        refresh();

        //navigation + exit controls
        let key = getch();
        match key as u8 as char {
            'q' => {
                quit = true;
            }
            'w' => {
                if todo_curr > 0 {
                    todo_curr -= 1;
                }
            }
            's' => {
                todo_curr = min(todo_curr + 1, todos.len() - 1);
            }
            _ => {}
        }
    }

    //* end ncurses window
    endwin();
}
