use actix_web::web;

use super::controller::index;

pub fn init(config: &mut web::ServiceConfig) {
    config.service(index);
}
