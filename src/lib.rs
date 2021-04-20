
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



