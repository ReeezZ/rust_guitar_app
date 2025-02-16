mod my_app;
mod fretboard_v2;

use leptos::mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();
    // mount_to_body(my_app::App);
    mount_to_body(fretboard_v2::FretboardV2);
}
