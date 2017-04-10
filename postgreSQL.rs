extern crate postgres;

use postgres::{Connection, TlsMode};

struct Spells {
    name: String,
    dmg: i32,
}

struct Dokemon {
    name: String,
    spell: Spells,
}

fn update() -> Result<u64, postgres::error::Error> {
    let conn = Connection::connect("postgres://postgres@localhost/trash_yard", TlsMode::None).unwrap();
    conn.execute("UPDATE spells SET dmg = $1 WHERE dmg = $2", &[&24, &12])
}

fn main() {

    let sadness = Spells{ name: "U re a fagt".to_string(), dmg: 12 };
    let sad_frog = Dokemon{ name: "Pepe".to_string(), spell: sadness};

    let conn = Connection::connect("postgres://postgres@localhost/trash_yard", TlsMode::None).unwrap();

    //conn.execute("INSERT INTO spells (name, dmg) VALUES ($1, $2)", &[&sad_frog.spell.name, &sad_frog.spell.dmg]).unwrap();
    //conn.execute("INSERT INTO dokemons (name, spell) VALUES ($1, $2)", &[&sad_frog.name, &sad_frog.spell.name]).unwrap();
    for row in &conn.query("SELECT * FROM spells", &[]).unwrap() {
        let spell = Spells {
            name: row.get(0),
            dmg: row.get(1),
        };
        println!("Found spell name: {}, dmg: {}.", spell.name, spell.dmg);
    }
    let updates = update().unwrap();
    println!("{} rows were updated", updates);
    for row in &conn.query("SELECT * FROM spells", &[]).unwrap() {
        let spell = Spells {
            name: row.get(0),
            dmg: row.get(1),
        };
        println!("Found spell name: {}, dmg: {}.", spell.name, spell.dmg);
    }
}
