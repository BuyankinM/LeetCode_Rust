// 478. Generate Random Point in a Circle
// https://leetcode.com/problems/generate-random-point-in-a-circle/

use rand::{Rng, rngs::ThreadRng};

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    rand: ThreadRng
}

impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius, 
            x_center, 
            y_center, 
            rand: rand::thread_rng()
        }        
    }
    
    fn rand_point(&mut self) -> Vec<f64> {
        
        let rand_radius = self.rand.gen::<f64>().sqrt() * self.radius;
        let rand_angle = self.rand.gen::<f64>() * 2.0 * std::f64::consts::PI;
        
        let (x, y) = (rand_radius * rand_angle.cos(), rand_radius * rand_angle.sin());
        
        vec![self.x_center + x, self.y_center + y]
    }
}
