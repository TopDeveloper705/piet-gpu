// Copyright 2021 The piet-gpu authors.
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

//! A multiplexer module that selects a back-end at runtime.

use smallvec::SmallVec;

mux_cfg! {
    #[cfg(vk)]
    use crate::vulkan;
}
mux_cfg! {
    #[cfg(dx12)]
    use crate::dx12;
}
mux_cfg! {
    #[cfg(mtl)]
    use crate::metal;
}
use crate::backend::CmdBuf as CmdBufTrait;
use crate::backend::DescriptorSetBuilder as DescriptorSetBuilderTrait;
use crate::backend::Device as DeviceTrait;
use crate::BackendType;
use crate::BindType;
use crate::ComputePassDescriptor;
use crate::ImageFormat;
use crate::MapMode;
use crate::{BufferUsage, Error, GpuInfo, ImageLayout, InstanceFlags};

mux_enum! {
    /// An instance, selected from multiple backends.
    pub enum Instance {
        Vk(vulkan::VkInstance),
        Dx12(dx12::Dx12Instance),
        Mtl(metal::MtlInstance),
    }
}

mux_enum! {
    /// A device, selected from multiple backends.
    pub enum Device {
        Vk(vulkan::VkDevice),
        Dx12(dx12::Dx12Device),
        Mtl(metal::MtlDevice),
    }
}

mux_enum! {
    /// A surface, which can apply to one of multiple backends.
    pub enum Surface {
        Vk(vulkan::VkSurface),
        Dx12(dx12::Dx12Surface),
        Mtl(metal::MtlSurface),
    }
}

mux_enum! {
    /// A surface, which can apply to one of multiple backends.
    pub enum Swapchain {
        Vk(vulkan::VkSwapchain),
        Dx12(dx12::Dx12Swapchain),
        Mtl(metal::MtlSwapchain),
    }
}

mux_device_enum! { Buffer }
mux_device_enum! { Image }
mux_device_enum! {
/// An object for waiting on command buffer completion.
Fence }
mux_device_enum! {
/// A semaphore for swapchain presentation.
///
/// Depending on what kind of synchronization is needed for swapchain
/// presentation by the back-end, this may or may not be a "real"
/// semaphore.
Semaphore }
mux_device_enum! {
/// A pipeline object; basically a compiled shader.
Pipeline }
mux_device_enum! { DescriptorSetBuilder }
mux_device_enum! {
/// A descriptor set; a binding of resources for access by a shader.
DescriptorSet }
mux_device_enum! { CmdBuf }
mux_device_enum! {
/// An object for recording timer queries.
QueryPool }
mux_device_enum! { Sampler }

