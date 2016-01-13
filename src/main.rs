#[macro_use(model, create, collection)] extern crate ohmers;
extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate mdbook;
// Dependencies for the Watch feature
extern crate notify;
extern crate time;
extern crate redis;
extern crate rustc_serialize;
extern crate rustorm;
extern crate codegenta;
use iron::status;
use iron::{Iron, Request, Response, IronResult};

use mount::Mount;
use router::Router;
use staticfile::Static;
use mdbook::MDBook;
use std::path::Path;
use std::fs;
use std::error::Error;
// Uses for the Watch feature
use notify::{RecommendedWatcher, Error as NError,Watcher};
use std::sync::mpsc::channel;
use std::thread;
//redis
use ohmers::{Ohmer, Reference, Set, Collection, get};
use redis::Commands;
use rustc_serialize::Encodable;
use rustorm::query::Query;
use rustorm::pool::ManagedPool;
use gen::public::User;
use codegenta::generator;
mod gen;
fn main() {
    //let watch = watch_book("static");
    //println!("Watch books at: static");

    let url = "postgres://postgres:rustps@localhost/ideo";
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();

    
    //let table = db.as_dev().get_table_metadata("","product_availability", false);
    let all_tables = generator::get_all_tables(db.as_dev());
    
    println!("all_tables: {:#?}", all_tables);

    let _ = Query::insert()
        .set("name", &"admin2")
        .set("bio", &"admin el")
        .into_table("public.user")
        .execute(db.as_ref());

    let users: Vec<User> = Query::select_all()
        .from_table("public.user")
        .collect(db.as_ref()).unwrap();

    for  i in users{
        println!("{}", i.name);
    }

    let mut router = Router::new();
    router
        .get("/", say_hello);

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/assets/", Static::new(Path::new("assets/")))
        .mount("/post/", Static::new(Path::new("post/")));
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}


fn say_hello(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}", req.url.path.join("/"));
    Ok(Response::with((status::Ok, "This request was routed!")))
}

fn build(book_dir: String) -> Result<(), Box<Error>> {
    let path = "../post/".to_owned() + &book_dir;
    println!("File changed: {:?}\nBuilding book...\n", path);	
    let mut book =  MDBook::new(Path::new("static"))   // Path to root
        .set_src(Path::new(&(book_dir.to_owned() + "/src")))      // Path from root to source directory
        .set_dest(Path::new(&path))    // Path from root to output directory
        .read_config();                 // Parse book.json file for configuration
    book.build().unwrap();	           // Parse book.json file for configuration
    Ok(())
}

fn _watch (p: &Path) -> Result<(), NError>{
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));
    try!(watcher.watch(p));
    let _ = rx.recv();
    Ok(())
}

fn watch_book(root: &str) -> Result<(), Box<Error>> {
    let dir = Path::new(root);
    for entry in try!(fs::read_dir(dir)) {
        let entry = try!(entry);
        let metadata = try!(entry.metadata());
        if metadata.is_dir() {
            let book_dir =  entry.file_name().into_string().unwrap();
            let root_dir = root.to_owned() + "/" + &book_dir.clone();
            let path = "../post/".to_owned() + &book_dir.clone();
            let mut book =  MDBook::new(dir)   // Path to root
                .set_src(Path::new(&(book_dir.clone() + "/src")))      // Path from root to source directory
                .set_dest(Path::new(&path))    // Path from root to output directory
                .read_config();                 // Parse book.json file for configuration
            book.build().unwrap();	
            println!("book_dir: {:?}", &root_dir.clone());
            thread::spawn(move || {
                loop {
                    match _watch(Path::new(&root_dir.clone())) {
                        Ok(_) => {
                            if let Err(e) = build(book_dir.clone()) {
                                println!("Failed to build: {:?}", e);
                            }
                        },
                        Err(e) => {
                            println!("Error while building: {:?}", e);
                            panic!();
                        }
                    }
                }
            });

        } 
    }
    Ok(())
}

model!(
    derive { Clone }
    Users {
        username:String = "".to_string();
        password:String = "".to_string();
        email:String = "".to_string();
        nickname:String = "".to_string();
        bio:String = "".to_string();
        tweets: Collection<Tweet> = Collection::new();
    });

model!(
    derive { Clone }
    Tag {
        name:String = "".to_string();
        tweets: Set<Tweet> = Set::new();
    });

model!(
    derive { Clone }
    Tweet {
        indices {
            user: Reference<Users> = Reference::new();
        };
        content:String = "".to_string();
        append:String = "".to_string();
        tags: Set<Tag> = Set::new();
    });

fn redis_cache(){
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    for key in client.scan_match::<_, String>("*").unwrap().into_iter() {
        let _:bool = client.del(key).unwrap();
    }
    //let _:bool = client.del("Users:all").unwrap();
    let mut user = Users { 
        id: 0,
        username: "John".to_string(),
        password: "John".to_string(),
        email: "".to_string(),
        nickname: "".to_string(),
        bio: "".to_string(),
        tweets: Collection::new()
    };
    user.save(&client).unwrap();
    let user2:Users=get(user.id, &client).unwrap();
    println!("user name: {}",user2.username);

    let tt1 = create!(Tweet {
        user: Reference::with_value(&user),
        content: "Rocky IV1".to_string(),
        append: "".to_string(),
        tags:  Set::new()
    }, &client).unwrap();

    let tt2 = create!(Tweet {
        user: Reference::with_value(&user),
        content: "Rocky IV2".to_string(),
        append:  "".to_string(),
        tags: Set::new()
    }, &client).unwrap();

    let tg1 = create!(Tag {
        name:"tag1".to_string(),
        tweets: Set::new()
    }, &client).unwrap();

    let tg2 = create!(Tag {
        name:"tag2".to_string(),
        tweets: Set::new()
    }, &client).unwrap();

    tt1.tags.insert("tags", &tt1, &tg1, &client).unwrap();
    tt1.tags.insert("tags", &tt1, &tg2, &client).unwrap();
    tt2.tags.insert("tags", &tt2, &tg1, &client).unwrap();

    tg1.tweets.insert("tweets", &tg1, &tt1, &client).unwrap();
    tg1.tweets.insert("tweets", &tg1, &tt2, &client).unwrap();
    tg2.tweets.insert("tweets", &tg2, &tt1, &client).unwrap();

    let tags = tt1.tags.query("tags", &tt1, &client).unwrap().try_iter().unwrap().collect::<Vec<_>>();
     //tweet.tags
    for i in &tags {
        println!("tweet1 tag name: {}",i.name);
    }
    let tweets = tg1.tweets.query("tweets", &tg1, &client).unwrap().try_iter().unwrap().collect::<Vec<_>>();
     //tag.tweets
    for i in &tweets {
        println!("tag1 tweet content: {}",i.content);
    }
    //user.tweets
    let tweets = user2.tweets.all("user", &user2, &client).sort("name", None, true, true).unwrap().collect::<Vec<_>>();
    for i in &tweets {
        println!("user tweet content: {}",i.content);
        for t in &tags {
            println!("user tweet tag name: {}",t.name);
        }
    }
}

