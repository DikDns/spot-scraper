use crate::error::{Result, ScraperError};
use crate::models::Course;
use scraper::{ElementRef, Html, Selector};

fn extract_course_from_row(row: ElementRef) -> Result<Course> {
    let cell_selector = Selector::parse("td").unwrap();
    let cells: Vec<_> = row.select(&cell_selector).collect();

    if cells.len() < 5 {
        return Err(ScraperError::ParsingError(
            "Baris tabel mata kuliah tidak memiliki kolom yang cukup".to_string(),
        ));
    }

    let href = cells[1]
        .select(&Selector::parse("a").unwrap())
        .next()
        .and_then(|a| a.value().attr("href"))
        .ok_or_else(|| ScraperError::ElementNotFound("Link mata kuliah (href)".to_string()))?;

    let id = href.split('/').last().unwrap_or("").to_string();

    let course = Course {
        id,
        code: cells[0].text().collect::<String>().trim().to_string(),
        name: cells[1].text().collect::<String>().trim().to_string(),
        credits: cells[2].text().collect::<String>().trim().parse().unwrap_or(0),
        lecturer: cells[3].text().collect::<String>().trim().to_string(),
        academic_year: cells[4].text().collect::<String>().trim().to_string(),
        href: href.to_string(),
    };

    Ok(course)
}

pub fn parse_courses_from_html(html: &str) -> Result<Vec<Course>> {
    let document = Html::parse_document(html);
    let row_selector = Selector::parse("table > tbody > tr").unwrap();

    let mut courses = Vec::new();
    for row in document.select(&row_selector) {
        if let Ok(course) = extract_course_from_row(row) {
            courses.push(course);
        }
    }

    if courses.is_empty() {
        return Err(ScraperError::ElementNotFound(
            "Tidak ada baris mata kuliah yang ditemukan di dalam tabel".to_string(),
        ));
    }

    Ok(courses)
}
