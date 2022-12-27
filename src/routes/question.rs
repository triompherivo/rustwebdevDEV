use std::collections::HashMap;

use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, QuestionId};
use handle_errors::Error;
use warp::http::StatusCode;

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store, id:String,
) -> Result<impl warp::Reply, warp::Rejection> {
    /*let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string())),
    );
    match question.id.0.parse::<i32>(){
        Err(_) =>{
            Err(warp::reject::custom(InvalidId))
        },
        Ok(_) => {
            Ok(warp::reply::json(
                &question
            ))
        }
    }*/
    /*let mut start = 0;
    if let Some(n) = params.get("start") {
        start = n.parse::<usize>().expect("Could not parse start");
    }
    println!("{}",start);
    let res:Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))*/

    log::info!("{} Start querying questions",id);
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        log::info!("{} Pagination set {:?}",id,&pagination);
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        log::info!("{} No pagination used",id);
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question Updated", StatusCode::OK))
}

pub async fn delete_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => return Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
