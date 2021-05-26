/// The cross-platform abstraction for a GPU device.
///
/// This abstraction is inspired by gfx-hal, but is specialized to the needs of piet-gpu.
/// In time, it may go away and be replaced by either gfx-hal or wgpu.
use bitflags::bitflags;

pub mod hub;

#[macro_use]
mod macros;

// TODO make this not pub
pub mod mux;

mux! {
    #[cfg(vk)]
    pub mod vulkan;
}
mux! {
    #[cfg(dx12)]
    pub mod dx12;
}

/// This isn't great but is expedient.
pub type Error = Box<dyn std::error::Error>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageLayout {
    Undefined,
    Present,
    BlitSrc,
    BlitDst,
    General,
    ShaderRead,
}

/// The type of sampling for image lookup.
///
/// This could take a lot more params, such as filtering, repeat, behavior
/// at edges, etc., but for now we'll keep it simple.
#[derive(Copy, Clone, Debug)]
pub enum SamplerParams {
    Nearest,
    Linear,
}

bitflags! {
    /// The intended usage for this buffer.
    pub struct BufferUsage: u32 {
        /// The buffer can be mapped for reading CPU-side.
        const MAP_READ = 0x1;
        /// The buffer can be mapped for writing CPU-side.
        const MAP_WRITE = 0x2;
        /// The buffer can be copied from.
        const COPY_SRC = 0x4;
        /// The buffer can be copied to.
        const COPY_DST = 0x8;
        /// The buffer can be bound to a compute shader.
        const STORAGE = 0x80;
        /// The buffer can be used to store the results of queries.
        const QUERY_RESOLVE = 0x200;
        // May add other types.
    }
}

#[derive(Clone, Debug)]
/// Information about the GPU.
pub struct GpuInfo {
    /// The GPU supports descriptor indexing.
    pub has_descriptor_indexing: bool,
    /// The GPU supports subgroups.
    ///
    /// Right now, this just checks for basic subgroup capability (as
    /// required in Vulkan 1.1), and we should have finer grained
    /// queries for shuffles, etc.
    pub has_subgroups: bool,
    /// Info about subgroup size control, if available.
    pub subgroup_size: Option<SubgroupSize>,
    /// The GPU supports a real, grown-ass memory model.
    pub has_memory_model: bool,
    /// Whether staging buffers should be used.
    pub use_staging_buffers: bool,
}

#[derive(Clone, Debug)]
pub struct SubgroupSize {
    min: u32,
    max: u32,
}

pub trait Device: Sized {
    type Buffer: 'static;
    type Image;
    type Pipeline;
    type DescriptorSet;
    type QueryPool;
    type CmdBuf: CmdBuf<Self>;
    type Fence;
    type Semaphore;
    type PipelineBuilder: PipelineBuilder<Self>;
    type DescriptorSetBuilder: DescriptorSetBuilder<Self>;
    type Sampler;
    type ShaderSource: ?Sized;

    /// Query the GPU info.
    ///
    /// This method may be expensive, so the hub should call it once and retain
    /// the info.
    fn query_gpu_info(&self) -> GpuInfo;

    fn create_buffer(&self, size: u64, usage: BufferUsage) -> Result<Self::Buffer, Error>;

    /// Destroy a buffer.
    ///
    /// The same safety requirements hold as in Vulkan: the buffer cannot be used
    /// after this call, and all commands referencing this buffer must have completed.
    ///
    /// Maybe doesn't need result return?
    unsafe fn destroy_buffer(&self, buffer: &Self::Buffer) -> Result<(), Error>;

    unsafe fn create_image2d(&self, width: u32, height: u32) -> Result<Self::Image, Error>;

    /// Destroy an image.
    ///
    /// The same safety requirements hold as in Vulkan: the image cannot be used
    /// after this call, and all commands referencing this image must have completed.
    ///
    /// Use this only with images we created, not for swapchain images.
    ///
    /// Maybe doesn't need result return?
    unsafe fn destroy_image(&self, image: &Self::Image) -> Result<(), Error>;

