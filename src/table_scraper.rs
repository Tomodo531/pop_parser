use select::{document::Document, node::Node, predicate::Name};

pub type TableName = String;
pub type Player = Vec<String>;
pub type Team = Vec<Player>;
pub type Tables = Vec<Team>;

pub fn get_table(document: &Document, table_name: &str) -> (Team, Team) {
    let team_tables: Tables = document
        .find(Name("table"))
        .filter(|table| {
            let name: TableName = table.find(Name("th")).next().unwrap().text();
            name == table_name
        })
        .flat_map(|table| get_table_data(table))
        .collect();

    (team_tables[0].clone(), team_tables[1].clone())
}

fn get_table_data(table: Node) -> Tables {
    table
        .find(Name("tbody"))
        .map(|tbody| get_table_row(tbody))
        .collect()
}

fn get_table_row(tbody: Node) -> Team {
    tbody
        .find(Name("tr"))
        .map(|tr| {
            tr.find(Name("td"))
                .enumerate()
                .map(|(index, td)| {
                    if index == 0 {
                        let tag = td.find(Name("a")).find(|name| name.text() != "🌵");

                        match tag {
                            Some(name) => return name.text(),
                            None => return String::from("NaN"),
                        }
                    }

                    td.text()
                })
                .collect()
        })
        .collect()
}
