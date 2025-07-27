//! Creates instances with fake data of models. Does *not* add them to the
//! database.
use crate::models::book::BookBuilder;
use crate::models::user::UserBuilder;
use crate::models::user_book::UserBookBuilder;
use crate::models::{
    book::Book,
    user::User,
    user_book::{ReadingStatus, UserBook},
};
use chrono::{TimeZone, Utc};
use fake::faker::internet::en::*;
use fake::faker::lorem::en::*;
use fake::faker::name::en::*;
use fake::faker::number::en::*;
use fake::{Fake, Faker};

pub fn fake_user() -> User {
    UserBuilder::default()
        .name(Name().fake())
        .email(FreeEmail().fake())
        .password(Password(8..16).fake())
        .build()
        .unwrap()
}

pub fn fake_book() -> Book {
    BookBuilder::default()
        .title(Word().fake())
        .author(Name().fake())
        .isbn(Faker.fake())
        .published_year(NumberWithFormat("19##").fake::<String>().parse().unwrap())
        .description(Some(Sentence(5..10).fake()))
        .cover_url(Some(Faker.fake()))
        .pages((100..=1000).fake())
        .build()
        .unwrap()
}

pub fn fake_user_book(user_id: i64, book_id: i64) -> UserBook {
    UserBookBuilder::default()
        .user_id(user_id)
        .book_id(book_id)
        .status(Faker.fake::<ReadingStatus>())
        .rating(Some((1..=10).fake()))
        .added_at(Utc::now())
        .began_reading(Some(Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap()))
        .done_reading(None)
        .current_page(Some((1..=500).fake()))
        .build()
        .unwrap()
}
