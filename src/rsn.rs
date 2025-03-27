use std::vec;

use common::{c1, c2, database, l, not_found};
use mysql::{prelude::Queryable, *};

pub fn process(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let mut split = query.split_whitespace();
    let command = split.next().unwrap_or("");
    let query = split.collect::<Vec<&str>>().join(" ");

    match command {
        "set" => set(&query, author, rsn_n),
        "del" | "delete" => delete(author, rsn_n),
        "show" => show(author, rsn_n),
        "list" => list(author),
        _ => help(),
    }
}

fn set(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let null = vec!["".to_string()];

    let mut conn = match database::connect() {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error connecting to database: {}", e);
            return Ok(vec!["The database appears to be down!".to_string()]);
        }
    };

    let mut host = author.split("!").collect::<Vec<&str>>()[1];
    if host.starts_with("~") {
        host = host.split("~").collect::<Vec<&str>>()[1];
    }

    match conn.exec_first::<String, &str, Params>(
        "SELECT rsn FROM rsn WHERE host = :host AND rsn_ident = :rsn_n",
        params! { host, rsn_n },
    ) {
        Ok(Some(rsn)) => {
            match conn.exec_drop(
                "UPDATE rsn SET rsn = :query WHERE host = :host AND rsn_ident = :rsn_n",
                params! { query, host, rsn_n },
            ) {
                Ok(_) => Ok::<Vec<String>, ()>(vec![format!(
                    "{} {}",
                    l("RSN"),
                    format!(
                        "{}{} {} {} {} {}",
                        c1("Set rsn #"),
                        c2(rsn_n),
                        c1("from"),
                        c2(&rsn),
                        c1("to"),
                        c2(query)
                    )
                )]),
                Err(e) => {
                    println!(
                        "Error setting rsn #{} for {} to `{}`: {}",
                        rsn_n, author, query, e
                    );

                    Ok(null)
                }
            }
        }
        Ok(None) => {
            match conn.exec_first::<String, &str, Params>(
                "INSERT INTO rsn (host, rsn_ident, rsn, private) VALUES (:host, :rsn_n, :query, 1)",
                params! { host, rsn_n, query },
            ) {
                Ok(_) => Ok::<Vec<String>, ()>(vec![format!(
                    "{} {}",
                    l("RSN"),
                    format!(
                        "{}{} {} {}",
                        c1("Set rsn #"),
                        c2(rsn_n),
                        c1("to"),
                        c2(query)
                    )
                )]),
                Err(e) => {
                    println!(
                        "Error setting rsn{} for {} to `{}`: {}",
                        rsn_n, author, query, e
                    );

                    Ok(vec![format!(
                        "Error setting rsn #{} for {} to `{}`: {}",
                        rsn_n, author, query, e
                    )])
                }
            }
        }
        Err(e) => {
            println!("Error getting rsn #{} for {}: {}", rsn_n, author, e);
            Ok(null)
        }
    }
}

fn delete(author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let mut conn = match database::connect() {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error connecting to database: {}", e);
            return Ok(vec!["The database appears to be down!".to_string()]);
        }
    };

    let mut host = author.split("!").collect::<Vec<&str>>()[1];
    if host.starts_with("~") {
        host = host.split("~").collect::<Vec<&str>>()[1];
    }

    match conn.exec_first::<String, &str, Params>(
        "DELETE FROM rsn WHERE host = :host AND rsn_ident = :rsn_n",
        params! { host, rsn_n },
    ) {
        Ok(_) => Ok(vec![format!(
            "{} {}",
            l("RSN"),
            format!("{}{}", c1("Deleted rsn #"), c2(rsn_n))
        )]),
        Err(e) => {
            println!("Error deleting rsn #{} for {}: {}", rsn_n, author, e);
            Ok(vec![format!(
                "Error deleting rsn #{} for {}",
                rsn_n, author
            )])
        }
    }
}

fn show(author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let null = vec!["".to_string()];

    let mut conn = match database::connect() {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error connecting to database: {}", e);
            return Ok(vec!["The database appears to be down!".to_string()]);
        }
    };

    let mut host = author.split("!").collect::<Vec<&str>>()[1];
    if host.starts_with("~") {
        host = host.split("~").collect::<Vec<&str>>()[1];
    }

    match conn.exec_first::<String, &str, Params>(
        "SELECT rsn FROM rsn WHERE host = :host AND rsn_ident = :rsn_n",
        params! { host, rsn_n },
    ) {
        Ok(Some(rsn)) => Ok(vec![format!(
            "{} {}",
            l("RSN"),
            l(&format!("#{} {}", rsn_n, rsn))
        )]),
        Ok(None) => Ok(vec![format!(
            "{} {}",
            l("RSN"),
            &format!("{}{} {}", c1("No rsn #"), c2(rsn_n), c1("set"))
        )]),
        Err(e) => {
            println!("Error getting rsn #{} for {}: {}", rsn_n, author, e);
            Ok(null)
        }
    }
}

fn list(author: &str) -> Result<Vec<String>, ()> {
    let null = vec!["".to_string()];

    let mut conn = match database::connect() {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error connecting to database: {}", e);
            return Ok(vec!["The database appears to be down!".to_string()]);
        }
    };

    let mut host = author.split("!").collect::<Vec<&str>>()[1];
    if host.starts_with("~") {
        host = host.split("~").collect::<Vec<&str>>()[1];
    }

    match conn.exec(
        "SELECT rsn_ident, rsn FROM rsn WHERE host = :host",
        params! { host, },
    ) {
        Ok(rows) => {
            let mapped = rows
                .into_iter()
                .map(from_row::<(u32, String)>)
                .map(|(id, rsn)| (id.to_string(), rsn));

            Ok(vec![format!(
                "{} {}",
                l("RSN"),
                not_found(
                    mapped
                        .into_iter()
                        .map(|(id, rsn)| l(&format!("#{} {}", &id, &rsn)))
                        .collect::<Vec<String>>()
                )
            )])
        }
        Err(e) => {
            println!("Error getting rsn: {}", e);
            return Ok(null);
        }
    }
}

fn help() -> Result<Vec<String>, ()> {
    Ok(vec![
        "Syntax: -rsn[N] set <name> | -rsn[N] show | -rsn[N] del | -rsn list".to_string(),
        "Examples: -rsn1 set exampleName | -rsn1 show | -rsn1 del | -rsn list".to_string(),
    ])
}
