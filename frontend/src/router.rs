use crate::{
    app::{self, PageId},
    petlist_page, courselist_page,
};
use std::collections::VecDeque;
use zoon::{println, *};

// ------ route_history ------

#[static_ref]
fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

fn push_to_route_history(route: Route) {
    let mut history = route_history().lock_mut();
    if history.len() == 2 {
        history.pop_back();
    }
    history.push_front(route);
}

pub fn previous_route() -> Option<Route> {
    route_history().lock_ref().get(1).cloned()
}

// ------ router ------

#[static_ref()]
pub fn router() -> &'static Router<Route> {
    Router::new(|route: Option<Route>| {
        println!("{}", routing::url());

        let route = match route {
            Some(route) => {
                push_to_route_history(route.clone());
                route
            }
            None => {
                return app::set_page_id(PageId::Unknown);
            }
        };

        match route {
            Route::Petlist => {
                app::set_page_id(PageId::Petlist);
            }
            Route::Courselist => {
                app::set_page_id(PageId::Courselist);
            }
            Route::Root => {
                app::set_page_id(PageId::Home);
            }
        }

    })
}

// ------ Route ------

#[route]
#[derive(Clone)]
pub enum Route {
    #[route("petlist")]
    Petlist,

    #[route("courselist")]
    Courselist,

    #[route()]
    Root,
}