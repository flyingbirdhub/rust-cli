use run_script::ScriptOptions;

#[derive(Debug)]
pub struct UserInfo {
    name: String,
    email: String,
}

impl UserInfo {
    pub fn new() -> Result<UserInfo, String> {
        let (code, name, error) = run_script::run("whoami", &vec![], &ScriptOptions::new()).unwrap();
        if code == 0 {
            return Ok(UserInfo { name, email: String::from("") });
        }
        return Err(format!("error occurred when exec whoami, {}", error));
    }
}
