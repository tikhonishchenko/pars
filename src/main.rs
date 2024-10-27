use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()> {
    let successful_parse1 = Grammar::parse(Rule::field, "-273.15")?;
    println!("Результат парсингу числового поля: {:?}", successful_parse1);

    let successful_parse2 = Grammar::parse(Rule::field, "email@example.com")?;
    println!("Результат парсингу текстового поля: {:?}", successful_parse2);

    let successful_parse3 = Grammar::parse(Rule::record, "value,-14.7,value2")?;
    println!("Результат парсингу запису: {:?}", successful_parse3);

    let unparsed_file = fs::read_to_string("files/records.csv").expect("cannot read file");

    let successful_parse4 = Grammar::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next().unwrap();
    println!("Результат парсингу файлу: {:?}", successful_parse4);

    Ok(())
}