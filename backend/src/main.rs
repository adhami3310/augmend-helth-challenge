use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, io, sync::Arc};
use tokio_postgres::Client;

#[derive(Clone)]
struct AppData {
    connection: Arc<Client>,
}

impl AppData {
    fn database(&self) -> Arc<Client> {
        self.connection.clone()
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum MaritalStatus {
    Single,
    Married,
    Widowed,
    Other(String),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct Survey {
    name: String,
    age: usize,
    email: String,
    marital_status: MaritalStatus,
    seen_therapist: bool,
    meds: Vec<String>,
}

#[post("/survey/{uid}")]
/// submits an answer
async fn survey_insert(
    client: web::Data<AppData>,
    uid: web::Path<String>,
    submission: web::Json<Survey>,
) -> impl Responder {
    let client = client.database();
    client
        .execute(
            "INSERT INTO submissions (uid, json) VALUES ($1, $2)",
            &[
                &uid.into_inner(),
                &serde_json::to_string(&submission).expect("serializer failed"),
            ],
        )
        .await
        .expect("Query failed");
    HttpResponse::Ok()
}

#[get("/survey/{uid}")]
/// checks if a user submitted an answer already
async fn survey_get(client: web::Data<AppData>, uid: web::Path<String>) -> impl Responder {
    let client = client.database();
    HttpResponse::Ok().json(json!(client
        .query(
            "SELECT * FROM submissions WHERE uid=$1",
            &[&uid.into_inner()],
        )
        .await
        .expect("Query failed")
        .first()
        .is_some()))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());

    let (client, connection) = tokio_postgres::connect(&env::var("postgres").unwrap(), connector)
        .await
        .expect("couldn't conntect to database");

    // Keep connection alive
    actix_web::rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create table if it doesn't exist
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS submissions (uid varchar(128) PRIMARY KEY, json text)",
            &[],
        )
        .await
        .expect("creating table failed");

    let app_data = AppData {
        connection: Arc::new(client),
    };

    // start HTTP server on port 8080
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(app_data.clone()))
            .service(survey_insert)
            .service(survey_get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserializer() {
        assert_eq!(
            serde_json::from_str::<Survey>(
                r#"{"name":"Khaleel Al-Adhami","email":"khaleel.aladhami@gmail.com","age":21,"maritalStatus":"single","seenTherapist":true,"meds":["Sertraline"]}"#
            ).unwrap(),
            Survey {
                name: "Khaleel Al-Adhami".to_owned(),
                email: "khaleel.aladhami@gmail.com".to_owned(),
                age: 21,
                marital_status: MaritalStatus::Single,
                seen_therapist: true,
                meds: vec!["Sertraline".to_owned()]
            }
        );
    }
}
