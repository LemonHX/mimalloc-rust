use std::process::Command;
fn main(){
        let status = Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .status();
    match status {
        Ok(status) => {
            if !status.success() {
                panic!("failed to update git submodule due to {:?}",status);
            }
        }
        Err(err) => {
            panic!(
                "failed to clone git submodule of mimalloc due to {:?}",
                &err
            )
        }
    }
}