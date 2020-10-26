pub mod save;
pub mod structs;

fn main() {
    let mut picture = structs::Picture::new((10000,10000));
    let scene = structs::Scene::new();
    scene.generate_picture(&mut picture, 0);
    save::save(&picture);
}
