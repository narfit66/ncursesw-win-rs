## Menu module.

The menu module is still under active development and as of this moment does not work correctly.

When calling `ncursesw::shims::nmenu::new_menu()` we seem to be passing all the `ITEM` pointers correctly, `Menu::current_item()` returns the correct first `MenuItem` and `Menu::item_count()` returns the correct number of items, but when we perform `Menu::menu_items()` the first `MenuItem` has a null (0x00) handle (all the other handle's seem to be correct and in the correct order!) and `Menu::post_menu()` returns a `NCurseswWinError::MenuError { source: NotConnected { func: "post_menu" } }`.
