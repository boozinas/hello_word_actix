use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::diesel::prelude::*;
use crate::models::{Users, NewUser};
use crate::schema::users::dsl::{users, id, name, geopoints};
//use crate::schema::users::dsl::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use uuid::Uuid as AUuid;

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

#[derive(Message)]
#[rtype(result="QueryResult<Users>")]
pub struct Create {
    pub name: String,
    pub geopoints: String,
}

#[derive(Message)]
#[rtype(result="QueryResult<Users>")]
pub struct Update {
    pub id: i32,
    pub name: String,
    pub geopoints: String,
}

#[derive(Message)]
#[rtype(result="QueryResult<Users>")]
pub struct Delete {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result="QueryResult<Vec<Users>>")]
pub struct GetUsers;


impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl Handler<Create> for DbActor{
    type Result = QueryResult<Users>;
    
    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_user = NewUser {
            name: Some(msg.name), 
            geopoints: Some(msg.geopoints),
        };

        diesel::insert_into(users)
        .values(new_user)
        .get_result::<Users>(&conn)
    }
} 

impl Handler<Update> for DbActor{
    type Result = QueryResult<Users>;
    
    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        diesel::update(users).filter(id.eq(msg.id))
        .set((name.eq(msg.name), geopoints.eq(msg.geopoints)))
        .get_result::<Users>(&conn)
    }
} 

impl Handler<Delete> for DbActor{
    type Result = QueryResult<Users>;
    
    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        diesel::delete(users)
        .filter(id.eq(msg.id))
        .get_result::<Users>(&conn)
    }
}


impl Handler<GetUsers> for DbActor{
    type Result = QueryResult<Vec<Users>>;
    
    fn handle(&mut self, msg: GetUsers, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        users.select((name, geopoints, id)).get_results::<Users>(&conn)

    }
}