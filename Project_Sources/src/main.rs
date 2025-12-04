use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::SystemTime;
use pancurses::{initscr, endwin, A_BOLD, A_COLOR, A_ALTCHARSET, COLOR_GREEN, start_color, init_color, COLOR_MAGENTA, init_pair, COLOR_RED, COLOR_BLACK, ColorPair};

fn main() {

    let window = initscr();
    //window.printw("Hello Rust");
    window.nodelay(true);
    window.keypad(true);
    start_color();
    init_pair(1,COLOR_GREEN, COLOR_BLACK);

    let arc_string = Arc::new(Mutex::new(String::from("Hello, World!")));
    let arc_string_2 = arc_string.clone();


    thread::spawn(move || {

        let mut date_time;
        loop{
            sleep(std::time::Duration::from_millis(1000));
            date_time = chrono::Local::now();

            {
                let mut a = arc_string.lock().unwrap();
                *a = format!("{}", date_time.to_rfc2822());
            }

        }

    });

    let mut str;

    loop{

        window.clear();
        let x_center = window.get_max_x() / 2;

        str = arc_string_2.lock().unwrap().clone();
        window.attron(ColorPair(1));
        window.mvaddstr(0, x_center - str.len() as i32/2, str );
        window.attroff(ColorPair(1));

        window.refresh();
        let ch = window.getch();

        if let Some(T) = ch{
            println!("Key pressed{:?}",T);
            break;
        }

        sleep(std::time::Duration::from_millis(1000));
    }

    endwin();
}
