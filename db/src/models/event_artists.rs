use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use models::{Artist, Event};
use schema::{artists, event_artists};
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
use utils::errors::*;
use uuid::Uuid;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[belongs_to(Event)]
#[belongs_to(Artist)]
#[table_name = "event_artists"]
pub struct EventArtist {
    pub id: Uuid,
    pub event_id: Uuid,
    pub artist_id: Uuid,
    pub rank: i32,
    pub set_time: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub importance: i32,
    pub stage_id: Option<Uuid>,
}

#[derive(Insertable)]
#[table_name = "event_artists"]
pub struct NewEventArtist {
    pub event_id: Uuid,
    pub artist_id: Uuid,
    pub rank: i32,
    pub set_time: Option<NaiveDateTime>,
    pub importance: i32,
    pub stage_id: Option<Uuid>,
}

impl NewEventArtist {
    pub fn commit(&self, conn: &PgConnection) -> Result<EventArtist, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::InsertError,
            "Could not add artist to event",
            diesel::insert_into(event_artists::table)
                .values(self)
                .get_result(conn),
        )
    }
}

impl EventArtist {
    pub fn create(
        event_id: Uuid,
        artist_id: Uuid,
        rank: i32,
        set_time: Option<NaiveDateTime>,
        importance: i32,
        stage_id: Option<Uuid>,
    ) -> NewEventArtist {
        NewEventArtist {
            event_id,
            artist_id,
            rank,
            set_time,
            importance,
            stage_id,
        }
    }

    pub fn find_all_from_event(
        event_id: Uuid,
        conn: &PgConnection,
    ) -> Result<Vec<DisplayEventArtist>, DatabaseError> {
        let results: Vec<(EventArtist, Artist)> = event_artists::table
            .inner_join(artists::table)
            .filter(event_artists::event_id.eq(event_id))
            .select((event_artists::all_columns, artists::all_columns))
            .load(conn)
            .to_db_error(ErrorCode::QueryError, "Could not load artists for event")?;

        let mut display_results = Vec::new();
        for x in results {
            display_results.push(DisplayEventArtist {
                event_id: x.0.event_id,
                artist: x.1,
                rank: x.0.rank,
                set_time: x.0.set_time,
                importance: x.0.importance,
                stage_id: x.0.stage_id,
            })
        }
        Ok(display_results)
    }

    pub fn clear_all_from_event(event_id: Uuid, conn: &PgConnection) -> Result<(), DatabaseError> {
        let _result =
            diesel::delete(event_artists::table.filter(event_artists::event_id.eq(event_id)))
                .execute(conn)
                .to_db_error(ErrorCode::DeleteError, "Could not delete event artists.")?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DisplayEventArtist {
    pub event_id: Uuid,
    pub artist: Artist,
    pub rank: i32,
    pub set_time: Option<NaiveDateTime>,
    pub importance: i32,
    pub stage_id: Option<Uuid>,
}
