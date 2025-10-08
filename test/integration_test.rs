use spot_scraper::{Result, SpotClient, Course, User};
use std::env;

#[tokio::test]
async fn test_login_and_scrape_data() -> Result<()> {
    let user = client.get_user_profile().await?;
    println!("Successfully fetched profile for: {}", user.name);
    assert_eq!(user.nim, nim, "The fetched NIM should match the login NIM");

    println!("\nFetching courses...");
    let courses = client.get_courses().await?;
    println!("Successfully fetched {} courses.", courses.len());

    assert!(!courses.is_empty(), "Course list should not be empty");

    if let Some(first_course) = courses.first() {
        println!("Sample course: {} ({})", first_course.name, first_course.code);
    }

    Ok(())
}
