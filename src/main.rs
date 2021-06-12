use std::sync::Arc;
use vulkano::instance::{Instance, PhysicalDevice, InstanceExtensions};
use vulkano::device::{DeviceExtensions, Features, Device};
use vulkano::image::{AttachmentImage, ImageUsage, ImageDimensions, ImageLayout};
use vulkano::format::{Format, ClearValue};
use vulkano::render_pass::{RenderPass, RenderPassDesc, AttachmentDesc};
use vulkano::render_pass::Framebuffer;
use vulkano::image::view::ImageView;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::command_buffer::PrimaryCommandBuffer;
use vulkano::Version;

fn main() {
	let app_infos = vulkano::app_info_from_cargo_toml!();
	let instance = Instance::new(Some(&app_infos), Version::V1_2, &InstanceExtensions::none(), None).unwrap();
	let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
	let queue_family = physical.queue_families().next().unwrap();
	let (device, mut queues) = Device::new(physical, &Features::none(), &DeviceExtensions::none(), Some((queue_family, 1.0))).unwrap();
	let queue = queues.next().unwrap();
	
	let image_usage = ImageUsage {
		transfer_source: true,
		transfer_destination: true,
		color_attachment: true,
		..ImageUsage::none()
	};
	
	let image1 = AttachmentImage::with_usage(device.clone(), [256, 256], Format::R8G8B8A8Unorm, image_usage).unwrap();
	let image2 = AttachmentImage::with_usage(device.clone(), [256, 256], Format::R8G8B8A8Unorm, image_usage).unwrap();
	let image3 = AttachmentImage::with_usage(device.clone(), [256, 256], Format::R8G8B8A8Unorm, image_usage).unwrap();
	
	let render_pass = Arc::new(vulkano::single_pass_renderpass!(device.clone(),
		attachments: {
			image1: { load: Load, store: Store, format: Format::R8G8B8A8Unorm, samples: 1, },
			image2: { load: Load, store: Store, format: Format::R8G8B8A8Unorm, samples: 1, },
			image3: { load: Load, store: Store, format: Format::R8G8B8A8Unorm, samples: 1, }
		},
		pass: {
			color: [image1, image2, image3],
			depth_stencil: {}
		}
	).unwrap());
	
	let framebuffer = Arc::new(Framebuffer::start(render_pass.clone())
		.add(ImageView::new(image1.clone()).unwrap()).unwrap()
		.add(ImageView::new(image2.clone()).unwrap()).unwrap()
		.add(ImageView::new(image3.clone()).unwrap()).unwrap()
		.build().unwrap());
	
	let mut builder = AutoCommandBufferBuilder::primary(device.clone(), queue_family, CommandBufferUsage::OneTimeSubmit).unwrap();
	
	builder.begin_render_pass(framebuffer, SubpassContents::Inline, [ClearValue::None, ClearValue::None, ClearValue::None]).unwrap()
	       .end_render_pass().unwrap()
	       .copy_image(image3.clone(),
	                   [0, 0, 0],
	                   0,
	                   0,
	                   image1.clone(),
	                   [0, 0, 0],
	                   0,
	                   0,
	                   [256, 256, 1],
	                   1).unwrap()
	       .copy_image(image3.clone(),
	                   [0, 0, 0],
	                   0,
	                   0,
	                   image1.clone(),
	                   [0, 0, 0],
	                   0,
	                   0,
	                   [256, 256, 1],
	                   1).unwrap();
	
	builder.build().unwrap();
}
