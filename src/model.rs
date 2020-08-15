use sqlx::{SqlitePool, FromRow};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Deserialize)]
pub struct Info {
    pub id: i32,
}
#[derive(Serialize, FromRow)]
pub struct HeadJeu {
    pub id: i32,
    pub title: String,
    pub time: String,
}

impl HeadJeu {
    pub async fn query(pool: &SqlitePool) -> Result<Vec<HeadJeu>> {
        let mut jeu: Vec<HeadJeu> = vec![];
        let recs = sqlx::query!(
                r#"
                    SELECT id, title, time
                        FROM jeu
                    ORDER BY id DESC
                "#
            )
            .fetch_all(pool)
            .await?;
        for rec in recs {
            jeu.push(HeadJeu {
                id: rec.id,
                title: rec.title,
                time: rec.time
            });
        }
        Ok(jeu)
    }
    pub async fn details(id: i32, pool: &SqlitePool) -> Result<String> {
        let rec = sqlx::query!(
                r#"
                    SELECT article
                        FROM jeu
                    WHERE id = $1
                "#,
                id
            )
            .fetch_one(pool)
            .await?;
        Ok(rec.article)
    }
}

#[derive(Serialize, FromRow)]
pub struct Edit {
    // pub id: i32,
    // pub title: String,
    pub author: String,
    pub chapter: String,
    pub article: String,
    pub time: String,
}

impl Edit {
    pub async fn details_(id: i32, pool: &SqlitePool) -> Result<Edit> {
        let rec = sqlx::query!(
                r#"
                    SELECT author, chapter, article, time
                        FROM edit
                    WHERE id = $1
                "#,
                id
            )
            .fetch_one(pool)
            .await?;
        Ok(Edit {
            author: rec.author,
            chapter: rec.chapter,
            article: rec.article,
            time: rec.time
        })
    }
}

#[derive(Serialize)]
pub struct A {
    pub chapter: String,
    pub author: String,
    pub time: String,
    pub article: Vec<String>
}