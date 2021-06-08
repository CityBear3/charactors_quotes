use super::db::connection;
use super::schema::charactors;
use diesel::prelude::*;
//use diesel::query_dsl::methods::FilterDsl;
use super::api_error::ApiError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, AsChangeset, Insertable)]
#[table_name = "charactors"]
pub struct Charactor {
    pub name: String,
    pub title: String,
    pub quote: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "charactors"]
pub struct Charactors {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub quote: String,
}

#[derive(Serialize)]
pub struct Titles {
    pub titles: Vec<String>,
}

#[derive(Serialize)]
pub struct Names {
    pub names: Vec<String>,
}

impl Charactor {
    fn from(charactor: Charactor) -> Charactor {
        Charactor {
            name: charactor.name,
            title: charactor.title,
            quote: charactor.quote,
        }
    }
}

impl Charactors {
    pub fn view_all() -> Result<Vec<Self>, ApiError> {
        let conn = connection()?;
        let charactors = charactors::dsl::charactors.load::<Charactors>(&conn)?;
        Ok(charactors)
    }

    pub fn search_by_id(id: i32) -> Result<Self, ApiError> {
        let conn = connection()?;
        let charactor = charactors::dsl::charactors.find(id).first(&conn)?;
        Ok(charactor)
    }

    pub fn search_by_name(name: String) -> Result<Vec<Self>, ApiError> {
        let conn = connection()?;
        let charactors = charactors::table
            .filter(charactors::name.eq_all(name))
            .load::<Charactors>(&conn)?;
        Ok(charactors)
    }

    pub fn seach_by_title(title: String) -> Result<Vec<Self>, ApiError> {
        let conn = connection()?;
        let charactors = charactors::dsl::charactors
            .filter(charactors::title.eq_all(title))
            .load::<Charactors>(&conn)?;
        Ok(charactors)
    }

    pub fn create(charactor: Charactor) -> Result<Self, ApiError> {
        let conn = connection()?;
        let charactor = Charactor::from(charactor);
        let charactor = diesel::insert_into(charactors::dsl::charactors)
            .values(charactor)
            .get_result(&conn)?;
        Ok(charactor)
    }

    pub fn update(id: i32, charactor: Charactor) -> Result<Self, ApiError> {
        let conn = connection()?;
        let charactor = diesel::update(charactors::dsl::charactors.find(id))
            .set(charactor)
            .get_result(&conn)?;
        Ok(charactor)
    }

    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let conn = connection()?;
        let res = diesel::delete(charactors::dsl::charactors.find(id)).execute(&conn)?;
        Ok(res)
    }
}

impl Titles {
    pub fn view_titles() -> Result<Titles, ApiError> {
        let conn = connection()?;
        let titles = charactors::dsl::charactors
            .select(charactors::title)
            .load::<String>(&conn)?;
        let title_list = Titles { titles };
        Ok(title_list)
    }
}

impl Names {
    pub fn view_names() -> Result<Names, ApiError> {
        let conn = connection()?;
        let names = charactors::dsl::charactors
            .select(charactors::name)
            .load::<String>(&conn)?;
        Ok(Names { names })
    }
}
