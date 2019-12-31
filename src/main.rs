extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

fn main() {
    println!("The roster includes: ");
    scrape_team_data("https://www.nba.com/warriors/roster");
}

fn scrape_team_data(url: &str) {
    let mut req = reqwest::get(url).unwrap();
    assert!(req.status().is_success());
    let doc_body = Html::parse_document(&req.text().unwrap());

    let roster = Selector::parse(".roster__player__header__heading").unwrap();

    for roster in doc_body.select(&roster) {
        let rosters = roster.text().collect::<Vec<_>>();
        println!(" {}",rosters[0]);
    }
}