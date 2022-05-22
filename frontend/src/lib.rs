use zoon::*;

mod app;
mod header;
mod router;
mod petlist_page;
mod courselist_page;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    router::router();
    start_app("main", app::root);
}
