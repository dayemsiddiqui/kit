use kit::{get, routes};

use crate::controllers;

routes! {
    get!("/", controllers::home::index),
}
