pub mod canvas;
pub mod resize;

pub use canvas::Canvas;

use crate::chain::ProcessableImage;
pub trait TransformProcessor {
    fn execute(&mut self, processable_image: &mut ProcessableImage) {
        self.handle(processable_image);

        if let Some(next) = &mut self.next() {
            next.execute(processable_image);
        }
    }

    fn handle(&mut self, processable_image: &mut ProcessableImage);
    fn next(&mut self,) -> &mut Option<Box<dyn TransformProcessor>>;
}

pub fn into_next(transform_processor: impl TransformProcessor + Sized + 'static) -> Option<Box<dyn TransformProcessor>> {
    Some(Box::new(transform_processor))
}