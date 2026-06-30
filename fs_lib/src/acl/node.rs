use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::schema::acl_nodes;
use crate::db_connect;
use crate::error::{Error, Result};

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = acl_nodes)]
pub struct AclNode {
    pub id: i32,
    pub list_id: i32,
    pub node_type: String,
    pub cidr: String,
}

#[derive(Insertable)]
#[diesel(table_name = acl_nodes)]
pub struct NewAclNode {
    pub list_id: i32,
    pub node_type: String,
    pub cidr: String,
}

pub fn list_by(a_list_id: Option<i32>) -> Result<Vec<AclNode>> {
    use crate::schema::acl_nodes::dsl::*;

    let mut conn = db_connect();
    let rows = match a_list_id {
        Some(a_list_id) => {
            acl_nodes
                .filter(list_id.eq(a_list_id))
                .load::<AclNode>(&mut conn)?
        },
        None => {
            acl_nodes
                .load::<AclNode>(&mut conn)?
        }
    };

    Ok(rows)
}

pub fn add(list_id_arg: i32, node_type_s: &str, cidr_s: &str) -> Result<i32> {
    use crate::schema::acl_nodes::dsl::*;

    let mut conn = db_connect();
    let inserted: Vec<AclNode> = diesel::insert_into(acl_nodes)
        .values((list_id.eq(list_id_arg), node_type.eq(node_type_s), cidr.eq(cidr_s)))
        .load(&mut conn)?;

    if let Some(first) = inserted.first() {
        Ok(first.id)
    } else {
        Err(Error::Fslib("Failed to insert acl_node".to_string()))
    }
}

pub fn del(node_id_arg: i32) -> Result<()> {
    use crate::schema::acl_nodes::dsl::*;

    let mut conn = db_connect();
    diesel::delete(acl_nodes.filter(id.eq(node_id_arg))).execute(&mut conn)?;
    Ok(())
}
