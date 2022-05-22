use crate::{router::Route};
use zoon::*;

// ------ ------
//     View
// ------ ------

pub fn page() -> impl Element {
    Column::new()
        .item(name())
}

fn name() -> impl Element {
    Row::new()
        .item(El::new().child("Teeeeeest"))
}