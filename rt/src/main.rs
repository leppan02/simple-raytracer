pub mod save;
pub mod structs;

fn main() {
    let mut picture = structs::Picture::new((3000,3000));
    let scene = structs::Scene::new();
    scene.generate_picture(&mut picture, 5);
    save::save(&picture);
}
