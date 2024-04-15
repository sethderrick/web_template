use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponder};
use serde::{Deserialization, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io.Write;


fn main() {

}
