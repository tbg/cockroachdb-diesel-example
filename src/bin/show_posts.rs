extern crate dieselroach;
extern crate diesel;

use self::dieselroach::*;
use self::dieselroach::models::*;
use self::diesel::prelude::*;

fn main() {
    use dieselroach::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}: {}{}", post.id, post.title, if post.published { "" } else { " (unpublished)" });
        println!("----------\n");
        println!("{}", post.body);
    }
}
