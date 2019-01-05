extern crate rusqlite;

use crate::node::Node;

use std::path::Path;

use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};

#[derive(Debug)]
pub enum ProviderError {
    New,
    Init,
    Insert,
    InsertIterator,
    Select,
    SelectIterator
}

pub struct NodeProvider {
    conn: Connection,
}

impl NodeProvider {

    pub fn new(path: &Path) ->  Result<NodeProvider, ProviderError> {

        let conn = Connection::open(path).unwrap();
        let provider = NodeProvider {
            conn: conn,
        };

        provider.init().map_err(|_| ProviderError::New)?;

        Ok(provider)
    }

    fn init(&self) -> Result<usize, ProviderError> {

        self.conn.execute("
            CREATE TABLE IF NOT EXISTS node (
                id  INTEGER PRIMARY KEY,
                ip  VARCHAR(64) UNIQUE NOT NULL,
                src VARCHAR(64) NOT NULL,
                creation DATETIME NOT NULL
            )
        ",
            NO_PARAMS,
        ).map_err(|_| ProviderError::Init)
    }

    pub fn insert(&self, n: &Node) -> Result<usize, ProviderError> {

        self.conn.execute(
            "INSERT OR IGNORE INTO node (ip, src, creation) VALUES (?1, ?2, ?3)",
            &[ 
                &n.ip as &ToSql, 
                &n.src as &ToSql, 
                &n.creation 
            ],
            )
            .map_err(|_| ProviderError::Insert)
    }

    pub fn bulkinsert(&self, ips: Vec<String>, src: &String) -> Result<(), ProviderError> {
        
        for ip in ips {
            let node = Node {
                id: 0,
                ip: ip,
                src: src.clone(),
                creation: time::get_time(),
            };
            self.insert(&node).map_err(|_| ProviderError::InsertIterator)?;
        }

        Ok(())
    }
    pub fn all(&self) -> Result<Vec<Node>, ProviderError> {

        let mut stmt = self.conn
            .prepare("
            SELECT id, ip, src, creation 
              FROM node
              ;
              ")
            .unwrap()
            ;

        let iter = stmt
            .query_map(NO_PARAMS, |row| Node {
                id: row.get(0),
                ip: row.get(1),
                src: row.get(2),
                creation: row.get(3),
            })
            .map_err(|_| ProviderError::Select)?;

        let mut result : Vec<Node> = Vec::new();
        for item in iter {
            let node = item.map_err(|_| ProviderError::SelectIterator)?;
            result.push(node);
        }

        Ok(result)
    }
}
