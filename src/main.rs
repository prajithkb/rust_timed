
extern crate timed;
use std::env;
use timed::timed_block;
use timed::timed_fn;
fn main() {
    let key = "TIMED_ENABLED";
    env::set_var(key, "1");
    timed_block_fn();
    timed_attribute_fn_with_custom_name();
    timed_attribute_fn_with_no_name();
}

fn timed_block_fn() {
    timed_block!("timed_fn_name");
    println!("A timed block function");
}

#[timed_fn(name = "custom_name")]
fn timed_attribute_fn_with_custom_name() {
    println!("A timed attribute function with custom name");
}

#[timed_fn]
fn timed_attribute_fn_with_no_name() {
    println!("A timed attribute function no name");
}


#[cfg(test)]
mod tests {
    use std::io::{self};
    use std::process::{Command, Stdio};

    #[test]
    fn run_timed_macro_tests() -> io::Result<()> {
        {
            println!("Building, running cargo ********");
            let mut c = Command::new("/Users/kprajith/.cargo/bin/cargo")
                .arg("build")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            c.wait()?;
            println!("Completed build, ran cargo ********");
        }
        let child = Command::new("target/debug/timed")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let o = child.wait_with_output()?;
        let output: Vec<String> = std::str::from_utf8(&o.stdout)
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| {
                // Get rid of the timing information since it is not consistent
                let mut ss: Vec<&str> = s.split(":").collect();
                println!("{:?}", ss.len());
                if ss.len() == 4{
                    ss.remove(3);
                    return ss.join(":");
                }
                println!("{:?}", ss);
                return s.to_string()
            })
            .collect();
        assert_eq!(vec!["A timed block function", 
            "[timed]:[function:timed_fn_name]", 
            "A timed attribute function with custom name", 
            "[timed]:[function:timed_attribute_fn_with_custom_name]",
            "A timed attribute function no name", 
            "[timed]:[function:timed_attribute_fn_with_no_name]"], 
             output);
        Ok(())
    }
}


