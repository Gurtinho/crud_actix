use chrono::{NaiveDateTime, Utc};

pub fn date_now() -> NaiveDateTime {
  chrono::NaiveDateTime::new(Utc::now().date_naive(), Utc::now().time())
}