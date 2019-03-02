use amethyst::{core::bundle::SystemBundle, ecs::prelude::DispatcherBuilder, core::Error};

pub struct AntSimBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for AntSimBundle {
    fn build(self, _builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        Ok(())
    }
}