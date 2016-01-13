//! WARNING: This file is generated, derived from table public.user, DO NOT EDIT

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
pub struct User {
    /// primary
    /// default: nextval('users_id_seq'::regclass)
    /// not nullable 
    /// db data type: integer
    pub id: i32,
    /// db data type: character
    pub bio: Option<String>,
    /// not nullable 
    /// db data type: character
    pub name: String,

}



impl IsDao for User {
    fn from_dao(dao: &Dao) -> Self {
        User {
            id: dao.get(column::id),
            name: dao.get(column::name),
            bio: dao.get_opt(column::bio),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set(column::id, &self.id);
        dao.set(column::name, &self.name);
        match self.bio {
            Some(ref _value) => dao.set(column::bio, _value),
            None => dao.set_null(column::bio)
        }
        dao
    }
}

impl ToJson for User {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl Default for User {

    fn default() -> Self {
        User{
            id: Default::default(),
            name: Default::default(),
            bio: Default::default(),
        }
    }
}

impl IsTable for User {

    fn table() -> Table {
        Table {
            schema: schema::public.to_owned(),
            name: table::user.to_owned(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns: vec![
                Column {
                    name: column::id.to_owned(),
                    data_type: Type::I32,
                    db_data_type: "integer".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: Some("nextval('users_id_seq'::regclass)".to_owned()),
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::name.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::bio.to_owned(),
                    data_type: Type::String,
                    db_data_type: "character".to_owned(),
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
pub static id: &'static str = "user.id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "user.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static bio: &'static str = "user.bio";
