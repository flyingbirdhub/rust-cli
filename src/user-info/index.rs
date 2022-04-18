use run_script::ScriptOptions;

struct UserInfo {
    name: String,
    email: String,
}

impl UserInfo {
    pub fn new() {
        let (code, name, error) = run_script::run("whoami", vec![], ScriptOptions::new()).unwrap();
        if code == 0 {
            
        }
    }
}