// Copyright 2022 The piet-gpu authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Also licensed under MIT license, at your choice.

mod blend;
mod builder;
mod style;

pub use blend::{Blend, Compose, Mix};
pub use builder::{build_fragment, build_scene, Builder};
pub use style::*;

use super::brush::*;
use super::geometry::{Affine, Rect};
use super::path::Element;
use core::ops::Range;

#[derive(Default)]
struct SceneData {
    transform_stream: Vec<Affine>,
    tag_stream: Vec<u8>,
    pathseg_stream: Vec<u8>,
    linewidth_stream: Vec<f32>,
    drawtag_stream: Vec<u32>,
    drawdata_stream: Vec<u8>,
    n_path: u32,
    n_pathseg: u32,
    n_clip: u32,
}

impl SceneData {
    fn clear(&mut self) {
        self.transform_stream.clear();
        self.tag_stream.clear();
        self.pathseg_stream.clear();
        self.linewidth_stream.clear();
        self.drawtag_stream.clear();
        self.drawdata_stream.clear();
        self.n_path = 0;
        self.n_pathseg = 0;
        self.n_clip = 0;
    }

    fn append(&mut self, other: &SceneData) {
        self.transform_stream
            .extend_from_slice(&other.transform_stream);
        self.tag_stream.extend_from_slice(&other.tag_stream);
        self.pathseg_stream.extend_from_slice(&other.pathseg_stream);
        self.linewidth_stream
            .extend_from_slice(&other.linewidth_stream);
        self.drawtag_stream.extend_from_slice(&other.drawtag_stream);
        self.drawdata_stream
            .extend_from_slice(&other.drawdata_stream);
        self.n_path += other.n_path;
        self.n_pathseg += other.n_pathseg;
        self.n_clip += other.n_clip;
    }
}

/// Encoded definition of a scene that is ready for rendering when paired with
/// an associated resource context.
#[derive(Default)]
pub struct Scene {
    data: SceneData,
}

/// Encoded definition of a scene fragment and associated resources.
#[derive(Default)]
pub struct Fragment {
    data: SceneData,
    resources: FragmentResources,
}

#[derive(Default)]
struct FragmentResources {
    patches: Vec<ResourcePatch>,
    stops: Vec<Stop>,
}

enum ResourcePatch {
    Ramp {
        drawdata_offset: usize,
        stops: Range<usize>,
    },
}
