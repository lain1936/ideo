//! WARNING: This file is generated, derived from table public.comment, DO NOT EDIT

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
pub struct Comment {
    /// primary
    /// default: nextval('comments_id_seq'::regclass)
    /// not nullable 
    /// db data type: integer
    pub id: i32,
    /// not nullable 
    /// db data type: integer
    pub post_id: i32,
    /// not nullable 
    /// db data type: text
    pub text: String,

}



impl IsDao for Comment {
    fn from_dao(dao: &Dao) -> Self {
        Comment {
            id: dao.get(column::id),
            post_id: dao.get(column::post_id),
            text: dao.get(column::text),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set(column::id, &self.id);
        dao.set(column::post_id, &self.post_id);
        dao.set(column::text, &self.text);
        dao
    }
}

impl ToJson for Comment {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for Comment {

    fn default() -> Self {
        Comment{
            id: Default::default(),
            post_id: Default::default(),
            text: Default::default(),
        }
    }
}

impl IsTable for Comment {

    fn table() -> Table {
        Table {
            schema: schema::public.to_owned(),
            name: table::comment.to_owned(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns: vec![
                Column {
                    name: column::id.to_owned(),
                    data_type: Type::I32,
                    db_data_type: "integer".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: Some("nextval('comments_id_seq'::regclass)".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::post_id.to_owned(),
                    data_type: Type::I32,
                    db_data_type: "integer".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::text.to_owned(),
                    data_type: Type::String,
                    db_data_type: "text".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: false,
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
pub static id: &'static str = "comment.id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static post_id: &'static str = "comment.post_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static text: &'static str = "comment.text";
