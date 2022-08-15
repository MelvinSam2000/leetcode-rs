/*
    1603 - Parking System
*/
pub struct ParkingSystem([i32; 3]);

impl ParkingSystem {
    // O(1)
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        Self([big, medium, small])
    }

    // O(1)
    pub fn add_car(&mut self, car_type: i32) -> bool {
        let count = &mut self.0[car_type as usize - 1];
        if *count > 0 {
            *count -= 1;
            true
        } else {
            false
        }
    }
}
