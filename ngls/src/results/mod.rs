#[derive(Default)]
enum Filesize{
    #[default]
    Byte,
    KiB,
    MiB,
    GiB,
}
#[derive(Default)]
pub struct Result{
    path: String,
    size: i32,
    line: String,
    filesize_unit: Filesize,
}



impl Result{
    pub fn new(path: String, size: i32, line: String, filesize_unit: Filesize) -> Result{
        Result { path, size, line, filesize_unit }
    }

    pub fn convert_size(&mut self, unit: String){
        match unit.as_str() {
            "KiB" => {
                self.size = self.size / 1024; 
                self.filesize_unit = Filesize::KiB
            }
            "MiB" => {
                self.size = self.size / i32::pow(1024, 2); 
                self.filesize_unit = Filesize::MiB
            }
            "GiB" => {
                self.size = self.size / i32::pow(1024, 3); 
                self.filesize_unit = Filesize::GiB
            }
            _ => ()
        }
    }
}