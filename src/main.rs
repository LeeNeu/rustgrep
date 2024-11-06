

fn main() {
    let args = rustgrep::read_args();
    let res = rustgrep::exec_args(args);

    println!("{}", res);
}



