use diesel::{self, result::QueryResult, prelude::*};
use std::time::SystemTime;

mod schema {
    table! {
        records {
            id -> Int4,
            name -> Text,
	    challenge_id -> Text,
            toc -> Timestamp,
        }
    }
}

use self::schema::records;
use self::schema::records::dsl::{records as all_records};

#[table_name="records"]
#[derive(serde::Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub challenge_id: String,
    pub toc: SystemTime,
}

impl Record {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Record>> {
        all_records.order(records::id.desc()).load::<Record>(conn)
    }
}
