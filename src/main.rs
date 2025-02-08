use axum::{
    Router,
    http::header::{CACHE_CONTROL, CONTENT_TYPE},
    response::IntoResponse,
    routing::get,
};
use chrono::{DateTime, Datelike, Duration, Utc, Weekday};

const FAVICON: &[u8] = include_bytes!("favicon.ico");

const MONDAY: &[u8] = include_bytes!("gifs/monday.gif");
const TUESDAY: &[u8] = include_bytes!("gifs/tuesday.gif");
const WEDNESDAY: &[u8] = include_bytes!("gifs/wednesday.gif");
const THURSDAY: &[u8] = include_bytes!("gifs/thursday.gif");
const FRIDAY: &[u8] = include_bytes!("gifs/friday.gif");
const SATURDAY: &[u8] = include_bytes!("gifs/saturday.gif");
const SUNDAY: &[u8] = include_bytes!("gifs/sunday.gif");
const CHRISTMAS: &[u8] = include_bytes!("gifs/christmas.gif");
const INDEPENDENCE: &[u8] = include_bytes!("gifs/independence.gif");
const NEW_YEARS: &[u8] = include_bytes!("gifs/new_years.gif");
const MIKU: &[u8] = include_bytes!("gifs/miku.gif");
const NACHO: &[u8] = include_bytes!("gifs/nacho.gif");
const BAGEL: &[u8] = include_bytes!("gifs/bagel.gif");
const TRANSGENDER: &[u8] = include_bytes!("gifs/transgender.gif");
const JELLYFISH: &[u8] = include_bytes!("gifs/jellyfish.gif");

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index)).route(
        "/favicon.ico",
        get(|| async { ([(CONTENT_TYPE, "image/x-icon")], FAVICON) }),
    );

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
    let (month, day) = (now.month(), now.day());
    if month == 12 && day == 25 {
        return CHRISTMAS;
    } else if month == 7 && day == 4 {
        return INDEPENDENCE;
    } else if month == 1 && day == 1 {
        return NEW_YEARS;
    } else if month == 3 && day == 9 {
        return MIKU;
    } else if month == 11 && day == 6 {
        return NACHO;
    } else if month == 1 && day == 15 {
        return BAGEL;
    } else if month == 11 && (day >= 13 && day <= 19) {
        return TRANSGENDER;
    } else if month == 11 && day == 3 {
        return JELLYFISH;
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
