use piet_gpu_derive::piet_gpu;

piet_gpu! {
    #[gpu_write]
    mod ptcl {
        struct CmdStroke {
            // This is really a Ref<Tile>, but we don't have cross-module
            // references.
            tile_ref: u32,
            half_width: f32,
        }
        struct CmdFill {
            // As above, really Ref<Tile>
            tile_ref: u32,
            backdrop: i32,
        }
        struct CmdColor {
            rgba_color: u32,
        }
        struct CmdLinGrad {
            index: u32,
            // line equation for gradient
            line_x: f32,
            line_y: f32,
            line_c: f32,
        }
        struct CmdRadGrad {
            index: u32,
            mat: [f32; 4],
            xlat: [f32; 2],
            c1: [f32; 2],
            ra: f32,
            roff: f32,
        }
        struct CmdImage {
            index: u32,
            offset: [i16; 2],
        }
        struct CmdAlpha {
            alpha: f32,
        }
        struct CmdEndClip {
            blend: u32,
        }
        struct CmdJump {
            new_ref: u32,
        }
        enum Cmd {
            End,
            Fill(CmdFill),
            Stroke(CmdStroke),
            Solid,
            Alpha(CmdAlpha),
            Color(CmdColor),
            LinGrad(CmdLinGrad),
            RadGrad(CmdRadGrad),
            Image(CmdImage),
            BeginClip,
            EndClip(CmdEndClip),
            Jump(CmdJump),
        }
    }
}
