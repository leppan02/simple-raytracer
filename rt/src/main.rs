pub mod save;
pub mod structs;
use std::thread;
fn main() {
    let mut picture = structs::Picture::new((1920, 1080));
    let scene = structs::Scene::new(360);
    scene.generate_picture(&mut picture, 20);
    save::save(&picture, 0);
    return;
    let mut handles = vec![];
    for i in 0..8 {
        let trange = (i * 10, (i + 1) * 10);
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
        scene.generate_picture(&mut picture, 20);
        save::save(&picture, t);
    }
}
