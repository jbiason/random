use diesel::Queryable;

#[derive(Queryable)]
pub struct Asset {
    pub name: String,
    pub source: String,
}
