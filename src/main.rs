use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error;

pub type Post = Vec<PostElement>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostElement {
  #[serde(rename = "userId")]
  user_id: i64,

  #[serde(rename = "id")]
  id: i64,

  #[serde(rename = "title")]
  title: String,

  #[serde(rename = "body")]
  body: String,
}

struct UserPost {}

impl UserPost {
  fn new() -> UserPost {
    UserPost {}
  }

  #[allow(dead_code)]
  async fn get_posts(&self) -> Result<Post, Box<dyn error::Error>> {
    let resp = reqwest::get("https://jsonplaceholder.typicode.com/posts").await?;
    let posts = resp.json::<Post>().await?;

    Ok(posts)
  }

  #[allow(dead_code)]
  async fn get_post_by_id(&self, id: &u8) -> Result<PostElement, Box<dyn error::Error>> {
    let resp = reqwest::get(format!("https://jsonplaceholder.typicode.com/posts/{}", id)).await?;
    let post = resp.json::<PostElement>().await?;

    Ok(post)
  }

  #[allow(dead_code)]
  async fn create_post(&self, post: PostElement) -> Result<PostElement, Box<dyn error::Error>> {
    let fetch = reqwest::Client::new();

    let json_response = json!({
        "title": post.title,
        "body": post.body,
        "userId": post.user_id
    });

    fetch
      .post("https://jsonplaceholder.typicode.com/posts")
      .json(&json_response)
      .send()
      .await?;

    println!("{:#?}", json_response);

    Ok(post)
  }

  #[allow(dead_code)]
  async fn update_post(
    &self,
    post_id: &u8,
    post_item: &PostElement,
  ) -> Result<(), Box<dyn error::Error>> {
    let fetch = reqwest::Client::new();

    let json_update_values = json!({
      "title": post_item.title,
      "body": post_item.body
    });

    fetch
      .patch(format!(
        "https://jsonplaceholder.typicode.com/posts/{}",
        post_id
      ))
      .json(&json_update_values)
      .send()
      .await?;

    Ok(())
  }

  async fn delete_post_by_id(&self, post_id: &u8) -> Result<(), Box<dyn error::Error>> {
    let fetch = reqwest::Client::new();
    fetch
      .delete(format!(
        "https://jsonplaceholder.typicode.com/posts/{}",
        post_id
      ))
      .send()
      .await?;

    Ok(())
  }
}

// tokio enables async/await
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  let post = UserPost::new();

  // let posts = post.get_posts().await?;

  // let post = post.get_post_by_id(&2).await?;

  // post
  //   .create_post(PostElement {
  //     body: "Hola ewewe".to_string(),
  //     title: "Title Hey!".to_string(),
  //     user_id: 200,
  //     id: 1,
  //   })
  //   .await?;

  // post
  //   .update_post(
  //     &1,
  //     &PostElement {
  //       body: "Hola ewewe 3232".to_string(),
  //       title: "Title Hey updated!".to_string(),
  //       user_id: 200,
  //       id: 1,
  //     },
  //   )
  //   .await?;

  post.delete_post_by_id(&1).await?;

  Ok(())
}
