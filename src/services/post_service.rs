use crate::models::post::{Post, CreatePost, UpdatePost};
use crate::repositories::post_repository::PostRepository;
use crate::config::oracle_database::OracleDatabase;

use oracle::Error;

pub struct PostService;

impl PostService {
    pub fn create_post(post: CreatePost) -> Result<(), oracle::Error> {
        let conn = OracleDatabase::get_connection()?;
        PostRepository::create_post(&conn, &post)
    }

    pub fn read_post(id: i32) -> Result<Post, Error> {
        let conn = OracleDatabase::get_connection()?;
        PostRepository::read_post(&conn, id)
    }

    pub fn update_post(post: UpdatePost) -> Result<(), Error> {
        let conn = OracleDatabase::get_connection()?;
        PostRepository::update_post(&conn, &post)
    }

    pub fn delete_post(id: i32) -> Result<(), Error> {
        let conn = OracleDatabase::get_connection()?;
        PostRepository::delete_post(&conn, id)
    }

    pub fn get_all_posts() -> Result<Vec<Post>, Error> {
        let conn = OracleDatabase::get_connection()?;
        PostRepository::get_all_posts(&conn)
    }

}