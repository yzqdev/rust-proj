use std::thread;
use std::time::Duration;
use guess::util::req::main_req;

#[test]
  fn req(){
    main_req( );
  thread::sleep(Duration::from_secs(3));
  println!("hello");
}