use log::info;
use oracle::{Connection};
use crate::models::post::{Post, CreatePost, UpdatePost};

pub struct PostRepository;

impl PostRepository {
    pub fn create_post(conn: &Connection, post: &CreatePost) -> Result<(), oracle::Error> {
        let sql = "INSERT INTO POSTS (ID, TITLE, DESCRIPTION, CONTENT) VALUES (:id, :title, :description, :content)";
        let mut stmt = conn.prepare(sql, &[])?;
        stmt.execute(&[&post.id, &post.title, &post.description, &post.content])?;
        conn.commit()?; // Commit the transaction
        Ok(())
    }
    pub fn read_post(conn: &Connection, id: i32) -> Result<Post, oracle::Error> {
        let sql = "SELECT ID, TITLE, DESCRIPTION, CONTENT FROM POSTS WHERE ID = :1";
        let mut stmt = conn.prepare(sql, &[])?;
        let mut rows = stmt.query(&[&id])?;
        if let Some(row) = rows.next() {
            let row = row?;
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                content: row.get(3)?,
            })
        } else {
            Err(oracle::Error::NoDataFound)
        }
    }

    pub fn update_post(conn: &Connection, post: &UpdatePost) -> Result<(), oracle::Error> {
        let sql = "UPDATE POSTS SET TITLE = :1, DESCRIPTION = :2, CONTENT = :3 WHERE ID = :4";
        let mut stmt = conn.prepare(sql, &[])?;
        stmt.execute(&[&post.title, &post.description, &post.content, &post.id])?;
        Ok(())
    }

    pub fn delete_post(conn: &Connection, id: i32) -> Result<(), oracle::Error> {
        let sql = "DELETE FROM POSTS WHERE ID = :1";
        let mut stmt = conn.prepare(sql, &[])?;
        stmt.execute(&[&id])?;
        Ok(())
    }

    pub fn get_all_posts(conn: &Connection) -> Result<Vec<Post>, oracle::Error> {
        let sql = "SELECT ID, TITLE, DESCRIPTION, CONTENT FROM POSTS";
        let mut stmt = conn.prepare(sql, &[])?;
        let mut rows = stmt.query(&[])?;
        let mut posts = Vec::new();
        let mut count = 0;
        for row_result in rows {
            let row = row_result?;

            posts.push(Post {
                id: row.get("ID")?,
                title: row.get("TITLE")?,
                description: row.get("DESCRIPTION")?,
                content: row.get("CONTENT")?,
            });
            count += 1;
        }
        println!("Number of rows: {}", count);
        Ok(posts)
    }
}