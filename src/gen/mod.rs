pub mod public;

pub mod schema;
pub mod table;
pub mod column;

use rustorm::table::Table;
use rustorm::table::IsTable;
use gen::public::Comment;
use gen::public::Post;
use gen::public::User;


pub fn get_all_tables() -> Vec<Table> {
    vec![
        Comment::table(),
        Post::table(),
        User::table(),
    ]
}