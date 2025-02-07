use axum::{
    Router,
    http::header::{CACHE_CONTROL, CONTENT_TYPE},
    response::IntoResponse,
    routing::get,
};
use chrono::{DateTime, Datelike, Duration, Utc, Weekday};

const MONDAY: &[u8] = include_bytes!("gifs/monday.gif");
const TUESDAY: &[u8] = include_bytes!("gifs/tuesday.gif");
const WEDNESDAY: &[u8] = include_bytes!("gifs/wednesday.gif");
const THURSDAY: &[u8] = include_bytes!("gifs/thursday.gif");
const FRIDAY: &[u8] = include_bytes!("gifs/friday.gif");
const SATURDAY: &[u8] = include_bytes!("gifs/saturday.gif");
const SUNDAY: &[u8] = include_bytes!("gifs/sunday.gif");
const CHRISTMAS: &[u8] = include_bytes!("gifs/christmas.gif");

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    let now = Utc::now();

    let midnight = now.naive_local().date().and_hms_opt(0, 0, 0).unwrap();
    let max_age = (midnight + Duration::days(1) - now.naive_local()).num_seconds();

    (
        [
            (CACHE_CONTROL, format!("public, max-age={}", max_age)),
            (CONTENT_TYPE, "image/gif".to_string()),
        ],
        get_gif(now).await,
    )
}

async fn get_gif<'a>(now: DateTime<Utc>) -> &'a [u8] {
    if now.month() == 12 && now.day() == 25 {
        return CHRISTMAS;
    }

    match now.weekday() {
        Weekday::Mon => MONDAY,
        Weekday::Tue => TUESDAY,
        Weekday::Wed => WEDNESDAY,
        Weekday::Thu => THURSDAY,
        Weekday::Fri => FRIDAY,
        Weekday::Sat => SATURDAY,
        Weekday::Sun => SUNDAY,
    }
}
