use leptos::mount::mount_to_body;
use rust_guitar_app::app::App;

fn main() {
  console_error_panic_hook::set_once();
  // mount_to_body(my_app::App);
  mount_to_body(App);
}
