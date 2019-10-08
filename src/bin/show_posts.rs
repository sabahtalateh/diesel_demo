use diesel::prelude::*;
use diesel_demo::models::Post;
use diesel_demo::*;

fn main() {
    use diesel_demo::schema::posts;

    let connection = establish_connection();
    let result: Vec<Post> = posts::table
        .filter(posts::published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", result.len());
    for post in result {
        println!("{}", post.title);
        println!("---------------");
        println!("{}", post.body);
    }
}
