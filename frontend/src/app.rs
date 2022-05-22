use crate::{
    petlist_page,
    header::header,
    courselist_page,
    router::{previous_route, router, Route},
};
use zoon::*;

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    Home,
    Petlist,
    Courselist,
    Unknown,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Unknown)
}

// ------ ------
//   Commands
// ------ ------

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

// ------ ------
//     View
// ------ ------

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::all(20))
        .s(Spacing::new(20))
        .item(header())
        .item(page())
}
fn page() -> impl Element {
    El::new().child_signal(page_id().signal().map(|page_id| match page_id {
        PageId::Home => El::new().child("Hallo Tierfreund :)").into_raw_element(),
        PageId::Petlist => petlist_page::page().into_raw_element(),
        PageId::Courselist => courselist_page::page().into_raw_element(),
        PageId::Unknown => El::new().child("404").into_raw_element(),
    }))
}