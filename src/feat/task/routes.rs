use actix_web::web;

use super::controller::{create, index};

pub fn init(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(create);
}