/// The code for a shader, either as source or intermediate representation.
pub enum ShaderCode<'a> {
    /// SPIR-V (binary intermediate representation)
    Spv(&'a [u8]),
    /// HLSL (source)
    Hlsl(&'a str),
    /// DXIL (DX12 intermediate language)
    Dxil(&'a [u8]),
    /// Metal Shading Language (source)
    Msl(&'a str),
}

impl Instance {
    /// Create a new GPU instance.
    ///
    /// When multiple back-end GPU APIs are available (for example, Vulkan
    /// and DX12), this function selects one at runtime.
    ///
    /// When no surface is given, the instance is suitable for compute-only
    /// work.
    pub fn new(
        flags: InstanceFlags,
    ) -> Result<Instance, Error> {
        let mut backends = [BackendType::Vulkan, BackendType::Dx12];
        if flags.contains(InstanceFlags::DX12) {
            backends.swap(0, 1);
        }
        for backend in backends {
            if backend == BackendType::Vulkan {
                mux_cfg! {
                    #[cfg(vk)]
                    {
                        if let Ok(instance) = vulkan::VkInstance::new(flags.contains(InstanceFlags::PRESENT)) {
                            return Ok(Instance::Vk(instance));
                        }
                    }
                }
            }
            if backend == BackendType::Dx12 {
                mux_cfg! {
                    #[cfg(dx12)]
                    {
                        if let Ok(instance) = dx12::Dx12Instance::new() {
                            return Ok(Instance::Dx12(instance))
                        }
                    }
                }
            }
        }
        mux_cfg! {
            #[cfg(mtl)]
            {
                if let Ok(instance) = metal::MtlInstance::new() {
                    return Ok(Instance::Mtl(instance));
                }
            }
        }
        // TODO plumb creation errors through.
        Err("No suitable instances found".into())
    }

    /// Create a surface from the specified window handle.
    pub unsafe fn surface(
        &self,
        window_handle: &dyn raw_window_handle::HasRawWindowHandle,
    ) -> Result<Surface, Error> {
        mux_match! { self;
            Instance::Vk(i) => i.surface(window_handle).map(Surface::Vk),
            Instance::Dx12(i) => i.surface(window_handle).map(Surface::Dx12),
            Instance::Mtl(i) => i.surface(window_handle).map(Surface::Mtl),
        }
    }

    /// Create a device.
    ///
    /// The "device" is the low-level GPU abstraction for creating resources
    /// and submitting work. Most users of this library will want to wrap it in
    /// a "session" which is similar but provides many conveniences.
    pub unsafe fn device(&self) -> Result<Device, Error> {
        mux_match! { self;
            Instance::Vk(i) => i.device(true).map(Device::Vk),
            Instance::Dx12(i) => i.device().map(Device::Dx12),
            Instance::Mtl(i) => i.device(surface.map(Surface::mtl)).map(Device::Mtl),
        }
    }

    /// Create a swapchain.
    ///
    /// A swapchain is a small vector of images shared with the platform's
    /// presentation logic. To actually display pixels, the application writes
    /// into the swapchain images, then calls the present method to display
    /// them.
    pub unsafe fn swapchain(
        &self,
        width: usize,
        height: usize,
        device: &Device,
        surface: &Surface,
    ) -> Result<Swapchain, Error> {
        mux_match! { self;
            Instance::Vk(i) => i
                .swapchain(width, height, device.vk(), surface.vk())
                .map(Swapchain::Vk),
            Instance::Dx12(i) => i
                .swapchain(width, height, device.dx12(), surface.dx12())
                .map(Swapchain::Dx12),
            Instance::Mtl(i) => i
                .swapchain(width, height, device.mtl(), surface.mtl())
                .map(Swapchain::Mtl),
        }
    }
}

// This is basically re-exporting the backend device trait, and we could do that,
// but not doing so lets us diverge more easily (at the moment, the divergence is
// missing functionality).
impl Device {
    #[cfg(target_os = "macos")]
    pub fn new_from_raw_mtl(
        device: &::metal::DeviceRef,
        queue: &::metal::CommandQueueRef,
    ) -> Device {
        Device::Mtl(metal::MtlDevice::new_from_raw_mtl(
            device.to_owned(),
            queue.to_owned(),
        ))
    }

    #[cfg(target_os = "macos")]
    pub fn cmd_buf_from_raw_mtl(&self, raw_cmd_buf: &::metal::CommandBufferRef) -> CmdBuf {
        // Note: this will cause problems if we support multiple back-ends on mac. But it will
        // be a compile error;
        let Device::Mtl(d) = self;
        CmdBuf::Mtl(d.cmd_buf_from_raw_mtl(raw_cmd_buf.to_owned()))
    }

    #[cfg(target_os = "macos")]
    pub fn image_from_raw_mtl(
        &self,
        raw_texture: &::metal::TextureRef,
        width: u32,
        height: u32,
    ) -> Image {
        // Note: this will cause problems if we support multiple back-ends on mac. But it will
        // be a compile error;
        let Device::Mtl(d) = self;
        Image::Mtl(d.image_from_raw_mtl(raw_texture.to_owned(), width, height))
    }

    pub fn query_gpu_info(&self) -> GpuInfo {
        mux_match! { self;
            Device::Vk(d) => d.query_gpu_info(),
            Device::Dx12(d) => d.query_gpu_info(),
            Device::Mtl(d) => d.query_gpu_info(),
        }
    }

    pub fn create_buffer(&self, size: u64, usage: BufferUsage) -> Result<Buffer, Error> {
        mux_match! { self;
            Device::Vk(d) => d.create_buffer(size, usage).map(Buffer::Vk),
            Device::Dx12(d) => d.create_buffer(size, usage).map(Buffer::Dx12),
            Device::Mtl(d) => d.create_buffer(size, usage).map(Buffer::Mtl),
        }
    }

    pub unsafe fn destroy_buffer(&self, buffer: &Buffer) -> Result<(), Error> {
        mux_match! { self;
            Device::Vk(d) => d.destroy_buffer(buffer.vk()),
            Device::Dx12(d) => d.destroy_buffer(buffer.dx12()),
            Device::Mtl(d) => d.destroy_buffer(buffer.mtl()),
        }
    }

    pub unsafe fn create_image2d(
        &self,
        width: u32,
        height: u32,
        format: ImageFormat,
    ) -> Result<Image, Error> {
        mux_match! { self;
            Device::Vk(d) => d.create_image2d(width, height, format).map(Image::Vk),
            Device::Dx12(d) => d.create_image2d(width, height, format).map(Image::Dx12),
            Device::Mtl(d) => d.create_image2d(width, height, format).map(Image::Mtl),
        }
    }

    pub unsafe fn destroy_image(&self, image: &Image) -> Result<(), Error> {
        mux_match! { self;
            Device::Vk(d) => d.destroy_image(image.vk()),
            Device::Dx12(d) => d.destroy_image(image.dx12()),
            Device::Mtl(d) => d.destroy_image(image.mtl()),
        }
    }

    pub unsafe fn create_fence(&self, signaled: bool) -> Result<Fence, Error> {
        mux_match! { self;
            Device::Vk(d) => d.create_fence(signaled).map(Fence::Vk),
            Device::Dx12(d) => d.create_fence(signaled).map(Fence::Dx12),
            Device::Mtl(d) => d.create_fence(signaled).map(Fence::Mtl),
        }
    }

    pub unsafe fn destroy_fence(&self, fence: Fence) -> Result<(), Error> {
        mux_match! { self;
            Device::Vk(d) => d.destroy_fence(fence.vk_owned()),
            Device::Dx12(d) => d.destroy_fence(fence.dx12_owned()),
            Device::Mtl(d) => d.destroy_fence(fence.mtl_owned()),
        }
    }

    // Consider changing Vec to iterator (as is done in gfx-hal)
    pub unsafe fn wait_and_reset(&self, fences: Vec<&mut Fence>) -> Result<(), Error> {
        mux_match! { self;
            Device::Vk(d) => {
                let fences = fences
                    .into_iter()
                    .map(|f| f.vk_mut())
                    .collect::<Vec<_>>();
                d.wait_and_reset(fences)
            }
            Device::Dx12(d) => {
                let fences = fences
                    .into_iter()
                    .map(|f| f.dx12_mut())
                    .collect::<Vec<_>>();
                d.wait_and_reset(fences)
            }
            Device::Mtl(d) => {
                let fences = fences
                    .into_iter()
                    .map(|f| f.mtl_mut())
                    .collect::<Vec<_>>();
                d.wait_and_reset(fences)
            }
        }
    }

    pub unsafe fn get_fence_status(&self, fence: &mut Fence) -> Result<bool, Error> {
        mux_match! { self;
            Device::Vk(d) => d.get_fence_status(fence.vk_mut()),
            Device::Dx12(d) => d.get_fence_status(fence.dx12_mut()),
            Device::Mtl(d) => d.get_fence_status(fence.mtl_mut()),
        }
    }

    pub unsafe fn create_semaphore(&self) -> Result<Semaphore, Error> {
        mux_match! { self;
            Device::Vk(d) => d.create_semaphore().map(Semaphore::Vk),
            Device::Dx12(d) => d.create_semaphore().map(Semaphore::Dx12),
            Device::Mtl(d) => d.create_semaphore().map(Semaphore::Mtl),
        }
    }

    pub unsafe fn create_compute_pipeline<'a>(
        &self,
        code: ShaderCode<'a>,
        bind_types: &[BindType],
    ) -> Result<Pipeline, Error> {
        mux_match! { self;
            Device::Vk(d) => {
                let shader_code = match code {
                    ShaderCode::Spv(spv) => spv,
                    // Panic or return "incompatible shader" error here?
                    _ => panic!("Vulkan backend requires shader code in SPIR-V format"),
                };
                d.create_compute_pipeline(shader_code, bind_types)
                    .map(Pipeline::Vk)
            }
            Device::Dx12(d) => {
                let shader_code = match code {
                    //ShaderCode::Hlsl(hlsl) => hlsl,
                    ShaderCode::Dxil(dxil) => dxil,
                    // Panic or return "incompatible shader" error here?
                    _ => panic!("DX12 backend requires shader code in DXIL format"),
                };
                d.create_compute_pipeline(shader_code, bind_types)
                    .map(Pipeline::Dx12)
            }
            Device::Mtl(d) => {
                let shader_code = match code {
                    ShaderCode::Msl(msl) => msl,
                    // Panic or return "incompatible shader" error here?
                    _ => panic!("Metal backend requires shader code in MSL format"),
                };
                d.create_compute_pipeline(shader_code, bind_types)
                    .map(Pipeline::Mtl)
            }
        }
    }

    pub unsafe fn descriptor_set_builder(&self) -> DescriptorSetBuilder {
        mux_match! { self;
            Device::Vk(d) => DescriptorSetBuilder::Vk(d.descriptor_set_builder()),
            Device::Dx12(d) => DescriptorSetBuilder::Dx12(d.descriptor_set_builder()),
            Device::Mtl(d) => DescriptorSetBuilder::Mtl(d.descriptor_set_builder()),
        }
    }

    pub unsafe fn update_buffer_descriptor(
        &self,
        ds: &mut DescriptorSet,
        index: u32,
        buffer: &Buffer,
    ) {
        mux_match! { self;
            Device::Vk(d) => d.update_buffer_descriptor(ds.vk_mut(), index, buffer.vk()),
            Device::Dx12(d) => d.update_buffer_descriptor(ds.dx12_mut(), index, buffer.dx12()),
            Device::Mtl(d) => d.update_buffer_descriptor(ds.mtl_mut(), index, buffer.mtl()),
        }
    }

    pub unsafe fn update_image_descriptor(
        &self,
        ds: &mut DescriptorSet,
        index: u32,
        image: &Image,
    ) {
        mux_match! { self;
            Device::Vk(d) => d.update_image_descriptor(ds.vk_mut(), index, image.vk()),
            Device::Dx12(d) => d.update_image_descriptor(ds.dx12_mut(), index, image.dx12()),
            Device::Mtl(d) => d.update_image_descriptor(ds.mtl_mut(), index, image.mtl()),
        }
    }

    pub fn create_cmd_buf(&self) -> Result<CmdBuf, Error> {
        mux_match! { self;
            Device::Vk(d) => d.create_cmd_buf().map(CmdBuf::Vk),
            Device::Dx12(d) => d.create_cmd_buf().map(CmdBuf::Dx12),
            Device::Mtl(d) => d.create_cmd_buf().map(CmdBuf::Mtl),
        }
    }

    pub unsafe fn destroy_cmd_buf(&self, cmd_buf: CmdBuf) -> Result<(), Error> {
        mux_match! { self;
            Device::Vk(d) => d.destroy_cmd_buf(cmd_buf.vk_owned()),
            Device::Dx12(d) => d.destroy_cmd_buf(cmd_buf.dx12_owned()),
            Device::Mtl(d) => d.destroy_cmd_buf(cmd_buf.mtl_owned()),
        }
    }

    pub fn create_query_pool(&self, n_queries: u32) -> Result<QueryPool, Error> {
        mux_match! { self;
            Device::Vk(d) => d.create_query_pool(n_queries).map(QueryPool::Vk),
            Device::Dx12(d) => d.create_query_pool(n_queries).map(QueryPool::Dx12),
            Device::Mtl(d) => d.create_query_pool(n_queries).map(QueryPool::Mtl),
        }
    }

    pub unsafe fn fetch_query_pool(&self, pool: &QueryPool) -> Result<Vec<f64>, Error> {
        mux_match! { self;
            Device::Vk(d) => d.fetch_query_pool(pool.vk()),
            Device::Dx12(d) => d.fetch_query_pool(pool.dx12()),
            Device::Mtl(d) => d.fetch_query_pool(pool.mtl()),
        }
    }

    pub unsafe fn run_cmd_bufs(
        &self,
        cmd_bufs: &[&CmdBuf],
        wait_semaphores: &[&Semaphore],
        signal_semaphores: &[&Semaphore],
        fence: Option<&mut Fence>,
    ) -> Result<(), Error> {
        mux_match! { self;
            Device::Vk(d) => d.run_cmd_bufs(
                &cmd_bufs
                    .iter()
                    .map(|c| c.vk())
                    .collect::<SmallVec<[_; 4]>>(),
                &wait_semaphores
                    .iter()
                    .copied()
                    .map(Semaphore::vk)
                    .collect::<SmallVec<[_; 4]>>(),
                &signal_semaphores
                    .iter()
                    .copied()
                    .map(Semaphore::vk)
                    .collect::<SmallVec<[_; 4]>>(),
                fence.map(Fence::vk_mut),
            ),
            Device::Dx12(d) => d.run_cmd_bufs(
                &cmd_bufs
                    .iter()
                    .map(|c| c.dx12())
                    .collect::<SmallVec<[_; 4]>>(),
                &wait_semaphores
                    .iter()
                    .copied()
                    .map(Semaphore::dx12)
                    .collect::<SmallVec<[_; 4]>>(),
                &signal_semaphores
                    .iter()
                    .copied()
                    .map(Semaphore::dx12)
                    .collect::<SmallVec<[_; 4]>>(),
                fence.map(Fence::dx12_mut),
            ),
            Device::Mtl(d) => d.run_cmd_bufs(
                &cmd_bufs
                    .iter()
                    .map(|c| c.mtl())
                    .collect::<SmallVec<[_; 4]>>(),
                &wait_semaphores
                    .iter()
                    .copied()
                    .map(Semaphore::mtl)
                    .collect::<SmallVec<[_; 4]>>(),
                &signal_semaphores
                    .iter()
                    .copied()
                    .map(Semaphore::mtl)
                    .collect::<SmallVec<[_; 4]>>(),
                fence.map(Fence::mtl_mut),
            ),
        }
    }

    pub unsafe fn map_buffer(
        &self,
        buffer: &Buffer,
        offset: u64,
        size: u64,
        mode: MapMode,
    ) -> Result<*mut u8, Error> {
        mux_match! { self;
            Device::Vk(d) => d.map_buffer(buffer.vk(), offset, size, mode),
            Device::Dx12(d) => d.map_buffer(buffer.dx12(), offset, size, mode),
            Device::Mtl(d) => d.map_buffer(buffer.mtl(), offset, size, mode),
        }
    }

    pub unsafe fn unmap_buffer(
        &self,
        buffer: &Buffer,
        offset: u64,
        size: u64,
        mode: MapMode,
    ) -> Result<(), Error> {
        mux_match! { self;
            Device::Vk(d) => d.unmap_buffer(buffer.vk(), offset, size, mode),
            Device::Dx12(d) => d.unmap_buffer(buffer.dx12(), offset, size, mode),
            Device::Mtl(d) => d.unmap_buffer(buffer.mtl(), offset, size, mode),
        }
    }

    /// Choose shader code from the available choices.
    pub fn choose_shader<'a>(
        &self,
        _spv: &'a [u8],
        _hlsl: &'a str,
        _dxil: &'a [u8],
        _msl: &'a str,
    ) -> ShaderCode<'a> {
        mux_match! { self;
            Device::Vk(_d) => ShaderCode::Spv(_spv),
            Device::Dx12(_d) => ShaderCode::Dxil(_dxil),
            Device::Mtl(_d) => ShaderCode::Msl(_msl),
        }
    }

    pub fn backend_type(&self) -> BackendType {
        mux_match! { self;
            Device::Vk(_d) => BackendType::Vulkan,
            Device::Dx12(_d) => BackendType::Dx12,
            Device::Mtl(_d) => BackendType::Metal,
        }
    }
}

