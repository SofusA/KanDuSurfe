use crate::functions::get_surfdays::get_surfdays;

mod models;
mod functions;

fn main() {
    let surf_days = get_surfdays(); 

    println!("{:#?}", surf_days);
}
