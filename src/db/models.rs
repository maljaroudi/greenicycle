use super::schema::task;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a,'b,'c> {
    pub _name: &'a str,
    pub category: &'b str,
    pub recyclable: &'c str,
}







#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub _name: String,
    pub category: String,
    pub recyclable: String,
}