impl DescriptorSetBuilder {
    pub fn add_buffers(&mut self, buffers: &[&Buffer]) {
        mux_match! { self;
            DescriptorSetBuilder::Vk(x) => x.add_buffers(
                &buffers
                    .iter()
                    .copied()
                    .map(Buffer::vk)
                    .collect::<SmallVec<[_; 8]>>(),
            ),
            DescriptorSetBuilder::Dx12(x) => x.add_buffers(
                &buffers
                    .iter()
                    .copied()
                    .map(Buffer::dx12)
                    .collect::<SmallVec<[_; 8]>>(),
            ),
            DescriptorSetBuilder::Mtl(x) => x.add_buffers(
                &buffers
                    .iter()
                    .copied()
                    .map(Buffer::mtl)
                    .collect::<SmallVec<[_; 8]>>(),
            ),
        }
    }

    pub fn add_images(&mut self, images: &[&Image]) {
        mux_match! { self;
            DescriptorSetBuilder::Vk(x) => x.add_images(
                &images
                    .iter()
                    .copied()
                    .map(Image::vk)
                    .collect::<SmallVec<[_; 8]>>(),
            ),
            DescriptorSetBuilder::Dx12(x) => x.add_images(
                &images
                    .iter()
                    .copied()
                    .map(Image::dx12)
                    .collect::<SmallVec<[_; 8]>>(),
            ),
            DescriptorSetBuilder::Mtl(x) => x.add_images(
                &images
                    .iter()
                    .copied()
                    .map(Image::mtl)
                    .collect::<SmallVec<[_; 8]>>(),
            ),
        }
    }

