#[macro_use] extern crate rocket;

pub mod data;

use rocket::{response::content::RawHtml, Build, Rocket};
use rocket::http::Status;
use rocket::response::Redirect;
use crate::data::{get_current_stream, get_day_stream, get_current_stream_for_year, get_day_stream_for_year};

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml("<p>https://pnwfrc.live/(event)</p>")
}

// Task 3.1: Current stream handler for /:event
#[get("/<event>")]
fn current_stream(event: &str) -> Result<Redirect, Status> {
    let event_lower = event.to_lowercase();
    match get_current_stream(&event_lower, false) {
        Some(url) => Ok(Redirect::to(url)),
        None => Err(Status::NotFound),
    }
}

// Task 3.2: Current stream alternate handler for /:event/alt
#[get("/<event>/alt", rank = 1)]
fn current_stream_alt(event: &str) -> Result<Redirect, Status> {
    let event_lower = event.to_lowercase();
    match get_current_stream(&event_lower, true) {
        Some(url) => Ok(Redirect::to(url)),
        None => Err(Status::NotFound),
    }
}

// Task 3.3: Day stream handler for /:event/:day
#[get("/<event>/<day>", rank = 2)]
fn day_stream(event: &str, day: &str) -> Result<Redirect, Status> {
    // Parse day as usize, return 404 if invalid
    let day_num = match day.parse::<usize>() {
        Ok(d) => d,
        Err(_) => return Err(Status::NotFound),
    };
    
    // Return 404 if day < 1
    if day_num < 1 {
        return Err(Status::NotFound);
    }
    
    // Convert 1-indexed to 0-indexed
    let day_index = day_num - 1;
    
    // Normalize event code to lowercase
    let event_lower = event.to_lowercase();
    
    // Call get_day_stream and return result
    match get_day_stream(&event_lower, day_index, false) {
        Some(url) => Ok(Redirect::to(url)),
        None => Err(Status::NotFound),
    }
}

// Task 3.4: Day stream alternate handler for /:event/:day/alt
#[get("/<event>/<day>/alt", rank = 1)]
fn day_stream_alt(event: &str, day: &str) -> Result<Redirect, Status> {
    // Parse day as usize, return 404 if invalid
    let day_num = match day.parse::<usize>() {
        Ok(d) => d,
        Err(_) => return Err(Status::NotFound),
    };
    
    // Return 404 if day < 1
    if day_num < 1 {
        return Err(Status::NotFound);
    }
    
    // Convert 1-indexed to 0-indexed
    let day_index = day_num - 1;
    
    // Normalize event code to lowercase
    let event_lower = event.to_lowercase();
    
    // Call get_day_stream with alt = true and return result
    match get_day_stream(&event_lower, day_index, true) {
        Some(url) => Ok(Redirect::to(url)),
        None => Err(Status::NotFound),
    }
}

// Task 6.1: Year current stream handler for /:year/:event
// #[get("/<year>/<event>", rank = 2)]
// fn year_current_stream(year: &str, event: &str) -> Result<Redirect, Status> {
//     // Normalize event code to lowercase
//     let event_lower = event.to_lowercase();
    
//     // Call get_current_stream_for_year and return result
//     match get_current_stream_for_year(year, &event_lower, false) {
//         Some(url) => Ok(Redirect::to(url)),
//         None => Err(Status::NotFound),
//     }
// }

// Task 6.2: Year current stream alternate handler for /:year/:event/alt
// #[get("/<year>/<event>/alt", rank = 2)]
// fn year_current_stream_alt(year: &str, event: &str) -> Result<Redirect, Status> {
//     // Normalize event code to lowercase
//     let event_lower = event.to_lowercase();
    
//     // Call get_current_stream_for_year with alt = true and return result
//     match get_current_stream_for_year(year, &event_lower, true) {
//         Some(url) => Ok(Redirect::to(url)),
//         None => Err(Status::NotFound),
//     }
// }

// Task 6.3: Year day stream handler for /:year/:event/:day
// #[get("/<year>/<event>/<day>", rank = 3)]
// fn year_day_stream(year: &str, event: &str, day: &str) -> Result<Redirect, Status> {
//     // Parse day as usize, return 404 if invalid
//     let day_num = match day.parse::<usize>() {
//         Ok(d) => d,
//         Err(_) => return Err(Status::NotFound),
//     };
    
//     // Return 404 if day < 1
//     if day_num < 1 {
//         return Err(Status::NotFound);
//     }
    
//     // Convert 1-indexed to 0-indexed
//     let day_index = day_num - 1;
    
//     // Normalize event code to lowercase
//     let event_lower = event.to_lowercase();
    
//     // Call get_day_stream_for_year and return result
//     match get_day_stream_for_year(year, &event_lower, day_index, false) {
//         Some(url) => Ok(Redirect::to(url)),
//         None => Err(Status::NotFound),
//     }
// }

// Task 6.4: Year day stream alternate handler for /:year/:event/:day/alt
// #[get("/<year>/<event>/<day>/alt", rank = 1)]
// fn year_day_stream_alt(year: &str, event: &str, day: &str) -> Result<Redirect, Status> {
//     // Parse day as usize, return 404 if invalid
//     let day_num = match day.parse::<usize>() {
//         Ok(d) => d,
//         Err(_) => return Err(Status::NotFound),
//     };
    
//     // Return 404 if day < 1
//     if day_num < 1 {
//         return Err(Status::NotFound);
//     }
    
//     // Convert 1-indexed to 0-indexed
//     let day_index = day_num - 1;
    
//     // Normalize event code to lowercase
//     let event_lower = event.to_lowercase();
    
//     // Call get_day_stream_for_year with alt = true and return result
//     match get_day_stream_for_year(year, &event_lower, day_index, true) {
//         Some(url) => Ok(Redirect::to(url)),
//         None => Err(Status::NotFound),
//     }
// }

pub fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, current_stream, current_stream_alt, day_stream,
                            day_stream_alt]) // , year_current_stream, year_current_stream_alt,
                            // year_day_stream, year_day_stream_alt])
}

#[cfg(test)]
mod tests;
