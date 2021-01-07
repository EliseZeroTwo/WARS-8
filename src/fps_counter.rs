pub struct FpsCounter (u32, u32);

impl FpsCounter {
    pub fn new(timer: u32) -> FpsCounter {
        FpsCounter((timer / 1000) % 10, 0)
    }
    
    pub fn tick(&mut self, timer: u32) {
        let sec = (timer / 1000) % 10;
        if sec != self.0 {
            println!("FPS: {}", self.1);
            self.1 = 0;
            self.0 = sec % 10;
        }
        self.1 += 1;
    }
}