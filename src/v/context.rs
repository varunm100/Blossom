use erupt::{
    cstr,
    utils::{self, surface},
    vk, DeviceLoader, EntryLoader, InstanceLoader,
};
use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
    sync::Arc,
};
use erupt::vk::DebugUtilsMessengerEXT;

const TITLE: &str = "Blossom Vulkan Backend";
const LAYER_KHRONOS_VALIDATION: *const c_char = cstr!("VK_LAYER_KHRONOS_validation");

unsafe extern "system" fn debug_callback(
    _message_severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    _message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    info!(
        "{}",
        CStr::from_ptr((*p_callback_data).p_message).to_string_lossy()
    );

    vk::FALSE
}

const VALIDATION_LAYERS:bool = true;

pub struct Context {
    messenger: DebugUtilsMessengerEXT,
    surface: vk::SurfaceKHR,
    instance_loader: Arc<InstanceLoader>,
    entry: Arc<EntryLoader>,
}

impl Context {
    pub fn create(window_handle: &impl raw_window_handle::HasRawWindowHandle) -> Self {
        env_logger::init();
        let entry = Arc::new(EntryLoader::new().unwrap());
        info!(
            "{} - Vulkan Instance {}.{}.{}",
            "Blossom",
            vk::api_version_major(entry.instance_version()),
            vk::api_version_minor(entry.instance_version()),
            vk::api_version_patch(entry.instance_version())
        );

        let application_name = CString::new("Blossom").unwrap();
        let engine_name = CString::new("Blossom").unwrap();
        let app_info = vk::ApplicationInfoBuilder::new()
            .application_name(&application_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(&engine_name)
            .engine_version(vk::make_api_version(0, 1, 0, 0))
            .api_version(vk::make_api_version(0, 1, 3, 0));

        let mut instance_extensions = surface::enumerate_required_extensions(&window_handle).unwrap();
        if VALIDATION_LAYERS {
            instance_extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION_NAME);
        }

        let mut instance_layers = Vec::new();
        if VALIDATION_LAYERS {
            instance_layers.push(LAYER_KHRONOS_VALIDATION);
        }

        let device_extensions = vec![vk::KHR_SWAPCHAIN_EXTENSION_NAME];

        let mut device_layers = Vec::new();
        if VALIDATION_LAYERS {
            device_layers.push(LAYER_KHRONOS_VALIDATION);
        }

        let instance_info = vk::InstanceCreateInfoBuilder::new()
            .application_info(&app_info)
            .enabled_extension_names(&instance_extensions)
            .enabled_layer_names(&instance_layers);

        let instance_loader = Arc::new(unsafe { InstanceLoader::new(&entry, &instance_info) }.unwrap());

        let messenger = if VALIDATION_LAYERS {
            let messenger_info = vk::DebugUtilsMessengerCreateInfoEXTBuilder::new()
                .message_severity(
                    //vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT |
                         vk::DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT |
                         vk::DebugUtilsMessageSeverityFlagsEXT::ERROR_EXT,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL_EXT
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT,
                )
                .pfn_user_callback(Some(debug_callback));

            unsafe { instance_loader.create_debug_utils_messenger_ext(&messenger_info, None) }.unwrap()
        } else {
            Default::default()
        };

        let surface = unsafe { surface::create_surface(&instance_loader, &window_handle, None) }.unwrap();

        Context {
            entry,
            instance_loader,
            messenger,
            surface
        }
    }

    pub fn get_highest_vram_gpu(&self) -> Gpu {
        let devices= unsafe { self.instance_loader.enumerate_physical_devices(None) }.unwrap();
        let mut device_index = 0usize;
        let mut max_vram: vk::DeviceSize = 0;
        for (i, d) in devices.iter().enumerate() {
            unsafe {
                let props = self.instance_loader.get_physical_device_memory_properties(*d);
                max_vram = if props.memory_heaps[0].size > max_vram {
                    device_index = i;
                    props.memory_heaps[0].size
                } else {
                    max_vram
                }
            }
        }

        let device_extensions = vec![vk::KHR_SWAPCHAIN_EXTENSION_NAME];
        let mut device_layers = Vec::new();
        if VALIDATION_LAYERS { device_layers.push(LAYER_KHRONOS_VALIDATION); }

        // TODO: support multiple queues
        let queue_info = vec![vk::DeviceQueueCreateInfoBuilder::new()
            .queue_family_index(0)
            .queue_priorities(&[1.0])];
        let features = vk::PhysicalDeviceFeaturesBuilder::new();

        let device_info = vk::DeviceCreateInfoBuilder::new()
            .queue_create_infos(&queue_info)
            .enabled_features(&features)
            .enabled_extension_names(&device_extensions)
            .enabled_layer_names(&device_layers);

        let physical_device = devices[device_index];
        let device = Arc::new(unsafe { DeviceLoader::new(&self.instance_loader, physical_device, &device_info) }.unwrap());

        let props = unsafe { self.instance_loader.get_physical_device_properties(physical_device) };
        info!("Fetching Device: {:?}", unsafe { CStr::from_ptr(props.device_name.as_ptr()) });
        info!("Fetched Device Driver Version: {:?}", props.driver_version);
        Gpu {
            physical_device,
            device,
        }
    }
}

pub struct Gpu {
    physical_device: vk::PhysicalDevice,
    pub device: Arc<DeviceLoader>,
}

