extern crate dieselroach;
extern crate diesel;

use self::diesel::prelude::*;
use self::dieselroach::*;
use self::dieselroach::models::Post;
use std::env::args;

fn main() {
    use dieselroach::schema::posts::dsl::{posts, published};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i64>().expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}
