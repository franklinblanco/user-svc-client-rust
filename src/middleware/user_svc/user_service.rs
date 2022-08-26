use dev_dtos::{dtos::user::user_dtos::{UserForCreationDto, UserForLoginDto, UserForAuthenticationDto}, domain::user::{user::User, token::Token}, enums::error::Error};
use reqwest::{Client, Method};

use crate::middleware::client::perform_request;

//TODO: Move this into a separate service
const BASE_URL_USER_SVC: &str = "http://localhost:8080";

pub async fn authenticate_user_with_token(client: &Client, user: &UserForAuthenticationDto, user_id: &i32) -> Result<User, Error> {
    perform_request::<&UserForAuthenticationDto, User>(BASE_URL_USER_SVC.to_string(), client, Method::POST, format!("/user/auth/token/{}", user_id), Some(user), 200, vec![(String::from("auth-token"), user.token.clone())]).await
}
pub async fn create_user(client: &Client, user: &UserForCreationDto) -> Result<User, Error> {
    perform_request::<&UserForCreationDto, User>(BASE_URL_USER_SVC.to_string(), client, Method::POST, "/user".to_string(), Some(user), 200, vec![]).await
}
pub async fn authenticate_user_with_password(client: &Client, user: &UserForLoginDto) -> Result<User, Error> {
    perform_request::<&UserForLoginDto, User>(BASE_URL_USER_SVC.to_string(), client, Method::POST, "/user/auth/password".to_string(), Some(user), 200, vec![]).await
}
pub async fn refresh_token_for_user(client: &Client, user: &UserForAuthenticationDto, user_id: &i32) -> Result<Token, Error> {
    perform_request::<&UserForAuthenticationDto, Token>(BASE_URL_USER_SVC.to_string(), client, Method::PATCH, format!("/user/refresh/{}", user_id), None, 200, vec![(String::from("refresh-token"), user.token.clone())]).await
}