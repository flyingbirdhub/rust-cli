use console::style;
use structopt::StructOpt;

mod user;
use user::user_info::UserInfo;

mod args;
use args::args_parser::ArgsParser;

fn main() {
    println!("This is {} neat", style("quite").cyan());

    let args = ArgsParser::from_args();
    println!("{:#?}", args);

    let user_info = UserInfo::new().unwrap();

    print!("UserInfo: {:#?}", user_info);
}
