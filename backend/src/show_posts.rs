use diesel::prelude::*;
use crate::models::*;
use crate::schema::*;
use crate::connect::*;
use crate::schema::posts::published;

pub fn show_posts() {

    let connection = establish_connection();
    let results = posts::table
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-------------\n");
        println!("{}", post.body);
    }

}