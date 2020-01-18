# Changelog

All breaking changes are marked with [BC] and potentially require API consumer changes after updating to the respective version.

## [0.5.0] - ????-??-?? [BC]
- NCurses form module implemented as `ncurseswwin::form`.
- `menu::menu_request_name()` now takes a `MenuRequest` instead of a `i32` for the request. [BC]
- `menu::Menu::set_menu_pad()` now takes a `char` instead of a `i32` for the pad character. [BC]
- `menu::Menu::{set_item_init, set_item_term, set_menu_init, set_menu_term}` now use `Fn(&Menu)` traits. [BC]
- `menu::Menu::{item_init, item_term, menu_init, menu_term}` have been depreciated. [BC]
- `menu::PostedMenu::menu_driver()` now returns a `Result<Option<MenuRequest>, NCurseswMenuError>` instead of `Result<Option<i32>, NCurseswMenuError>`. [BC]
- `gen::BaseCanvas::putwin()`, `gen::IsWindow::putwin()` and `gen::IsPad::putwin()` now take `O: std::os::unix::io::AsRawFD + std::io::Write` instead of `&std::path::Path`. [BC]
- `gen::NCurseswWindow::getwin()` and `gen::IsPad::genwin()` now take `I: std::os::unix::io::AsRawFD + std::io::Read` instead of `&std::path::Path`. [BC]

## [0.4.0] - 2019-12-09 [BC]
- Coordinate system's used within the crate (i.e. `Origin`, `Size` etc.) have been changed from using axis basic types of `i32` to `u16`. [BC]
- NCurses menu module implemented as `ncurseswwin::menu`.
- Moved `dupwin()` and `getwin()` from the `IsWindow` trait to `NCursesWindow` trait [BC].

## [0.3.1] - 2019-11-07
- NCurses `mouse` functionality added.
- `Window`, `Pad` and `RipoffWindow` now use traits to provide functionality.
- Added non-blocking functionality to `get` type functions.

## [0.3.0] - 2019-11-02 [BC]
- Initialising NCurses functionality is now done using the `ncursesw_entry()` function instead of the `ncursesw_init()` function to provide better panic trapping that can be caught and processed as a `Result`.
- ...

## [0.2.1] - 2019-10-25
- ...

## [0.2.0] - 2019-10-15 [BC]
- ...

## [0.1.5] - 2019-10-14
- ...

## [0.1.4] - 2019-07-13
- ...

## [0.1.3] - 2019-07-09
- ...

## [0.1.2] - 2019-07-06
- ...

## [0.1.1] - 2019-07-04
- ...

## [0.1.0] - 2019-07-01
- Initial release.
