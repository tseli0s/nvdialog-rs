/// A trait implemented for most types that stem from NvDialog's API.
/// 
/// This trait is used for a few reasons:
/// - It allows various kinds of dialogs to be grouped together and used in a generic way.
/// - It allows for a unified way to show and free the underlying object.
/// - It provides access to the internal object pointer without duplicating code unnecessarily.
/// 
/// Whereas previous versions of the crate relied on manually mapping each nvdialog function to a Rust one,
/// this trait allows nvdialog types that share common functionality to be grouped together. In addition, it will
/// allow developers to create generic types and `dyn` objects:
/// ```rust
/// use nvdialog_rs::Object;
/// 
/// fn push_dialog<T: nvdialog_rs::Object>(vec: Vec<*mut c_void>, dialog: T) {
///     vec.push(dialog.get_raw());
/// }
/// ```
/// # Safety
/// 
/// The `Object` trait is designed to provide safe access to the underlying native object in the 
/// `NvDialog` API. However, as it deals with raw pointers, it requires the user to ensure 
/// safety by adhering to the following guidelines:
/// 
/// 1. **Mutability**: The pointer returned by [`Object::get_raw`] should not be mutated if it's going to be used
///    in a subsequent call to this crate or [`nvdialog-sys`]. Mutating the pointer's contents will cause undefined behavior.
/// 
/// 2. **Object Ownership**: The ownership of the native object is managed externally by the 
///    underlying `NvDialog` API. Implementors of this trait must ensure that the object is not 
///    freed or modified while it is still in use by the `Object` trait methods. Freeing the 
///    object before calling `free` or modifying it after `free` has been called will result in 
///    undefined behavior.
/// 
/// 3. **Calling `free` Safely**: It is not adviced to manually call `free`. The reason is that most of the time,
///    the underlying object is owned by the `NvDialog` API and calling `free` will cause undefined behavior. In addition
///    the crate provides `Drop` implementations that will automatically free the object when it goes out of scope.
/// 

pub trait Object {
    type NativeType;
    type ReturnValue;

    fn get_raw(&self) -> *mut Self::NativeType;
    fn show(&self) -> Self::ReturnValue;
    fn free(&mut self);
}
