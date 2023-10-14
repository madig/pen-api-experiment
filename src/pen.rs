use kurbo::Affine;

use super::{Component, Contour, Drawing, Node, PointType};

#[derive(Debug)]
pub struct PointPen<'a> {
    current_contour: Option<Contour>,
    drawing: &'a mut Drawing,
}

impl<'a> PointPen<'a> {
    pub fn new(drawing: &'a mut Drawing) -> Self {
        Self {
            current_contour: None,
            drawing,
        }
    }

    pub fn begin_path(&mut self) {
        assert_eq!(self.current_contour, None);
        self.current_contour = Some(Contour::new());
    }

    pub fn end_path(&mut self) {
        assert!(self.current_contour.is_some());
        self.drawing
            .contours
            .push(self.current_contour.take().unwrap());
    }

    pub fn add_point(&mut self, x: f64, y: f64, segment_type: PointType) {
        assert!(self.current_contour.is_some());
        self.current_contour
            .as_mut()
            .unwrap()
            .nodes
            .push(Node::new(x, y, segment_type));
    }

    pub fn add_component(&mut self, glyph_name: &str, transform: Affine) {
        self.drawing
            .components
            .push(Component::new(glyph_name, transform));
    }
}