    pub fn add_textures(&mut self, images: &[&Image]) {
        mux_match! { self;
            DescriptorSetBuilder::Vk(x) => x.add_textures(
                &images
                    .iter()
                    .copied()
                    .map(Image::vk)
                    .collect::<SmallVec<[_; 8]>>(),
            ),
            DescriptorSetBuilder::Dx12(x) => x.add_textures(
                &images
                    .iter()
                    .copied()
                    .map(Image::dx12)
                    .collect::<SmallVec<[_; 8]>>(),
            ),
            DescriptorSetBuilder::Mtl(x) => x.add_textures(
                &images
                    .iter()
                    .copied()
                    .map(Image::mtl)
                    .collect::<SmallVec<[_; 8]>>(),
            ),
        }
    }

    pub unsafe fn build(
        self,
        device: &Device,
        pipeline: &Pipeline,
    ) -> Result<DescriptorSet, Error> {
        mux_match! { self;
            DescriptorSetBuilder::Vk(x) =>
                x.build(device.vk(), pipeline.vk()).map(DescriptorSet::Vk),
            DescriptorSetBuilder::Dx12(x) => x
                .build(device.dx12(), pipeline.dx12())
                .map(DescriptorSet::Dx12),
            DescriptorSetBuilder::Mtl(x) => x
                .build(device.mtl(), pipeline.mtl())
                .map(DescriptorSet::Mtl),
        }
    }
}

