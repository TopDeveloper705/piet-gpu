use piet_gpu_derive::piet_gpu;

pub use self::scene::{
    Bbox, PietCircle, PietFill, PietItem, PietStrokeLine, PietStrokePolyLine, Point, SimpleGroup,
};

piet_gpu! {
    #[rust_encode]
    mod scene {
        struct Bbox {
            bbox: [i16; 4],
        }
        struct Point {
            xy: [f32; 2],
        }
        struct SimpleGroup {
            n_items: u32,
            // Note: both of the following items are actually arrays
            items: Ref<PietItem>,
            bboxes: Ref<Bbox>,
            offset: Point,
        }
        struct PietCircle {
            rgba_color: u32,
            center: Point,
            radius: f32,
        }
        struct PietStrokeLine {
            flags: u32,
            rgba_color: u32,
            width: f32,
            start: Point,
            end: Point,
        }
        struct PietFill {
            flags: u32,
            rgba_color: u32,
            n_points: u32,
            points: Ref<Point>,
        }
        struct PietStrokePolyLine {
            rgba_color: u32,
            width: f32,
            n_points: u32,
            points: Ref<Point>,
        }
        enum PietItem {
            Group(SimpleGroup),
            Circle(PietCircle),
            Line(PietStrokeLine),
            Fill(PietFill),
            Poly(PietStrokePolyLine),
        }
    }
}
