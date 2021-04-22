
pub fn test_in_lib(){

}

fn private_fn(){

}

pub trait Display{
    fn display(&self){
    }
}

pub trait Callable{
    fn call(&self);
}
pub struct Node{
    pub val: u32,
    pub key: String,
}

impl Callable for Node{
    fn call(&self) {
        todo!()
    }
}

impl Node{
    fn get_head(&self){

    }
}

pub trait Messanger{
    fn send(&self, msg: &str);

    fn test();
}

pub struct LimitTracker<'a, T:Messanger>{
    messanger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T> where T: Messanger{
    pub fn new(messanger: &T, max:usize) -> LimitTracker<T>{
        return LimitTracker{
            messanger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value:usize){
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;
        if percentage >= 1.0{
            self.messanger.send("Error: over quota!");
        }else if percentage >= 0.9{
            self.messanger.send("Warning");
        }else if percentage >= 0.75{
            self.messanger.send("OK")

        }
    }
}



