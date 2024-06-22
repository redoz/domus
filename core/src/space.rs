use crate::LifeCycle;

pub trait Space : LifeCycle {
    fn name(&self) -> &str;
    
    // fn tags(&self) -> &[String];
    //fn sub_spaces(&self) -> SubSpaceIterator<'_, &Self>;
//    fn devices(&self) -> &[Box<dyn Device>];   // Assuming a Device trait for all devices
}