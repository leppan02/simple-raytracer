use super::structs::{Circle, Color, Picture, Point};
pub fn generate() -> Picture {
    let mut ret = Picture::new((2000, 2000));
    let mut circle = Circle::new();
    let step = 2.0 * 0.58 / ret.size.0 as f64;
    for x in 0..ret.size.0 {
        for y in 0..ret.size.1 {
            let direction = ret.direction.pivot(step, x, y, ret.size.0, ret.size.1);
            let fake_r =
                (circle.position.len().powi(2) - direction.dot(&circle.position).powi(2)).sqrt();
            if fake_r < circle.radius {
                let dist = direction.dot(&circle.position)
                    - (circle.radius.powi(2) - fake_r.powi(2)).sqrt();
                println!("{:?}", (dist - 70.0) / 24.0);
                ret.data[x as usize][y as usize].0 = 1.0 - ((dist - 70.0) / 24.0);
            }
        }
    }
    circle.position.1 += 30.0;
    for x in 0..ret.size.0 {
        for y in 0..ret.size.1 {
            let direction = ret.direction.pivot(step, x, y, ret.size.0, ret.size.1);
            let fake_r =
                (circle.position.len().powi(2) - direction.dot(&circle.position).powi(2)).sqrt();
            if fake_r < circle.radius {
                let dist = direction.dot(&circle.position)
                    - (circle.radius.powi(2) - fake_r.powi(2)).sqrt();
                println!("{:?}", (dist - 70.0) / 24.0);
                if 1.0 - ((dist - 70.0) / 24.0) > ret.data[x as usize][y as usize].0 {
                    ret.data[x as usize][y as usize].1 = 1.0 - ((dist - 70.0) / 24.0);
                }
            }
        }
    }
    ret
}
