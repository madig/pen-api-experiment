use kurbo::{Affine, Point};
use norad::Name;

use pen::PointPen;

pub mod pen;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Drawing {
    pub height_and_origin: Option<(f64, f64)>,
    pub width: f64,
    pub anchors: Vec<Anchor>,
    pub components: Vec<Component>,
    pub contours: Vec<Contour>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Anchor {
    pub pt: Point,
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub base: Name,
    pub transform: Affine,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Contour {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub pt: Point,
    pub typ: PointType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointType {
    OffCurve,
    Move,
    SmoothMove,
    Line,
    SmoothLine,
    Curve,
    SmoothCurve,
    QCurve,
    SmoothQCurve,
}

impl Drawing {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_affine(&mut self, transform: Affine) {
        for anchor in &mut self.anchors {
            anchor.apply_affine(transform);
        }
        for component in &mut self.components {
            component.apply_affine(transform);
        }
        for contour in &mut self.contours {
            contour.apply_affine(transform);
        }
    }

    pub fn add_anchor(&mut self, x: f64, y: f64, name: &str) {
        self.anchors.push(Anchor::new(x, y, name));
    }

    pub fn point_pen(&mut self) -> PointPen {
        pen::PointPen::new(self)
    }
}

impl Anchor {
    pub fn new(x: f64, y: f64, name: &str) -> Self {
        Self {
            pt: Point::new(x, y),
            name: Name::new(name).expect("Name must match UFO restrictions"),
        }
    }

    pub fn apply_affine(&mut self, transform: Affine) {
        self.pt = transform * self.pt;
    }
}

impl Component {
    pub fn new(base: &str, transform: Affine) -> Self {
        Self {
            base: Name::new(base).expect("Name must match UFO restrictions"),
            transform,
        }
    }

    pub fn apply_affine(&mut self, transform: Affine) {
        self.transform = transform * self.transform;
    }
}

impl Contour {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn from_nodes(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    pub fn apply_affine(&mut self, transform: Affine) {
        for node in &mut self.nodes {
            node.pt = transform * node.pt;
        }
    }
}

impl Node {
    pub fn new(x: f64, y: f64, typ: PointType) -> Self {
        Self {
            pt: Point::new(x, y),
            typ,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn direct() {
        let mut drawing = Drawing::new();

        drawing.anchors.push(Anchor::new(0.0, 0.0, "a"));
        drawing
            .components
            .push(Component::new("b", Affine::translate((1.0, 1.0))));
        drawing.contours.push(Contour::from_nodes(vec![
            Node::new(0.0, 0.0, PointType::Move),
            Node::new(1.0, 1.0, PointType::Line),
        ]));
        drawing.apply_affine(Affine::translate((123.0, 456.0)));

        assert_eq!(
            drawing,
            Drawing {
                height_and_origin: None,
                width: 0.0,
                anchors: vec![Anchor::new(123.0, 456.0, "a")],
                components: vec![Component::new(
                    "b",
                    Affine::new([1.0, 0.0, 0.0, 1.0, 124.0, 457.0])
                )],
                contours: vec![Contour::from_nodes(vec![
                    Node::new(123.0, 456.0, PointType::Move),
                    Node::new(124.0, 457.0, PointType::Line),
                ])]
            }
        );
    }

    #[test]
    fn pen() {
        let mut drawing = Drawing::new();

        drawing.add_anchor(0.0, 0.0, "a");
        let mut pen = drawing.point_pen();
        pen.begin_path();
        pen.add_point(0.0, 0.0, PointType::Move);
        pen.add_point(1.0, 1.0, PointType::Line);
        pen.end_path();
        pen.add_component("b", Affine::translate((1.0, 1.0)));
        drawing.apply_affine(Affine::translate((123.0, 456.0)));

        assert_eq!(
            drawing,
            Drawing {
                height_and_origin: None,
                width: 0.0,
                anchors: vec![Anchor::new(123.0, 456.0, "a")],
                components: vec![Component::new(
                    "b",
                    Affine::new([1.0, 0.0, 0.0, 1.0, 124.0, 457.0])
                )],
                contours: vec![Contour::from_nodes(vec![
                    Node::new(123.0, 456.0, PointType::Move),
                    Node::new(124.0, 457.0, PointType::Line),
                ])]
            }
        );
    }
}
