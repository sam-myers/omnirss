use rocket::form::{Errors, FromFormField, ValueField};

pub struct SearchQuery(pub(crate) String);

impl FromFormField<'_> for SearchQuery {
    fn from_value(field: ValueField) -> rocket::form::Result<Self> {
        if field.name == "search" && !field.value.is_empty() {
            return Ok(SearchQuery(field.value.to_string()));
        }
        Err(Errors::from(rocket::form::Error::validation(
            "doesn't contain search",
        )))
    }
}
