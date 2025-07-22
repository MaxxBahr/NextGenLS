#[derive(Default)]
pub struct Result{
    path: String,
    size: i32,
    line: String
}

impl Result{
    pub fn new(path: String, size: i32, line: String) -> Result{
        Result { path, size, line }
    }

    pub fn convert_size(&mut self, unit: String){
        match unit.as_str() {
            "KiB" => self.size = self.size / 1024,
            "MiB" => self.size = self.size / i32::pow(1024, 2),
            "GiB" => self.size = self.size / i32::pow(1024, 3),
            _ => ()
        }
    }
}