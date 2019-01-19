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
    Update,

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


const NEW : i64 = 0;
const VALID : i64 = 1;
const DELETED : i64 = 2;
const DEACTIVATE : i64 = 4;

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
                creation DATETIME NOT NULL,
                updated DATETIME NOT NULL,
                status INTEGER NOT NULL
            )
        ",
            NO_PARAMS,
        ).map_err(|_| ProviderError::Init)
    }

    pub fn bulkinsert(&mut self, ips: Vec<String>, src: &String, id: u32) -> Result<(), ProviderError> {
        
        trace!("bulkinsert");
        debug!("bulkinsert [src: {}]", src);
        debug!("bulkinsert [ips: {}]", ips.len());

        let now = chrono::Local::now();

        for ip in ips {
            let n = Node {
                id: 0,
                ip: ip,
                src: src.clone(),
                creation: now.timestamp(),
                updated: now.timestamp(),
                status: NEW,
            };
            
            trace!("bulkinsert insert");
            self.conn.execute("
            INSERT OR IGNORE
                INTO node (ip, src, creation, updated, status) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
            &[ 
                &n.ip as &ToSql, 
                &n.src as &ToSql, 
                &n.creation,
                &n.updated,
                &n.status as &ToSql,
            ],
            ).map_err(|_| ProviderError::Insert)?;
        }

        trace!("bulkinsert update");
        self.conn.execute("
            UPDATE node 
                SET updated = ?1, 
                    status = ?2
            WHERE id = ?3
                    ;",
        &[ 
            now.timestamp(),
            VALID,
            id as i64 
        ],
        ).map_err(|_| ProviderError::Update)?;

        Ok(())
    }
    

    pub fn delete(&mut self, id: u32) -> Result<(), ProviderError> {
        
        trace!("delete");
        trace!("delete [id: {}]", id);
        self.update(id, DELETED)
    }
    
    pub fn deactivate(&mut self, id: u32) -> Result<(), ProviderError> {
        
        trace!("delete");
        trace!("delete [id: {}]", id);
        self.update(id, DEACTIVATE)
    }
    

    pub fn update(&mut self, id: u32, status: i64) -> Result<(), ProviderError> {
        
        trace!("update");
        trace!("update [id: {}]", id);
        trace!("update [status: {}]", status);

        let now = chrono::Local::now();
        self.conn.execute("
            UPDATE node 
                SET updated = ?1, 
                    status = ?2
              WHERE id = ?3
                    ;",
        &[ 
            now.timestamp(),
            status,
            id as i64 
        ],
        ).map_err(|_| ProviderError::Update)?;

        Ok(())
    }
    
    pub fn all(&self) -> Result<Vec<Node>, ProviderError> {

        trace!("all");

        let mut stmt = self.conn
            .prepare("
            SELECT id, ip, src, creation, updated, status
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
                updated: row.get(4),
                status: row.get(5)
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
             SELECT id, ip, src, creation, updated, status
               FROM node
              WHERE status = 0
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
                updated: row.get(4),
                status: row.get(5),
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
