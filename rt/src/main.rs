pub mod save;
pub mod structs;
use std::thread;
fn main() {
    let mut handles = vec![];
    for i in 0..16 {
        let trange = (i * 22, (i + 1) * 22);
        let handle = thread::spawn(move || thread_function(trange));
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
}

fn thread_function(trange: (i32, i32)) {
    for t in trange.0..trange.1 {
        let mut picture = structs::Picture::new((1920, 1080));
        let scene = structs::Scene::new(t);
        scene.generate_picture(&mut picture, 10);
        //save::save(&picture, t);
    }
}
