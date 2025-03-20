pub trait GlBuffer {
    fn gen_id(&mut self);
    fn del_id(&mut self);
    fn bind(&self);
    fn unbind(&self);
}
