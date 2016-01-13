//! WARNING: This file is generated, derived from table public.post, DO NOT EDIT

use gen::column;
use gen::schema;
use gen::table;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::dao::Type;
use rustorm::table::Column;
use rustorm::table::IsTable;
use rustorm::table::Table;




#[derive(Debug, Clone)]
pub struct Post {
    /// primary
    /// default: nextval('posts_id_seq'::regclass)
    /// not nullable 
    /// db data type: integer
    pub id: i32,
    /// db data type: text
    pub body: Option<String>,
    /// not nullable 
    /// db data type: character varying
    pub title: String,
    /// not nullable 
    /// db data type: integer
    pub user_id: i32,

}



impl IsDao for Post {
    fn from_dao(dao: &Dao) -> Self {
        Post {
            id: dao.get(column::id),
            user_id: dao.get(column::user_id),
            title: dao.get(column::title),
            body: dao.get_opt(column::body),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set(column::id, &self.id);
        dao.set(column::user_id, &self.user_id);
        dao.set(column::title, &self.title);
        match self.body {
            Some(ref _value) => dao.set(column::body, _value),
            None => dao.set_null(column::body)
        }
        dao
    }
}

impl ToJson for Post {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Post {

    fn default() -> Self {
        Post{
            id: Default::default(),
            user_id: Default::default(),
            title: Default::default(),
            body: Default::default(),
        }
    }
}

impl IsTable for Post {

    fn table() -> Table {
        Table {
            schema: schema::public.to_owned(),
            name: table::post.to_owned(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns: vec![
                Column {
                    name: column::id.to_owned(),
                    data_type: Type::I32,
                    db_data_type: "integer".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: Some("nextval('posts_id_seq'::regclass)".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::user_id.to_owned(),
                    data_type: Type::I32,
                    db_data_type: "integer".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::title.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character varying".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::body.to_owned(),
                    data_type: Type::String,
                    db_data_type: "text".to_owned(),
                    is_primary: false, is_unique: false, not_null: false, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
            ],
            is_view: false,
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static id: &'static str = "post.id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static user_id: &'static str = "post.user_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static title: &'static str = "post.title";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static body: &'static str = "post.body";