impl CmdBuf {
    pub unsafe fn begin(&mut self) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.begin(),
            CmdBuf::Dx12(c) => c.begin(),
            CmdBuf::Mtl(c) => c.begin(),
        }
    }

    pub unsafe fn finish(&mut self) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.finish(),
            CmdBuf::Dx12(c) => c.finish(),
            CmdBuf::Mtl(c) => c.finish(),
        }
    }

    pub unsafe fn reset(&mut self) -> bool {
        mux_match! { self;
            CmdBuf::Vk(c) => c.reset(),
            CmdBuf::Dx12(c) => c.reset(),
            CmdBuf::Mtl(c) => c.reset(),
        }
    }

    pub unsafe fn begin_compute_pass(&mut self, desc: &ComputePassDescriptor) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.begin_compute_pass(desc),
            CmdBuf::Dx12(c) => c.begin_compute_pass(desc),
            CmdBuf::Mtl(c) => c.begin_compute_pass(desc),
        }
    }

    /// Dispatch a compute shader.
    ///
    /// Note that both the number of workgroups (`workgroup_count`) and the number of
    /// threads in a workgroup (`workgroup_size`) are given. The latter is needed on
    /// Metal, while it's baked into the shader on Vulkan and DX12.
    ///
    /// Perhaps we'll have a mechanism to plumb the latter value to configure the size
    /// of a workgroup using specialization constants in the future.
    pub unsafe fn dispatch(
        &mut self,
        pipeline: &Pipeline,
        descriptor_set: &DescriptorSet,
        workgroup_count: (u32, u32, u32),
        workgroup_size: (u32, u32, u32),
    ) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.dispatch(pipeline.vk(), descriptor_set.vk(), workgroup_count, workgroup_size),
            CmdBuf::Dx12(c) => c.dispatch(pipeline.dx12(), descriptor_set.dx12(), workgroup_count, workgroup_size),
            CmdBuf::Mtl(c) => c.dispatch(pipeline.mtl(), descriptor_set.mtl(), workgroup_count, workgroup_size),
        }
    }

    pub unsafe fn end_compute_pass(&mut self) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.end_compute_pass(),
            CmdBuf::Dx12(c) => c.end_compute_pass(),
            CmdBuf::Mtl(c) => c.end_compute_pass(),
        }
    }

    pub unsafe fn memory_barrier(&mut self) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.memory_barrier(),
            CmdBuf::Dx12(c) => c.memory_barrier(),
            CmdBuf::Mtl(c) => c.memory_barrier(),
        }
    }

    pub unsafe fn host_barrier(&mut self) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.host_barrier(),
            CmdBuf::Dx12(c) => c.host_barrier(),
            CmdBuf::Mtl(c) => c.host_barrier(),
        }
    }

    pub unsafe fn image_barrier(
        &mut self,
        image: &Image,
        src_layout: ImageLayout,
        dst_layout: ImageLayout,
    ) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.image_barrier(image.vk(), src_layout, dst_layout),
            CmdBuf::Dx12(c) => c.image_barrier(image.dx12(), src_layout, dst_layout),
            CmdBuf::Mtl(c) => c.image_barrier(image.mtl(), src_layout, dst_layout),
        }
    }

    pub unsafe fn clear_buffer(&mut self, buffer: &Buffer, size: Option<u64>) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.clear_buffer(buffer.vk(), size),
            CmdBuf::Dx12(c) => c.clear_buffer(buffer.dx12(), size),
            CmdBuf::Mtl(c) => c.clear_buffer(buffer.mtl(), size),
        }
    }

    pub unsafe fn copy_buffer(&mut self, src: &Buffer, dst: &Buffer) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.copy_buffer(src.vk(), dst.vk()),
            CmdBuf::Dx12(c) => c.copy_buffer(src.dx12(), dst.dx12()),
            CmdBuf::Mtl(c) => c.copy_buffer(src.mtl(), dst.mtl()),
        }
    }

    pub unsafe fn copy_image_to_buffer(&mut self, src: &Image, dst: &Buffer) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.copy_image_to_buffer(src.vk(), dst.vk()),
            CmdBuf::Dx12(c) => c.copy_image_to_buffer(src.dx12(), dst.dx12()),
            CmdBuf::Mtl(c) => c.copy_image_to_buffer(src.mtl(), dst.mtl()),
        }
    }

    pub unsafe fn copy_buffer_to_image(&mut self, src: &Buffer, dst: &Image) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.copy_buffer_to_image(src.vk(), dst.vk()),
            CmdBuf::Dx12(c) => c.copy_buffer_to_image(src.dx12(), dst.dx12()),
            CmdBuf::Mtl(c) => c.copy_buffer_to_image(src.mtl(), dst.mtl()),
        }
    }

    pub unsafe fn blit_image(&mut self, src: &Image, dst: &Image) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.blit_image(src.vk(), dst.vk()),
            CmdBuf::Dx12(c) => c.blit_image(src.dx12(), dst.dx12()),
            CmdBuf::Mtl(c) => c.blit_image(src.mtl(), dst.mtl()),
        }
    }

    pub unsafe fn reset_query_pool(&mut self, pool: &QueryPool) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.reset_query_pool(pool.vk()),
            CmdBuf::Dx12(c) => c.reset_query_pool(pool.dx12()),
            CmdBuf::Mtl(c) => c.reset_query_pool(pool.mtl()),
        }
    }

    pub unsafe fn write_timestamp(&mut self, pool: &QueryPool, query: u32) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.write_timestamp(pool.vk(), query),
            CmdBuf::Dx12(c) => c.write_timestamp(pool.dx12(), query),
            CmdBuf::Mtl(c) => c.write_timestamp(pool.mtl(), query),
        }
    }

    pub unsafe fn finish_timestamps(&mut self, pool: &QueryPool) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.finish_timestamps(pool.vk()),
            CmdBuf::Dx12(c) => c.finish_timestamps(pool.dx12()),
            CmdBuf::Mtl(c) => c.finish_timestamps(pool.mtl()),
        }
    }

    pub unsafe fn begin_debug_label(&mut self, label: &str) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.begin_debug_label(label),
            CmdBuf::Dx12(c) => c.begin_debug_label(label),
            CmdBuf::Mtl(c) => c.begin_debug_label(label),
        }
    }

    pub unsafe fn end_debug_label(&mut self) {
        mux_match! { self;
            CmdBuf::Vk(c) => c.end_debug_label(),
            CmdBuf::Dx12(c) => c.end_debug_label(),
            CmdBuf::Mtl(c) => c.end_debug_label(),
        }
    }
}

