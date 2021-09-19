# errlog

errlog is a simple log library based on anyhow, it wraps file name and line number for your error
message.

Add the following dependencies into your Cargo.toml

    errlog = "v0.0.1"

Import errlog macro

    use errlog::{elog, AnyContext, AnyResult};

Now you can use it in function that return `AnyResult<xxx>` such as

    return Err(elog!("Unkown file type")); 

or

    File:open(filepath).context(elog!("Cannot open file {}", filepath))?; 
