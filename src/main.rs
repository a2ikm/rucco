fn main() {
    println!(".global main");
    println!("main:");
    println!(" mov x8, #93"); // setup exit syscall
    println!(" mov x0, #42"); // exit code is 42
    println!(" svc #0"); // invoke syscall
}
