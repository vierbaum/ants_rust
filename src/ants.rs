use super::vars;

pub struct Ant{
    pub position: [i32; 2],
    pub path: Vec<[i32; 2]>,
    pub mode: String,
    pub is_following_path: bool,
}

impl Ant{

    pub fn get_close(&self, parr: &Vec<Vec<i32>>) -> Vec<[i32; 3]> {
        // Vector wich contains all cells with pheromones
        let mut phers: Vec<[i32; 3]> = vec!();

        // looping through current x - 1 to x + 1 and checking if in boundaries
        for x in self.position[0] - 1..self.position[0] + 2{
            if x >= 0 && x <= vars::SIZEX {

                //same for y
                for y in self.position[1] - 1..self.position[1] + 2{

                    // if wer're not at current position                                                   and cell has pheromones
                    if !(x == self.position[0] && y == self.position[1]) && y >= 0 && y <= vars::SIZEX && parr[x as usize][y as usize] != 0 {

                        // adding [x, y, strength of pheromone] to vector
                        phers.push([x, y, parr[x as usize][y as usize]])
                    }
                }
            }
        }
        return phers;
    }

    pub fn set_new_pos(&mut self, new_pos: [i32; 2]) {
        self.path.push(self.position);
        self.position = new_pos;
    }
}
