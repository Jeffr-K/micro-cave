use serde::{ Serialize } ;

#[derive(serde::Serialize)]
pub struct Status {
  pub status: String
}