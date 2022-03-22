use structopt::StructOpt;
use sqlx::sqlite::SqlitePool;
use std::{thread, time};


#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "s")]
    sleep: bool
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  let opts = Opt::from_args();
  let pool = SqlitePool::connect("sqlite://test.db").await?;

  {
      let mut conn = pool.acquire().await?;
      
      // Create a table if not existing
      sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS contacts (
          contact_id INTEGER PRIMARY KEY,
          name TEXT NOT NULL
        );"#
      )
      .execute(&mut conn)
      .await?;
  }

  {
      let mut conn = pool.acquire().await?;
      // insert some new data
      let res: (i64, ) = sqlx::query_as("insert into contacts (name) values ($1) returning contact_id")
          .bind("JamesBond")
          .fetch_one(&mut conn)
          .await?;
      println!("With sleep = {:?}, Inserted contact id: {:?}", opts.sleep, res.0);
  }


  // Things start to go wrong when this sleep is removed
  if opts.sleep {
      thread::sleep(time::Duration::from_millis(1));
  }

  Ok(())
}
