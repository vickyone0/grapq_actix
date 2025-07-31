// // use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, GraphQLInputObject, RootNode};

// // #[derive(GraphQLObject)]
// // #[graphql(description = "A task in the system")]
// // struct Task {
// //     id: i32,
// //     title: String,
// //     completed: bool,
// // }

// // #[derive(GraphQLInputObject)]
// // struct NewTask {
// //     title: String,
// // }

// // struct QueryRoot;

// // #[graphql_object]
// // impl QueryRoot {
// //     fn api_version() -> String {
// //         "1.0".to_string()
// //     }

// //     fn all_tasks() -> FieldResult<Vec<Task>> {
// //         // In a real application, you would fetch tasks from a database.
// //         let tasks = vec![
// //             Task { id: 1, title: "Learn GraphQL".to_string(), completed: true },
// //             Task { id: 2, title: "Build a GraphQL API".to_string(), completed: false },
// //         ];
// //         Ok(tasks)
// //     }

// //     fn get_task(id: i32) -> FieldResult<Option<Task>> {
// //         // In a real application, you would fetch a task from a database by ID.
// //         let task = if id == 1 {
// //             Some(Task { id: 1, title: "Learn GraphQL".to_string(), completed: true })
// //         } else {
// //             None
// //         };
// //         Ok(task)
// //     }
// // }

// // struct MutationRoot;

// // #[graphql_object]
// // impl MutationRoot {
// //     fn create_task(new_task: NewTask) -> FieldResult<Task> {
// //         // In a real application, you would create a new task in the database.
// //         let task = Task {
// //             id: 3, // In a real application, you would generate a unique ID.
// //             title: new_task.title,
// //             completed: false,
// //         };
// //         Ok(task)
// //     }
// // }

// // type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

// // fn create_schema() -> Schema {
// //     Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
// // }

// // use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// // use juniper::http::{playground::playground_source, GraphQLRequest};
// // use std::sync::Arc;

// // async fn graphql(
// //     st: web::Data<Arc<Schema>>,
// //     data: web::Json<GraphQLRequest>,
// // ) -> impl Responder {
// //     let schema = st.into_inner();
// //     let res = data.execute(&schema, &()).await;
// //     Ok::<_, serde_json::Error>(HttpResponse::Ok()
// //         .content_type("application/json")
// //         .body(serde_json::to_string(&res)?))
// // }

// // async fn graphql_playground() -> impl Responder {
// //     HttpResponse::Ok()
// //         .content_type("text/html; charset=utf-8")
// //         .body(playground_source("/graphql", None))
// // }

// // #[actix_web::main]
// // async fn main() -> std::io::Result<()> {
// //     let schema = Arc::new(create_schema());

// //     println!("Server started: http://localhost:8080");

// //     HttpServer::new(move || {
// //         App::new()
// //             .app_data(web::Data::new(schema.clone()))
// //             .service(web::resource("/graphql").route(web::post().to(graphql)))
// //             .service(web::resource("/playground").route(web::get().to(graphql_playground)))
// //     })
// //     .bind("127.0.0.1:8080")?
// //     .run()
// //     .await
// // }

// use actix_web::{App, HttpServer};
// use tonic::transport::Server;
// use crate::greeter::greeter_server::GreeterServer;
// use crate::greeter_server::MyGreeter;

// mod greeter;
// mod greeter_server;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:50051".parse().unwrap();
//     let greeter = MyGreeter::default();

//     println!("GreeterServer listening on {}", addr);

//     Server::builder()
//         .add_service(GreeterServer::new(greeter))
//         .serve(addr)
//         .await?;

//     Ok(())
// }

// use tokio_postgres::{NoTls};
// use time::Instant;


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let start = Instant::now();

//     let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=password dbname=test", NoTls).await?;

//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!{
//                 " connection error: {}",e
//             };
//         }
//     });

//     let rows = client.query("SELECT pg_sleep(1)", &[]).await?;

//     let duration = Instant::now() -start;
//     println!("Async query took: {}", duration);

//     Ok(())
// }


use postgres::{Client, NoTls};
use time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let mut client = Client::connect("host=localhost user=postgres password=password dbname=test", NoTls)?;

    let rows = client.query("SELECT pg_sleep(1);", &[])?;

    let duration = Instant::now() - start;
    println!("Sync query took: {:.2?}", duration);

    Ok(())
}