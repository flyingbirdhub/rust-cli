use console::style;
use run_script::ScriptOptions;

fn main() {
    println!("This is {} neat", style("quite").cyan());

    let options = ScriptOptions::new();

    let args = vec![];

    let (code, output, error) = run_script::run("whoami", &args, &options);

    print!("Exit Code: {}", code);
    print!("Ouput: {}", output);
    print!("Error: {}", error);
}
