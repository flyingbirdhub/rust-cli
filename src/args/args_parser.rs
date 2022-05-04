use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct ArgsParser {
    #[structopt(short, long)]
    name: String,
}