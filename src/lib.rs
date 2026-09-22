#[derive(Debug)]
pub struct Downloads {
    pub platform: String,
    pub url: String,
    pub checksum: String,
}

struct Platform {
    url: String,
    checksum: String,
}

enum Platforms {
    macos_arm64(Platform),
    macos_x64(Platform),
    linux_arm64(Platform),
    linux_x64(Platform),
    linux_x64_baseline(Platform),
    linux_arm64_musl(Platform),
    linux_x64_musl(Platform),
    windows_arm64(Platform),
    windows_x64(Platform),
}

fn find_table(http_data: &str) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    use scraper::{ElementRef, Html, Selector};
    let css = |selector| Selector::parse(selector).unwrap();
    let get_cells = |row: ElementRef, selector| {
        row.select(&css(selector))
            .map(|cell| cell.inner_html().trim().to_string())
            .collect()
    };
    let html = Html::parse_document(http_data);
    let table = html.select(&css("table")).next()?;
    let tr = css("tr");
    let mut rows = table.select(&tr);
    let headers = get_cells(rows.next()?, "th");
    let rows: Vec<_> = rows.map(|row| get_cells(row, "td")).collect();
    Some((headers, rows))
}

fn strip_html_tags(s: &str) -> String {
    let mut data = String::new();
    let mut inside = false;
    for c in s.chars() {
        match c {
            '<' => inside = true,
            '>' => inside = false,
            _ if !inside => data.push(c),
            _ => {}
        }
    }
    data
}

pub fn parse_downloads(http_data: &str) -> Option<Vec<Downloads>> {
    let mut downloads: Vec<Downloads> = Vec::new();
    if let Some(table) = find_table(http_data) {
        let (headers, rows) = table;
        let platform_index = headers.iter().position(|h| h == "Platform")?;
        let url_index = headers.iter().position(|h| h == "URL")?;
        let checksum_index = headers.iter().position(|h| h == "Checksum (SHA-512)")?;
        downloads = rows
            .into_iter()
            .map(|row| Downloads {
                platform: row[platform_index].clone(),
                url: { let text = row[url_index].clone();
                       strip_html_tags(&text)
                     },
                checksum: { let text = row[checksum_index].clone();
                            strip_html_tags(&text)
                        },
            })
            .collect();
    }
    
    Some(downloads)
}

pub fn get_file() {
    
}