use crate::schema::user;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub domain_id: i32,
    pub number_alias: Option<String>,
    pub mailbox: Option<String>,
    pub cidr: Option<String>,
    pub user_id: String,
    pub password: String,
    pub toll_allow: Option<String>,
    pub user_context: Option<String>,
    pub default_gateway: Option<String>,
    pub effective_caller_id_name: Option<String>,
    pub effective_caller_id_number: Option<String>,
    pub outbound_caller_id_name : Option<String>,
    pub outbound_caller_id_number: Option<String>,
    pub callgroup: Option<String>,
    pub uservar1: Option<String>,
    pub uservar2: Option<String>,
    pub uservar3: Option<String>,
}

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub domain_id: i32,
    pub number_alias: Option<&'a str>,
    pub mailbox: Option<&'a str>,
    pub cidr: Option<&'a str>,
    pub user_id: &'a str,
    pub password: &'a str,
    pub toll_allow: Option<&'a str>,
    pub user_context: Option<&'a str>,
    pub default_gateway: Option<&'a str>,
    pub effective_caller_id_name: Option<&'a str>,
    pub effective_caller_id_number: Option<&'a str>,
    pub outbound_caller_id_name : Option<&'a str>,
    pub outbound_caller_id_number: Option<&'a str>,
    pub callgroup: Option<&'a str>,
    pub uservar1: Option<&'a str>,
    pub uservar2: Option<&'a str>,
    pub uservar3: Option<&'a str>,
}
