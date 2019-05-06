const FOV_MULT : [[i32; 8]; 4] = [
    [1, 0, 0, -1, -1, 0, 0, 1],
    [0, 1, -1, 0, 0, -1, 1, 0],
    [0, 1, 1, 0, 0, -1, -1, 0],
    [1, 0, 0, 1, -1, 0, 0, -1]
];

#[derive(Debug)]
pub struct FovMap {
    map : Vec<Vec<(bool, bool)>>,
}

impl FovMap {
    pub fn new(width: i32, height: i32) -> Self {
        FovMap {
            map: vec![vec![(false, false); height as usize]; width as usize],
        }
    }

    pub fn set(&mut self, x : i32, y: i32, val : bool) {
        self.map[x as usize][y as usize].0 = val;
    }

    pub fn is_obstacle(&self, x: i32, y: i32) -> bool {
        self.map[x as usize][y as usize].0
    }

    pub fn is_in_sight(&self, x: i32, y: i32) -> bool {
        self.map[x as usize][y as usize].1
    }

    pub fn compute_fov(&mut self, x: i32, y: i32, radius: i32) {
        self.reset();
        for i in 0..8 {
            self.cast_light(x, y, radius, 1, 1.0, 0.0,
                            FOV_MULT[0][i], FOV_MULT[1][i],
                            FOV_MULT[2][i], FOV_MULT[3][i]);
        }
        self.map[x as usize][y as usize].1 = true;
    }

    fn reset(&mut self) {
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                self.map[i][j] = (self.map[i][j].0, false);
            }
        }
    }

    fn get_width(&self) -> usize {
        self.map.len()
    }

    fn get_height(&self) -> usize {
        if self.map.len() <= 0 {
            0
        } else {
            self.map[0].len()
        }
    }

    // Implementation of recursive shadowcasting
    fn cast_light(&mut self, char_x : i32, char_y: i32, radius: i32, row: i32,
                  initial_start_slope: f32, end_slope: f32, // 1.0 / 0.0
                  xx: i32, xy: i32, yx: i32, yy: i32) { // 1 / 0 / 0 / 1
        if initial_start_slope < end_slope {
            return;
        }
        let mut start_slope = initial_start_slope;
        let mut next_start_slope = initial_start_slope;
        for i in row..radius + 1 {
            let mut blocked = false;
            let dy = -i;
            for dx in -i..1 {
                let l_slope = ((dx as f32) - 0.5) / ((dy as f32) + 0.5); // -1.5 / -0.5 == 3
                let r_slope = ((dx as f32) + 0.5) / ((dy as f32) - 0.5); // -0.5 / -1.5 == 1/3

                if start_slope < r_slope {
                    continue;
                } else if end_slope > l_slope {
                    break;
                }

                let from_char_x = dx * xx + dy * xy;
                let from_char_y = dx * yx + dy * yy;
                if (from_char_x < 0 && from_char_x.abs() > char_x) ||
                    (from_char_y < 0 && from_char_y.abs() > char_y) {
                    continue;
                }

                let map_x = char_x + from_char_x;
                let map_y = char_y + from_char_y;
                if map_x >= (self.get_width() as i32) || map_y >= (self.get_height() as i32) {
                    continue;
                }

                let radius2 = radius * radius;
                if (dx * dx + dy * dy) < radius2 {
                    self.map[map_x as usize][map_y as usize].1 = true;
                }

                if blocked {
                    if self.is_obstacle(map_x, map_y) {
                        next_start_slope = r_slope;
                        continue;
                    } else {
                        blocked = false;
                        start_slope = next_start_slope;
                    }
                } else if self.is_obstacle(map_x, map_y) {
                    blocked = true;
                    next_start_slope = r_slope;
                    self.cast_light(char_x, char_y, radius, i + 1,
                                    start_slope, l_slope,
                                    xx, xy, yx, yy);
                }
            }
            if blocked {
                break;
            }
        }
    }
}
