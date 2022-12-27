use warp::{
    filters::{
        body::BodyDeserializeError,
        cors::CorsForbidden,
    },
    Rejection,
    Reply,
    http::StatusCode,
    reject::Reject,
};
#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f,"cannot parse parameter: {}",err)
            },
            Error::MissingParameters => write!(f,"Missing parameter"),
            Error::QuestionNotFound => write!(f,"Question not found"),
        }
    }
}

impl Reject for Error {}


pub async fn return_error(r:Rejection) -> Result<impl Reply,Rejection> {
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    }

    else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
        /*}
        else if let Some(InvalidId) = r.find() {
            Ok(warp::reply::with_status(
                "No valid ID presented".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            ))*/
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}