use rocket::{get, http::Status, serde::json::Json};
use serde::Serialize;

use std::sync::Mutex;
use lazy_static::lazy_static;

mod home_assistant;

lazy_static! {
    static ref PEOPLE_IN_ROOM: Mutex<i32> = Mutex::new(1);
}


#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[get("/exit")]
pub async fn exit_handler() -> Result<Json<GenericResponse>, Status> {
    // Gets the number of people in the room from global variable
    let mut people_in_room = PEOPLE_IN_ROOM.lock().unwrap();
    *people_in_room -= 1;

    // Checks if there is no people in the room
    let no_people_in_room: bool = *people_in_room < 1;
    let message = if no_people_in_room {
        // Google Home logic
        "No people in room, lights successfully turned off".to_string()
    } else {
        format!("{} people in room", people_in_room)
    };

    let response = GenericResponse {
        status: "success".to_string(),
        message,
    };

    Ok(Json(response))
}

#[get("/enter")]
pub async fn enter_handler() -> Result<Json<GenericResponse>, Status> {
    // Gets the number of people in the room from global variable
    let mut people_in_room = PEOPLE_IN_ROOM.lock().unwrap();
    *people_in_room += 1;

    // Checks if there is no people in the room
    let room_not_empty: bool = *people_in_room > 0;
    let message = if room_not_empty {
        // Google Home logic
        "Room is no longer empty, Lights turned on".to_string()
    } else {
        "no one in room".to_string()
    };

    let response = GenericResponse {
        status: "success".to_string(),
        message,
    };

    Ok(Json(response))
}