    /// Start building a pipeline.
    ///
    /// A pipeline is a bit of shader IR plus a signature for what kinds of resources
    /// it expects.
    unsafe fn pipeline_builder(&self) -> Self::PipelineBuilder;

    /// Start building a descriptor set.
    ///
    /// A descriptor set is a binding of resources for a given pipeline.
    unsafe fn descriptor_set_builder(&self) -> Self::DescriptorSetBuilder;

    /// Create a simple compute pipeline that operates on buffers and storage images.
    ///
    /// This is provided as a convenience but will probably go away, as the functionality
    /// is subsumed by the builder.
    unsafe fn create_simple_compute_pipeline(
        &self,
        code: &Self::ShaderSource,
        n_buffers: u32,
        n_images: u32,
    ) -> Result<Self::Pipeline, Error> {
        let mut builder = self.pipeline_builder();
        builder.add_buffers(n_buffers);
        builder.add_images(n_images);
        builder.create_compute_pipeline(self, code)
    }

    /// Create a descriptor set for a given pipeline, binding buffers and images.
    ///
    /// This is provided as a convenience but will probably go away, as the functionality
    /// is subsumed by the builder.
    unsafe fn create_descriptor_set(
        &self,
        pipeline: &Self::Pipeline,
        bufs: &[&Self::Buffer],
        images: &[&Self::Image],
    ) -> Result<Self::DescriptorSet, Error> {
        let mut builder = self.descriptor_set_builder();
        builder.add_buffers(bufs);
        builder.add_images(images);
        builder.build(self, pipeline)
    }

    fn create_cmd_buf(&self) -> Result<Self::CmdBuf, Error>;

    fn create_query_pool(&self, n_queries: u32) -> Result<Self::QueryPool, Error>;

    /// Get results from query pool, destroying it in the process.
    ///
    /// The returned vector is one less than the number of queries; the first is used as
    /// a baseline.
    ///
    /// # Safety
    /// All submitted commands that refer to this query pool must have completed.
    unsafe fn fetch_query_pool(&self, pool: &Self::QueryPool) -> Result<Vec<f64>, Error>;

    unsafe fn run_cmd_bufs(
        &self,
        cmd_buf: &[&Self::CmdBuf],
        wait_semaphores: &[&Self::Semaphore],
        signal_semaphores: &[&Self::Semaphore],
        fence: Option<&Self::Fence>,
    ) -> Result<(), Error>;

    /// Copy data from the buffer to memory.
    ///
    /// Discussion question: add offset?
    ///
    /// # Safety
    ///
    /// The buffer must be valid to access. The destination memory must be valid to
    /// write to. The ranges must not overlap. The offset + size must be within
    /// the buffer's allocation, and size within the destination.
    unsafe fn read_buffer(
        &self,
        buffer: &Self::Buffer,
        dst: *mut u8,
        offset: u64,
        size: u64,
    ) -> Result<(), Error>;

    /// Copy data from memory to the buffer.
    ///
    /// # Safety
    ///
    /// The buffer must be valid to access. The source memory must be valid to
    /// read from. The ranges must not overlap. The offset + size must be within
    /// the buffer's allocation, and size within the source.
    unsafe fn write_buffer(
        &self,
        buffer: &Self::Buffer,
        contents: *const u8,
        offset: u64,
        size: u64,
    ) -> Result<(), Error>;

    unsafe fn create_semaphore(&self) -> Result<Self::Semaphore, Error>;
    unsafe fn create_fence(&self, signaled: bool) -> Result<Self::Fence, Error>;
    unsafe fn wait_and_reset(&self, fences: &[&Self::Fence]) -> Result<(), Error>;
    unsafe fn get_fence_status(&self, fence: Self::Fence) -> Result<bool, Error>;

    unsafe fn create_sampler(&self, params: SamplerParams) -> Result<Self::Sampler, Error>;
}

