use diesel::prelude::*;

use crate::model::person::Person;
use crate::schema::person;

use super::connection;

impl Person {
    pub fn insert_one(nickname: &String, password: &String) -> Option<Self> {
        let conn = &mut connection();

        let records = (person::nickname.eq(nickname), person::password.eq(password));

        diesel::insert_into(person::table)
            .values(records)
            .returning(Self::as_returning())
            .get_result(conn)
            .ok()
    }

    pub fn select_one_by_unique(id: i64) -> Option<Self> {
        let conn = &mut connection();

        person::table.find(id).first(conn).ok()
    }

    pub fn select_one_by_nickname(nickname: &String) -> Option<Self> {
        let conn = &mut connection();

        person::table
            .filter(person::nickname.eq(nickname))
            .limit(1)
            .get_result(conn)
            .ok()
    }

    pub fn select_one_by_nickname_password(nickname: &String, password: &String) -> Option<Self> {
        let conn = &mut connection();

        person::table
            .filter(person::nickname.eq(nickname))
            .filter(person::password.eq(password))
            .limit(1)
            .get_result(conn)
            .ok()
    }

    pub fn update_one_by_unique(id: i64, item: Self) -> Option<Person> {
        let conn = &mut connection();

        diesel::update(person::table.find(id))
            .set(item)
            .returning(Person::as_returning())
            .get_result(conn)
            .ok()
    }
}
