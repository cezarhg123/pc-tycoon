/// Bunch of generic functions for PC Parts
pub trait PCPart: Default {
    fn name(&self) -> &str;
    fn dev_name(&self) -> &str;

    fn random(seed: u64) -> Self
        where Self: Sized;
    fn save(&self);
    fn load(path: String) -> Option<Self>
        where Self: Sized;
}
