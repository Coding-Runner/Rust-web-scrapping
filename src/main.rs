extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

fn main(){
    println!("The Delhi Capitals are doing great this season as their results lately are the following:");
    scrape_team_data("https://www.iplt20.com/teams/delhi-capitals/results");
}

fn scrape_team_data(url:&str){

    let mut req = reqwest::get(url).unwrap();
    assert!(req.status().is_success());
    let doc_body = Html::parse_document(&req.text().unwrap());

    let team = Selector::parse(".result__outcome").unwrap();

    for team in doc_body.select(&team){
        let teams = team.text().collect::<Vec<_>>();
        println!("{}", teams[0]);
    }

}

