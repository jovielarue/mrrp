use chrono::{DateTime, Utc};
use diesel::{
    deserialize::FromSqlRow, expression::AsExpression, pg::Pg, prelude::*, sql_types::Timestamptz,
};
use rocket::form::{self, FromFormField, ValueField};

// Had to create DateTimeUtcForm so that I can create impls for DateTime as you can't impl for types
// defined in external crates
pub struct DateTimeUtcForm {
    pub time_taken: DateTime<Utc>,
}

// getter function
impl DateTimeUtcForm {
    pub fn inner(&self) -> &DateTime<Utc> {
        &self.time_taken
    }

    fn to_naive(&self) -> chrono::NaiveDateTime {
        self.time_taken.naive_utc()
    }
}

// Required for rocket FromForm macro
impl<'r> FromFormField<'r> for DateTimeUtcForm {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match DateTime::parse_from_str(field.value, "%Y-%m-%d %H:%M") {
            Ok(date) => Ok(DateTimeUtcForm {
                time_taken: date.into(),
            }),
            Err(e) => panic!("Error parsing DateTime - {}", e),
        }
    }
}

//// Required for diesel FromSqlRow trait
//impl FromSqlRow<Timestamptz, Pg> for DateTimeUtcForm {
//    fn build_from_row<'a>(
//        row: &impl diesel::row::Row<'a, Pg>,
//    ) -> diesel::deserialize::Result<Self> {
//        // Convert from the SQL `Timestamptz` to DateTime<Utc>
//        let timestamp: DateTime<Utc> = FromSqlRow::<Timestamptz, Pg>::build_from_row(row)?;
//        Ok(DateTimeUtcForm {
//            time_taken: timestamp,
//        })
//    }
//}
