use crate::schema::voicemails;

#[derive(Queryable)]
pub struct Voicemail {
    pub id: i32,
    pub user_id: i32,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name=voicemails)]
pub struct NewVoicemail<'a> {
    pub user_id: i32,
    pub password: &'a str,
    pub email: Option<&'a str>,
}
