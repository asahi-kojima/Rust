//https://www.sheshbabu.com/posts/rust-module-system/
mod config;//ファイル自身をModuleとみなす
mod routes;//moduleとなるディレクトリがある

fn main() {
    config::print_config();
    routes::health_route::print_health_route();
    routes::user_route::print_user_route();
    //models::user_models::print_user_model();
    println!("Hello, world!");
    let x: i32 = 2;
}

/*
|-main.rs
|-config.rs
|-routes
|    |-health_route.rs
|    |-user_route.rs
|-models
|   |-user_model.rs


*/