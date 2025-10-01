#[derive(Default, Debug)]
pub enum Filesize{
    #[default]
    Byte,
    KiB,
    MiB,
    GiB,
}
#[derive(Default, Debug)]
pub struct Result{
    path: String,
    size: u64,
    line: String,
    filesize_unit: Filesize,
}

impl std::fmt::Display for Result{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<20} {:>10}{:?}", self.path, self.size, self.filesize_unit)
    }
}

impl Result{
    pub fn new(path: String, size: u64, line: String, filesize_unit: Filesize) -> Result{
        Result { path, size, line, filesize_unit }
    }

    pub fn convert_size(&mut self, unit: String){
        match unit.as_str() {
            "KiB" => {
                self.size = self.size / 1024; 
                self.filesize_unit = Filesize::KiB
            }
            "MiB" => {
                self.size = self.size / u64::pow(1024, 2); 
                self.filesize_unit = Filesize::MiB
            }
            "GiB" => {
                self.size = self.size / u64::pow(1024, 3); 
                self.filesize_unit = Filesize::GiB
            }
            _ => ()
        }
    }

    pub fn line(&self){
        println!("{}", self);
        println!("According line: {}", self.line);
    }

    pub fn short(&self){
        println!("{}", self);
    }

    pub fn pretty_print(&self){
        println!("----------------------------------------------");
        println!("{}", self);
        println!("According line: {}", self.line);
        println!("----------------------------------------------");
    }

    pub fn lines_number(&self){
        todo!()
    }
}