use serde::{Deserialize, Serialize};
use surrealdb::engine::local::RocksDb;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize)]
struct Game<'a> {
    title: &'a str,
    score: u32,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection
    let db = Surreal::new::<RocksDb>("./storage").await?;

    // Select a specific namespace / database
    db.use_ns("luke").use_db("games").await?;

    // // Create a new person with a random id
    // let created: Vec<Record> = db
    //     .create("games")
    //     .content(Game {
    //         title: "Fallout 3",
    //         score: 85,
    //     })
    //     .await?;
    // dbg!(created);

    // // Create a new person with a random id
    // let created: Vec<Record> = db
    //     .create("games")
    //     .content(Game {
    //         title: "Fallout New Vegas",
    //         score: 100,
    //     })
    //     .await?;
    // dbg!(created);

    // // Create a new person with a random id
    // let created: Vec<Record> = db
    //     .create("games")
    //     .content(Game {
    //         title: "Fallout 4",
    //         score: 75,
    //     })
    //     .await?;
    // dbg!(created);

    // let created: Vec<Record> = db
    //     .create("games")
    //     .content(Game {
    //         title: "Fallout 74",
    //         score: 45,
    //     })
    //     .await?;
    // dbg!(created);

    // // Update a person record with a specific id
    // let updated: Option<Record> = db
    //     .update(("person", "jaime"))
    //     .merge(Responsibility { marketing: true })
    //     .await?;
    // dbg!(updated);

    // Select all people records
    let things: Vec<Record> = db.select("games").await?;
    dbg!(things);

    // // Perform a custom advanced query
    // let groups = db
    //     .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
    //     .bind(("table", "person"))
    //     .await?;
    // dbg!(groups);

    Ok(())
}
