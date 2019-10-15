extern crate ncurseswwin;

use ncurseswwin::*;
use ncurseswwin::normal::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswError> } }

fn main() {
    if let Err(e) = main_routine() {
        println!("error: {}", e);
    }
}

fn main_routine() -> result!(()) {
    setlocale(LcCategory::All, "");

    ncursesw_init(|ncurses| {
        let doit = |initial_window: &Window| -> result!(()) {
            curs_set(CursorType::Invisible)?;
            set_echo(false)?;

            start_color()?;
            use_default_colors()?;

            let fg_color = Color::Light(BaseColor::Yellow);
            let bg_color = Color::Dark(BaseColor::Blue);

            let color_pair1 = ColorPair::new(1, Colors::new(fg_color, bg_color))?;

            //initial_window.color_set(color_pair1)?;

            let attrs = Attributes::default();

            let window_size = initial_window.size()?;

            let origin = Origin { y: 1, x: 1 };
            let size = Size { lines: 20, columns: 20 };

            let box_drawing_type = BoxDrawingType::Double;

            let ul = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperLeftCorner, &attrs, &color_pair1)?;
            let ll = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerLeftCorner, &attrs, &color_pair1)?;
            let ur = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperRightCorner, &attrs, &color_pair1)?;
            let lr = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerRightCorner, &attrs, &color_pair1)?;
            let hl = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::HorizontalLine, &attrs, &color_pair1)?;
            let vl = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::VerticalLine, &attrs, &color_pair1)?;

            initial_window.border_set(vl, vl, hl, hl, ul, ur, ll, lr)?;

            initial_window.mvwbox_set(origin, size, box_drawing_type)?;
            initial_window.mvwbox_set(Origin { y: 5, x: 5 }, size, box_drawing_type)?;
            initial_window.mvwbox_set(Origin { y: 2, x: 0 }, size, box_drawing_type)?;
            initial_window.mvwbox_set(Origin { y: 10, x: 10 }, size, box_drawing_type)?;
            initial_window.mvwbox_set(Origin { y: 0, x: 10 }, size, box_drawing_type)?;

            initial_window.mvwbox_set(Origin { y: window_size.lines - size.lines, x: window_size.columns - size.columns }, size, box_drawing_type)?;

            initial_window.refresh()?;

            initial_window.getch()?;

            Ok(())
        };

        panic!(match doit(&ncurses.initial_window()) {
            Err(e) => e.to_string(),
            _      => "this is the end my friend, the only end!!!".to_string()
        })
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });

    Ok(())
}
