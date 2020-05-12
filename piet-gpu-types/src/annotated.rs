use piet_gpu_derive::piet_gpu;

piet_gpu! {
    #[gpu_write]
    mod annotated {
        struct AnnoLineSeg {
            p0: [f32; 2],
            p1: [f32; 2],
            // halfwidth in both x and y for binning
            stroke: [f32; 2],
        }
        struct AnnoQuadSeg {
            p0: [f32; 2],
            p1: [f32; 2],
            p2: [f32; 2],
            stroke: [f32; 2],
        }
        struct AnnoCubicSeg {
            p0: [f32; 2],
            p1: [f32; 2],
            p2: [f32; 2],
            p3: [f32; 2],
            stroke: [f32; 2],
        }
        struct AnnoFill {
            rgba_color: u32,
            bbox: [f32; 4],
        }
        struct AnnoStroke {
            rgba_color: u32,
            bbox: [f32; 4],
            // For the nonuniform scale case, this needs to be a 2x2 matrix.
            // That's expected to be uncommon, so we could special-case it.
            linewidth: f32,
        }
        enum Annotated {
            Nop,
            // The segments need a flag to indicate fill/stroke
            Line(AnnoLineSeg),
            Quad(AnnoQuadSeg),
            Cubic(AnnoCubicSeg),
            Stroke(AnnoStroke),
            Fill(AnnoFill),
        }
    }
}
