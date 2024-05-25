use surrealdb::{ engine::local::RocksDb, Surreal };
use surrealdb::sql::{ serde, Thing };
use serde_json::json; // Import serde_json for creating JSON objects

#[derive(Debug, serde::Serialize)]
struct InputData {
    text: String,
}

#[tokio::main]
async fn main() {
    // Create a RocksDB database connection using the Tauri configuration directory
    let db_path = "database";
    println!("Database path: {:?}", db_path);

    // Create the Surreal database connection
    let db_result = Surreal::new::<RocksDb>(db_path).await;

    // Handle the result to access the Surreal instance
    match db_result {
        Ok(mut db) => {
            println!("Database connection established");

            // Select a namespace and database
            if let Err(err) = db.use_ns("user").use_db("chat").await {
                eprintln!("Error selecting namespace and database: {}", err);
                return;
            }
            println!("Namespace and database selected");

            // Define input text content
            let input_text = "Your input text here";

            // Create a new InputData instance
            let input_data = InputData { text: input_text.to_string() };

            // Serialize the input data into JSON format
            let input_json = json!(input_data);

            // Create a new record with the input text content
            match db.create("input").content(input_json).await {
                Ok(created) => {
                    println!("Record created: {:#?}", created);

                    // Ensure only one record is created, as per your requirement
                    if let Some(record) = <Vec<Thing>>::into_iter(created).next() {
                        println!("Entry Created: {:?}", record);
                    } else {
                        println!("No record created");
                    }
                }
                Err(err) => {
                    eprintln!("Error creating record: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error establishing database connection: {}", err);
        }
    }
}
