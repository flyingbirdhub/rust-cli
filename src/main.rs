use console::style;

mod user;
use user::user_info::UserInfo;

fn main() {
    println!("This is {} neat", style("quite").cyan());

    let user_info = UserInfo::new().unwrap();

    print!("UserInfo: {:#?}", user_info);
}
