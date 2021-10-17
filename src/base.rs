// basic traits and types implemented by the structs.

trait Process {
    pub fn new() -> Self;

    pub fn generate(self);

    pub fn generate_single(self);

    pub fn generate_more(self);

    pub fn reset(self);

    fn push_back(self);

    fn pop_back(self);
}