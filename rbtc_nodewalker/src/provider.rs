extern crate rusqlite;

use crate::node::Node;

use std::path::Path;
use std::fmt;

use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};


#[derive(Debug)]
pub enum ProviderError {
    New,
    Init,
    Insert,
    InsertIterator,
    Select,
    SelectIterator,

    Transaction,
    Savepoint,
    Commit,
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct NodeProvider {
    conn: Connection,
}

impl NodeProvider {

    pub fn new(path: &Path) ->  Result<NodeProvider, ProviderError> {

        trace!("new");

        let conn = Connection::open(path).unwrap();
        let provider = NodeProvider {
            conn: conn,
        };

        provider.init().map_err(|_| ProviderError::New)?;

        Ok(provider)
    }

    fn init(&self) -> Result<usize, ProviderError> {

        trace!("init");

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

    pub fn bulkinsert(&mut self, ips: Vec<String>, src: &String) -> Result<(), ProviderError> {
        
        trace!("bulkinsert");
        trace!("bulkinsert [ips: {}]", ips.len());

        let now = chrono::Local::now();
        let mut tx = self.conn.transaction().map_err(|_| ProviderError::Transaction)?;
        {
            let sp = tx.savepoint().map_err(|_| ProviderError::Savepoint)?;

            for ip in ips {
                let n = Node {
                    id: 0,
                    ip: ip,
                    src: src.clone(),
                    creation: now.timestamp(),
                };
                

                sp.execute("INSERT OR IGNORE INTO node (ip, src, creation) VALUES (?1, ?2, ?3)",
                &[ 
                    &n.ip as &ToSql, 
                    &n.src as &ToSql, 
                    &n.creation
                ],
                ).map_err(|_| ProviderError::Insert)?;

            }
        }
        tx.commit().map_err(|_| ProviderError::Commit)?;

        Ok(())
    }
    
    pub fn all(&self) -> Result<Vec<Node>, ProviderError> {

        trace!("all");

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
                creation: row.get(3)
            })
            .map_err(|_| ProviderError::Select)?;

        let mut result : Vec<Node> = Vec::new();
        for item in iter {
            let node = item.map_err(|_| ProviderError::SelectIterator)?;
            result.push(node);
        }

        Ok(result)
    }


    pub fn ten(&self) -> Result<Vec<Node>, ProviderError> {

        trace!("ten");

        let mut stmt = self.conn
            .prepare("
            SELECT  id, ip, src, creation 
              FROM node
              ORDER BY id DESC
              LIMIT 1000;
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
