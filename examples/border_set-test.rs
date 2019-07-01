extern crate ncursesw;
extern crate ncurseswin;

use ncursesw::{curs_set, NCurseswError, LcCategory, CursorType, Size, Origin};
use ncursesw::extend::*;
use ncurseswin::*;

macro_rules! result { ($t: ty) => { Result<$t, NCurseswError> } }

fn main() {
    if let Err(e) = main_routine() {
        println!("error: {}", e);
    }
}

fn main_routine() -> result!(()) {
    setlocale(LcCategory::All, "");

    ncursesw_init(|ncurses| {
        let doit = |initial_window: Window| -> result!(()) {
            curs_set(CursorType::Invisible)?;
            set_echo(false)?;

            start_color()?;
            use_default_colors()?;

            let color_pair0 = ColorPair::default();
            let attrs = Attributes::default();

            let initial_size = initial_window.get_size()?;

            let inner_size = Size { lines: initial_size.lines - 2, columns: initial_size.columns - 2 };
            let inner_origin = Origin { y: 1, x: 1 };

            let inner_window = initial_window.subwin(inner_size, inner_origin)?;

            let mut origin = Origin { y: 1, x: 1 };

            inner_window.mvaddstr(origin, "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.")?;
            origin.y += 1;

            inner_window.mvaddstr(origin, "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.")?;
            origin.y += 2;

            let origin = Origin { y: origin.y, x: 1 };

            let box_drawing_types: [BoxDrawingType; 14] = [BoxDrawingType::Ascii,
                                                           BoxDrawingType::Light(BoxDrawingTypeDetail::Normal),
                                                           BoxDrawingType::Light(BoxDrawingTypeDetail::LeftDash),
                                                           BoxDrawingType::Light(BoxDrawingTypeDetail::RightDash),
                                                           BoxDrawingType::Light(BoxDrawingTypeDetail::DoubleDash),
                                                           BoxDrawingType::Light(BoxDrawingTypeDetail::TripleDash),
                                                           BoxDrawingType::Light(BoxDrawingTypeDetail::QuadrupleDash),
                                                           BoxDrawingType::Heavy(BoxDrawingTypeDetail::Normal),
                                                           BoxDrawingType::Heavy(BoxDrawingTypeDetail::LeftDash),
                                                           BoxDrawingType::Heavy(BoxDrawingTypeDetail::RightDash),
                                                           BoxDrawingType::Heavy(BoxDrawingTypeDetail::DoubleDash),
                                                           BoxDrawingType::Heavy(BoxDrawingTypeDetail::TripleDash),
                                                           BoxDrawingType::Heavy(BoxDrawingTypeDetail::QuadrupleDash),
                                                           BoxDrawingType::Double];

            for &box_drawing_type in &box_drawing_types {
                let ul = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperLeftCorner, &attrs, &color_pair0)?;
                let ll = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerLeftCorner, &attrs, &color_pair0)?;
                let ur = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::UpperRightCorner, &attrs, &color_pair0)?;
                let lr = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::LowerRightCorner, &attrs, &color_pair0)?;
                let hl = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::HorizontalLine, &attrs, &color_pair0)?;
                let vl = complex_box_graphic(box_drawing_type, BoxDrawingGraphic::VerticalLine, &attrs, &color_pair0)?;

                initial_window.border_set(vl, vl, hl, hl, ul, ur, ll, lr)?;

                inner_window.set_cursor(origin)?;
                inner_window.clrtoeol()?;

                inner_window.border_set(vl, vl, hl, hl, ul, ur, ll, lr)?;

                inner_window.mvaddstr(origin, &format!("box drawing type {:?}", box_drawing_type))?;

                // by refreshing the initial_window we also refresh our inner_window which is a sub window of inital_window
                initial_window.refresh()?;

                inner_window.getch()?;
            }

            Ok(())
        };

        panic!(match doit(ncurses.initial_window()) {
            Err(e) => e.to_string(),
            _      => "this is the end my friend, the only end!!!".to_string()
        })
    }).unwrap_or_else(|e| match e {
        Some(errmsg) => println!("A Panic Occurred: {}", errmsg),
        None         => println!("There was an error, but no error message."),
    });

    Ok(())
}
