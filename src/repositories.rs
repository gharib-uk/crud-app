use diesel::result::QueryResult;
use diesel::prelude::*;
use super::models::*;
use super::schema::*;

pub struct MemberRepository;

impl MemberRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Member> {
        members::table.find(id).get_result::<Member>(c)
    }

    pub fn find_multiple(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Member>> {
        members::table.order(members::id.desc()).limit(limit).load::<Member>(c)
    }

    pub fn create(c: &mut SqliteConnection, member: NewMember) -> QueryResult<Member> {
        diesel::insert_into(members::table)
            .values(member)
            .execute(c)?;

        let last_id = Self::last_inserted_id(c)?;
        Self::find(c, last_id)
    }

    pub fn save(c: &mut SqliteConnection, id: i32, member: Member) -> QueryResult<Member> {
        diesel::update(members::table.find(id))
            .set((
                members::name.eq(member.name.to_owned()),
                members::email.eq(member.email.to_owned())
            ))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(members::table.find(id)).execute(c)
    }

    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        members::table.select(members::id).order(members::id.desc()).first(c)
    }
}
