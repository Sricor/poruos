use crate::model::person::Person;

use super::connection;

impl Person {
    pub fn insert_one(nickname: &String, password: &String) -> Option<Self> {
        todo!()
    }

    pub fn select_one_by_unique(id: i64) -> Option<Self> {
        todo!()
    }

    pub fn select_one_by_nickname(nickname: &String) -> Option<Self> {
        todo!()
    }

    pub fn select_one_by_nickname_password(nickname: &String, password: &String) -> Option<Self> {
        todo!()
    }

    pub fn update_one_by_unique(id: i64, item: Self) -> Option<Person> {
        todo!()
    }
}