impl Buffer {
    pub fn size(&self) -> u64 {
        mux_match! { self;
            Buffer::Vk(b) => b.size,
            Buffer::Dx12(b) => b.size,
            Buffer::Mtl(b) => b.size,
        }
    }
}

impl Swapchain {
    pub unsafe fn next(&mut self) -> Result<(usize, Semaphore), Error> {
        mux_match! { self;
            Swapchain::Vk(s) => {
                let (idx, sem) = s.next()?;
                Ok((idx, Semaphore::Vk(sem)))
            }
            Swapchain::Dx12(s) => {
                let (idx, sem) = s.next()?;
                Ok((idx, Semaphore::Dx12(sem)))
            }
            Swapchain::Mtl(s) => {
                let (idx, sem) = s.next()?;
                Ok((idx, Semaphore::Mtl(sem)))
            }
        }
    }

    pub unsafe fn image(&self, idx: usize) -> crate::Image {
        crate::Image::wrap_swapchain_image(self.image_raw(idx))
    }

    pub unsafe fn image_raw(&self, idx: usize) -> Image {
        mux_match! { self;
            Swapchain::Vk(s) => Image::Vk(s.image(idx)),
            Swapchain::Dx12(s) => Image::Dx12(s.image(idx)),
            Swapchain::Mtl(s) => Image::Mtl(s.image(idx)),
        }
    }

    pub unsafe fn present(
        &self,
        image_idx: usize,
        semaphores: &[&Semaphore],
    ) -> Result<bool, Error> {
        mux_match! { self;
            Swapchain::Vk(s) => s.present(
                image_idx,
                &semaphores
                    .iter()
                    .copied()
                    .map(Semaphore::vk)
                    .collect::<SmallVec<[_; 4]>>(),
            ),
            Swapchain::Dx12(s) => s.present(
                image_idx,
                &semaphores
                    .iter()
                    .copied()
                    .map(Semaphore::dx12)
                    .collect::<SmallVec<[_; 4]>>(),
            ),
            Swapchain::Mtl(s) => s.present(
                image_idx,
                &semaphores
                    .iter()
                    .copied()
                    .map(Semaphore::mtl)
                    .collect::<SmallVec<[_; 4]>>(),
            ),
        }
    }
}
