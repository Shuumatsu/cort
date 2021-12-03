#[test]
fn it_adds_two() {
    // assert_eq!(4, cort::add(2, 2));

    let e = std::process::Command::new("uname")
        .arg("-a")
        .output()
        .unwrap();

    println!("Hello, world! {:?}", e);
}
