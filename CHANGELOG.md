# Changelog

All breaking changes are marked with [BC] and potentially require API consumer changes after updating to the respective version.

## [0.4.1] - ????-??-?? [BC]
- NCurses form module implemented as `ncurseswwin::form`.
- `menu_request_name()` now takes a `MenuRequest` instead of a `i32` for the request. [BC]
- `Menu::set_menu_pad()` now takes a `char` instead of a `i32` for the pad character. [BC]
- `Menu::{set_item_init, set_item_term, set_menu_init, set_menu_term}` now use `Fn(&Menu)` traits. [BC]
- `Menu::{item_init, item_term, menu_init, menu_term}` have been depreciated. [BC]

## [0.4.0] - 2019-12-09 [BC]
- Coordinate system used within the crate has been changed from using axis basic types of `i32` to `u16`. [BC]
- NCurses menu module implemented as `ncurseswwin::menu`.
- Moved `dupwin()` and `getwin()` from `IsWindow` to `NCursesWindow` trait [BC].

## [0.3.1] - 2019-11-07
- NCurses `mouse` functionality added.
- `Window`, `Pad` and `RipoffWindow` now use traits to provide functionality.
- Added non-blocking functionality to get type functions.

## [0.3.0] - 2019-11-02 [BC]
- Initialising ncurses functionality is now done using `ncursesw_entry()` instead of `ncursesw_init()` to provide better panic trapping that can be caught and processed as a `Result`.
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
