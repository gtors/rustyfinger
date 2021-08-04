

bitflags!{
    pub struct DeviceFeature: u32 {
        /// Device does not support any feature
        const Nope = 0;
        /// Supports image capture
        const Capture = 1 << 0;
        /// Supports finger verification
        const Verify = 1 << 1;
        /// Supports finger identification
        const Identify = 1 << 2;
        /// Device has a persistent storage;
        const Storage = 1 << 3;
        /// Supports listing the storage templates
        const StorageList = 1 << 4;
        /// Supports deleting stored templates
        const StorageDelete = 1 << 5;
        /// Supports clearing the whole storage
        const StorageClear = 1 << 6;
        /// Natively supports duplicates detection
        const DuplicatesCheck = 1 << 7;
    }
}

/// Device types
pub enum DeviceKind {
    /// The device is a virtual device
    Virtual,
    /// The device is a udev device
    Udev,
    /// The device is a USB device
    USB,
}


enum DeviceError {
    /// Device not opened
    NotOpen,
    /// Feature not supported
    NotSupported,
    /// Device is busy on other task
    Busy,
}

/// Current active action of the device. A driver can retrieve the action.
enum DeviceAction {
    /// No action is active
    Nope,
    /// Probe device for support and information
    Probe,
    /// Device is currently being opened
    Open,
    /// Device is currently being closed
    Close,
    /// Device is currently verifying
    Verify,
    /// Device is currently identifying
    Identify,
    /// Device is currently capturing an image
    Capture,
    /// Device stored prints are being queried
    ListPrints,
    /// Device stored print is being deleted
    DeletePrint,
    /// Device stored prints are being deleted
    ClearStorage,
}


pub struct Enroll {
    // print: Fingerprint,
    // progress: ???
    // progress_data: ???
    // progress_destroy: ???
}


pub struct Match {
    // enrolled_fingreprint: Fingerprint
    // gallery: ???
    result_reported: bool,
    // matched_fingerprint: Fingerprint,
    // fingerprint: Fingerprint,
    // match_cb: ???
    // match_data: ???
    // match_destroy: ???
}


pub struct Device {
    device_id: String,
    device_name: String,
    driver: Driver,
    features: DeviceFeature,
    is_open: bool,
    is_busy: bool,
    is_removed: bool,
    wait_for_finger: bool,

    kind: DeviceKind,

    // usb_device: UsbDevice,
    // virtual_env: char, 
    // udev_data: {spidev_path, hidraw_path},
    
    // scan_kind: ScanKind,
    driver_data: u64,
    nr_enroll_stages: i32,
    // sources: ???
    
    current_action: DeviceAction,
    // current_task: ???
    // current_user_cb: ???
    // current_cancellable_id: ???
    // current_idle_cancel_source: ??
    // current_task_idle_return_source: ???
    // finger_status: FingerStatusFlags
}

impl Device {
    /// Start an asynchronous operation to delete all prints from the device.
    pub async fn clear_storage(&self) -> Result<(), DeviceError> {
        self.clear_storage_finish().await;

        if !self.is_open {
            return Err(DeviceError::NotOpen)
        }

        if self.is_busy {
            return Err(DeviceError::Busy)
        }

        if !self.features.intersects(DeviceFeature::Storage) {
            return Err(DeviceError::NotSupported)
        }

        if !self.features.intersects(DeviceFeature::StorageClear) {
            return Err(DeviceError::NotSupported)
        }

        self.current_action = DeviceAction::ClearStorage;
        self.driver.clear_storage()
    }

    async fn clear_storage_finish(&self) {
    }

    /// List features supported by the device
    pub fn get_features(&self) -> DeviceFeature {
        self->features
    }

    /// Checks if device supports the requested device feature
    pub fn has_feature(
        &self, 
        /// flags to check against device supported features 
        features: DeviceFeature
    ) -> bool {
        (self.features & features) == features
    }
}
