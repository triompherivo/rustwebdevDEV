#![warn(clippy::all)]
use warp::{http::Method, Filter};

use handle_errors::return_error;

//mod error;
mod store;
mod types;

mod routes;

/*impl Question {
    fn new(id:QuestionId,title:String,content:String,tags:Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}*/

/*impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id:&str) -> Result<Self,Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput,"No id provided")
            ),
        }
    }
}*/

#[tokio::main]
async fn main() {

    //env_logger::init();
    log4rs::init_file("log4rs.yaml",Default::default()).unwrap();

    log::error!("This is an error!");
    log::info!("This is info");
    log::warn!("This is a warning!");

    let log= warp::log::custom(|info| {
        log::info!(
            "{} {} {} {:?} from {} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        );
    });



    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let id_filter=warp::any().map(|| uuid::Uuid::new_v4().to_string());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and(id_filter)
        .and_then(routes::question::get_questions);
    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);
    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);
    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .with(log)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
