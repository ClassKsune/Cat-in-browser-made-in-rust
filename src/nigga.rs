use std::io;

pub fn nigga() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here
    stdin.read_line(&mut buffer)?;
    let output = buffer.trim();
    Ok(())
}