use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use xml::reader::{EventReader, XmlEvent};
/// offer
///   house
///     name
///   price
///     value
///   area
///     value
///   number
///   facing
/// offer

#[derive(Clone, Debug)]
struct Record {
    offer_id: String,
    house_name: String,
    price_value: String,
    area_value: String,
    number: String, // flat_number
    facing: String,
}

impl Record {
    fn new() -> Record {
        Self {
            offer_id: "".to_string(),
            house_name: "".to_string(),
            price_value: "".to_string(),
            area_value: "".to_string(),
            number: "".to_string(),
            facing: "".to_string(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("file.xml")?;
    let file = BufReader::new(file); // Buffering is important for performance

    let mut res: Vec<Record> = Vec::new();

    let mut record: Record = Record::new();

    let mut we_are_in: &str = "";

    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                let local_name = name.local_name;
                // println!("StartElement local_name: {}", &local_name);

                if local_name == "offer" {
                    for attr in attributes {
                        record.offer_id += &attr.value;
                    }
                }

                if local_name == "house" {
                    we_are_in = "house";
                }
                if local_name == "facing" {
                    we_are_in = "facing";
                }
                if local_name == "number" {
                    we_are_in = "number";
                }

                if local_name == "name" && we_are_in == "house" {
                    we_are_in = "house_name";
                }
            }

            Ok(XmlEvent::Characters(val)) => {
                if we_are_in == "facing" {
                    record.facing = val;
                } else if we_are_in == "number" {
                    record.number = val;
                } else if we_are_in == "house_name" {
                    record.house_name = val;
                }
                // println!("Characters event: {}", val);
            }

            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "offer" {
                    res.push(record.clone());
                    record = Record::new();
                }
                if name.local_name == "number"
                    || name.local_name == "facing"
                    || name.local_name == "name"
                {
                    we_are_in = "";
                }
            }

            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    // println!("{:#?}", res);
    save_csv(res)?;
    Ok(())
}

fn save_csv(records: Vec<Record>) -> std::io::Result<()> {
    let file = File::create("file.csv")?;
    let mut writer = BufWriter::new(file);
    writer.write_fmt(format_args!("offer_id;house_name;flat_number;facing\n"))?;
    for r in records {
        writer.write_fmt(format_args!(
            "{};{};{};{}\n",
            r.offer_id, r.house_name, r.number, r.facing
        ))?
    }
    Ok(())
}
