
#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AclList {
    pub id: i32,
    pub acl_name: String,
    pub acl_default: String,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AclNode {
    pub id: i32,
    pub cidr: String,
    pub node_type: String,
}

impl AclList {
    pub fn new() -> Self {
        AclList {
            id:0,
            acl_name: "".to_string(),
            acl_default: "".to_string()
        }
    }
}