pub trait CmdBuf<D: Device> {
    unsafe fn begin(&mut self);

    unsafe fn finish(&mut self);

    unsafe fn dispatch(
        &mut self,
        pipeline: &D::Pipeline,
        descriptor_set: &D::DescriptorSet,
        size: (u32, u32, u32),
    );

    /// Insert an execution and memory barrier.
    ///
    /// Compute kernels (and other actions) after this barrier may read from buffers
    /// that were written before this barrier.
    unsafe fn memory_barrier(&mut self);

    /// Insert a barrier for host access to buffers.
    ///
    /// The host may read buffers written before this barrier, after the fence for
    /// the command buffer is signaled.
    ///
    /// See http://themaister.net/blog/2019/08/14/yet-another-blog-explaining-vulkan-synchronization/
    /// ("Host memory reads") for an explanation of this barrier.
    unsafe fn host_barrier(&mut self);

    unsafe fn image_barrier(
        &mut self,
        image: &D::Image,
        src_layout: ImageLayout,
        dst_layout: ImageLayout,
    );

    /// Clear the buffer.
    ///
    /// This is readily supported in Vulkan, but for portability it is remarkably
    /// tricky (unimplemented in gfx-hal right now). Possibly best to write a compute
    /// kernel, or organize the code not to need it.
    unsafe fn clear_buffer(&self, buffer: &D::Buffer, size: Option<u64>);

    unsafe fn copy_buffer(&self, src: &D::Buffer, dst: &D::Buffer);

    unsafe fn copy_image_to_buffer(&self, src: &D::Image, dst: &D::Buffer);

    unsafe fn copy_buffer_to_image(&self, src: &D::Buffer, dst: &D::Image);

    // low portability, dx12 doesn't support it natively
    unsafe fn blit_image(&self, src: &D::Image, dst: &D::Image);

    /// Reset the query pool.
    ///
    /// The query pool must be reset before each use, to avoid validation errors.
    /// This is annoying, and we could tweak the API to make it implicit, doing
    /// the reset before the first timestamp write.
    unsafe fn reset_query_pool(&mut self, pool: &D::QueryPool);

    unsafe fn write_timestamp(&mut self, pool: &D::QueryPool, query: u32);

    /// Prepare the timestamps for reading. This isn't required on Vulkan but
    /// is required on (at least) DX12.
    unsafe fn finish_timestamps(&mut self, pool: &D::QueryPool) {}
}

/// A builder for pipelines with more complex layouts.
pub trait PipelineBuilder<D: Device> {
    /// Add buffers to the pipeline. Each has its own binding.
    fn add_buffers(&mut self, n_buffers: u32);
    /// Add storage images to the pipeline. Each has its own binding.
    fn add_images(&mut self, n_images: u32);
    /// Add a binding with a variable-size array of textures.
    fn add_textures(&mut self, max_textures: u32);
    unsafe fn create_compute_pipeline(
        self,
        device: &D,
        code: &D::ShaderSource,
    ) -> Result<D::Pipeline, Error>;
}

/// A builder for descriptor sets with more complex layouts.
///
/// Note: the order needs to match the pipeline building, and it also needs to
/// be buffers, then images, then textures.
pub trait DescriptorSetBuilder<D: Device> {
    fn add_buffers(&mut self, buffers: &[&D::Buffer]);
    /// Add an array of storage images.
    ///
    /// The images need to be in `ImageLayout::General` layout.
    fn add_images(&mut self, images: &[&D::Image]);
    /// Add an array of textures.
    ///
    /// The images need to be in `ImageLayout::ShaderRead` layout.
    ///
    /// The same sampler is used for all textures, which is not very sophisticated;
    /// we should have a way to vary the sampler.
    fn add_textures(&mut self, images: &[&D::Image]);
    unsafe fn build(self, device: &D, pipeline: &D::Pipeline) -> Result<D::DescriptorSet, Error>;
}
