pub trait Object {
    type NativeType;
    type ReturnValue;

    fn get_raw(&self) -> *mut Self::NativeType;
    fn show(&self) -> Self::ReturnValue;
    fn free(&mut self);
}
