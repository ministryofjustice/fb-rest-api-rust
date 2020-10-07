use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::services;
use serde::{Serialize, Deserialize};

#[table_name = "services"]
#[derive(Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
pub struct Service {
    pub id: i32,
    pub name: String
}

impl Service {
    pub fn create(service: Service, connection: &PgConnection) -> Service {
        diesel::insert_into(services::table)
            .values(&service)
            .execute(connection)
            .expect("Error creating new service");

        services::table.order(services::id.desc()).first(connection).unwrap()
    }

    pub fn find_by_id(id: i32, connection: &PgConnection) -> Service {
        services::table.find(id).first(connection).unwrap()
    }
}
