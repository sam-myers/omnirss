use std::borrow::Cow;
use rocket::form::{DataField, Errors, FromFormField, ValueField};

use rocket::request::FromParam;

pub struct SearchQuery<'a>(pub(crate) Cow<'a, str>);

impl<'a> FromFormField<'a> for SearchQuery<'a> {
    fn from_value(field: ValueField<'a>) -> rocket::form::Result<'a, Self> {
        if field.name == "search" && !field.value.is_empty() {
            return Ok(SearchQuery(Cow::from(field.value)));
        }
        Err(Errors::from(rocket::form::Error::validation("doesn't contain search")))
    }
}
