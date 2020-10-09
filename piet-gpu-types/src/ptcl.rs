use piet_gpu_derive::piet_gpu;

piet_gpu! {
    #[gpu_write]
    mod ptcl {
        struct CmdCircle {
            center: [f32; 2],
            radius: f32,
            rgba_color: u32,
        }
        struct CmdLine {
            start: [f32; 2],
            end: [f32; 2],
        }
        struct CmdStroke {
            // This is really a Ref<Tile>, but we don't have cross-module
            // references.
            tile_ref: u32,
            half_width: f32,
            rgba_color: u32,
        }
        struct CmdFill {
            // As above, really Ref<Tile>
            tile_ref: u32,
            backdrop: i32,
            rgba_color: u32,
        }
        struct CmdFillMask {
            tile_ref: u32,
            backdrop: i32,
            mask: f32,
        }
        struct CmdSolid {
            rgba_color: u32,
        }
        struct CmdSolidMask {
            mask: f32,
        }
        struct CmdJump {
            new_ref: u32,
        }
        enum Cmd {
            End,
            Circle(CmdCircle),
            Line(CmdLine),
            Fill(CmdFill),
            FillMask(CmdFillMask),
            FillMaskInv(CmdFillMask),
            Stroke(CmdStroke),
            Solid(CmdSolid),
            SolidMask(CmdSolidMask),
            Jump(CmdJump),
        }

        // TODO: strongly consider using f16. If so, these would be
        // relative to the tile. We're doing f32 for now to minimize
        // divergence from piet-metal originals.
        struct Segment {
            start: [f32; 2],
            end: [f32; 2],

            // This is used for fills only, but we're including it in
            // the general structure for simplicity.
            y_edge: f32,
        }

        struct SegChunk {
            n: u32,
            next: Ref<SegChunk>,
            // Actually a reference to a variable-sized slice.
            segs: Ref<Segment>,
        }
    }
}
