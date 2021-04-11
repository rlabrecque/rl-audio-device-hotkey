use bindings::{Windows::Win32::CoreAudio::*, Windows::Win32::SystemServices::*};

pub fn switch_to_next_output_device() -> windows::Result<()> {
    windows::initialize_mta()?;

    let enumerator: IMMDeviceEnumerator = windows::create_instance(&MMDeviceEnumerator)?;

    let default_device = enumerator_get_default_audio_endpoint(&enumerator)?;
    println!("default_device: {:#?}", default_device);

    let default_device_id = device_get_id(&default_device)?;
    let default_device_id_str =
        unsafe { widestring::U16CString::from_ptr_str(default_device_id.0).to_string_lossy() };
    println!(
        "default_device_id: {:#?}\n{}",
        default_device_id, default_device_id_str
    );

    let collection = enumerator_enum_audio_endpoints(&enumerator)?;

    let count = collection_get_count(&collection)?;
    println!("Count: {}", count);

    for index in 0..count {
        let device = collection_get_item(&collection, index)?;
        println!("device: {:#?}", device);

        let device_id = device_get_id(&device)?;
        let device_id_str =
            unsafe { widestring::U16CString::from_ptr_str(default_device_id.0).to_string_lossy() };
        println!("device_id: {:#?}\n{}", device_id, device_id_str);
    }

    Ok(())
}

fn enumerator_get_default_audio_endpoint(
    enumerator: &IMMDeviceEnumerator,
) -> windows::Result<IMMDevice> {
    let mut default_device: Option<IMMDevice> = None;
    unsafe {
        enumerator
            .GetDefaultAudioEndpoint(EDataFlow::eRender, ERole::eConsole, &mut default_device)
            .and_some(default_device)
    }
}

fn enumerator_enum_audio_endpoints(
    enumerator: &IMMDeviceEnumerator,
) -> windows::Result<IMMDeviceCollection> {
    let mut collection: Option<IMMDeviceCollection> = None;
    unsafe {
        enumerator
            .EnumAudioEndpoints(EDataFlow::eRender, DEVICE_STATE_ACTIVE, &mut collection)
            .and_some(collection)
    }
}

fn device_get_id(device: &IMMDevice) -> windows::Result<PWSTR> {
    let mut id = PWSTR::default();
    unsafe { device.GetId(&mut id).ok()? };
    Ok(id)
}

fn collection_get_count(collection: &IMMDeviceCollection) -> windows::Result<u32> {
    let mut count = u32::default();
    unsafe { collection.GetCount(&mut count).ok()? };
    Ok(count)
}

fn collection_get_item(collection: &IMMDeviceCollection, index: u32) -> windows::Result<IMMDevice> {
    let mut device: Option<IMMDevice> = None;
    unsafe { collection.Item(index, &mut device).and_some(device) }
}
