#![feature(prelude_import)]
#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
extern crate alloc;
mod arch {
    pub struct Arch;
    impl tkern_arch::Arch for Arch {}
}
mod view {
    mod config_table {
        use super::prelude::*;
        pub struct ConfigTableView {
            index: usize,
            error: Option<&'static str>,
        }
        #[automatically_derived]
        impl ::core::default::Default for ConfigTableView {
            #[inline]
            fn default() -> ConfigTableView {
                ConfigTableView {
                    index: ::core::default::Default::default(),
                    error: ::core::default::Default::default(),
                }
            }
        }
        impl ConfigTableView {
            pub(super) fn show(&mut self, table: &TableTy) {
                ::uefi_services::_print(
                    ::core::fmt::Arguments::new_v1(
                        &["", ""],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &::core::fmt::Arguments::new_v1(&["  Config table >"], &[]),
                            ),
                            ::core::fmt::ArgumentV1::new_display(&"\n"),
                        ],
                    ),
                );
                ::uefi_services::_print(::core::fmt::Arguments::new_v1(&["\n"], &[]));
                for (idx, entry) in table.config_table().iter().enumerate() {
                    ::uefi_services::_print(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &if self.index == idx { "> " } else { "  " },
                                ),
                            ],
                        ),
                    );
                    ::uefi_services::_print(
                        ::core::fmt::Arguments::new_v1(
                            &["guid: ", ", address: "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&entry.guid),
                                ::core::fmt::ArgumentV1::new_pointer(&entry.address),
                            ],
                        ),
                    );
                    if let Some(entry) = guid_to_name(entry.guid) {
                        ::uefi_services::_print(
                            ::core::fmt::Arguments::new_v1(
                                &["", ""],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(
                                        &::core::fmt::Arguments::new_v1(
                                            &[", Type: "],
                                            &[::core::fmt::ArgumentV1::new_display(&entry)],
                                        ),
                                    ),
                                    ::core::fmt::ArgumentV1::new_display(&"\n"),
                                ],
                            ),
                        );
                    } else {
                        ::uefi_services::_print(
                            ::core::fmt::Arguments::new_v1(&["\n"], &[]),
                        );
                    }
                }
                if let Some(error) = self.error.take() {
                    ::uefi_services::_print(
                        ::core::fmt::Arguments::new_v1(&["\n"], &[]),
                    );
                    ::uefi_services::_print(
                        ::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &::core::fmt::Arguments::new_v1(
                                        &[""],
                                        &[::core::fmt::ArgumentV1::new_display(&error)],
                                    ),
                                ),
                                ::core::fmt::ArgumentV1::new_display(&"\n"),
                            ],
                        ),
                    );
                }
            }
            pub(super) fn update(&mut self, table: &TableTy, key: Key) -> UpdateResult {
                match key {
                    Key::Printable(key) if key.is('\r') => {
                        if let Some(config) = table.config_table().get(self.index) {
                            if let Some(view)
                                = View::try_from_guid_and_ptr(config.guid, config.address) {
                                return UpdateResult::NewView(view);
                            } else {
                                self.error = Some("Unknown type; could not construct view");
                            }
                        } else {
                            self.error = Some("Config table entry not found");
                        }
                        UpdateResult::Render
                    }
                    Key::Special(ScanCode::UP) if self.index > 0 => {
                        self.index -= 1;
                        UpdateResult::Render
                    }
                    Key::Special(
                        ScanCode::DOWN,
                    ) if self.index + 1 < table.config_table().len() => {
                        self.index += 1;
                        UpdateResult::Render
                    }
                    Key::Special(ScanCode::RIGHT) => {
                        UpdateResult::NewView(
                            View::ProtoTable(
                                super::proto_table::ProtoTableView::default(),
                            ),
                        )
                    }
                    _ => UpdateResult::None,
                }
            }
        }
        trait CharIs {
            fn is(&self, char: char) -> bool;
        }
        impl CharIs for uefi::Char16 {
            fn is(&self, char: char) -> bool {
                if let Ok(char) = uefi::Char16::try_from(char) {
                    char == *self
                } else {
                    false
                }
            }
        }
    }
    mod proto_table {
        use super::prelude::*;
        use alloc::vec::Vec;
        use uefi::{prelude::BootServices, table::boot::SearchType, Handle};
        pub struct ProtoTableView {
            protos: Option<Vec<Handle>>,
            index: usize,
        }
        #[automatically_derived]
        impl ::core::default::Default for ProtoTableView {
            #[inline]
            fn default() -> ProtoTableView {
                ProtoTableView {
                    protos: ::core::default::Default::default(),
                    index: ::core::default::Default::default(),
                }
            }
        }
        impl ProtoTableView {
            pub(super) fn show(&mut self, table: &TableTy) {
                let protos = self
                    .protos
                    .get_or_insert_with(|| load_protos(table.boot_services()));
                ::uefi_services::_print(
                    ::core::fmt::Arguments::new_v1(
                        &["", ""],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &::core::fmt::Arguments::new_v1(&["< Protocols"], &[]),
                            ),
                            ::core::fmt::ArgumentV1::new_display(&"\n"),
                        ],
                    ),
                );
                ::uefi_services::_print(::core::fmt::Arguments::new_v1(&["\n"], &[]));
                for (idx, handle) in protos.iter().enumerate() {
                    if idx > 0 {
                        ::uefi_services::_print(
                            ::core::fmt::Arguments::new_v1(&[", "], &[]),
                        );
                    }
                    ::uefi_services::_print(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &if idx == self.index { "> " } else { "  " },
                                ),
                            ],
                        ),
                    );
                    ::uefi_services::_print(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_debug(&handle)],
                        ),
                    );
                }
            }
            pub(super) fn update(&mut self, table: &TableTy, key: Key) -> UpdateResult {
                let len = self.protos.as_ref().map(|p| p.len()).unwrap_or(0);
                match key {
                    Key::Special(ScanCode::UP) if self.index > 0 => {
                        self.index -= 1;
                        UpdateResult::Render
                    }
                    Key::Special(ScanCode::DOWN) if self.index + 1 < len => {
                        self.index += 1;
                        UpdateResult::Render
                    }
                    Key::Special(ScanCode::LEFT) => {
                        UpdateResult::NewView(
                            View::ConfigTable(
                                super::config_table::ConfigTableView::default(),
                            ),
                        )
                    }
                    _ => UpdateResult::None,
                }
            }
        }
        fn load_protos(boot: &BootServices) -> Vec<Handle> {
            let search_type = SearchType::AllHandles;
            let buffer_size = boot.locate_handle(search_type, None).unwrap();
            let mut handles = Vec::with_capacity(buffer_size);
            let buffer = handles.spare_capacity_mut();
            let buffer_size = boot.locate_handle(search_type, Some(buffer)).unwrap();
            unsafe {
                handles.set_len(buffer_size);
            }
            handles
        }
        enum Protocols {
            Input,
            DebugSupport,
            DevicePath,
            DevicePathFromText,
            DevicePathToText,
            LoadedImage,
            BlockIO,
            DiskIo2,
            DiskIo,
            FileInfo,
            FileSystemInfo,
            FileSystemVolumeLabel,
            SimpleFileSystem,
            PartitionInfo,
            BaseCode,
            MpServices,
            Rng,
            MemoryProtection,
            ShimLock,
            UnicodeCollation,
            GraphicsOutput,
            Pointer,
            Serial,
            Output,
        }
        impl Protocols {
            pub fn all() -> &'static [Protocols] {
                &[
                    Self::Input,
                    Self::DebugSupport,
                    Self::DevicePath,
                    Self::DevicePathFromText,
                    Self::DevicePathToText,
                    Self::LoadedImage,
                    Self::BlockIO,
                    Self::DiskIo2,
                    Self::DiskIo,
                    Self::FileInfo,
                    Self::FileSystemInfo,
                    Self::FileSystemVolumeLabel,
                    Self::SimpleFileSystem,
                    Self::PartitionInfo,
                    Self::BaseCode,
                    Self::MpServices,
                    Self::Rng,
                    Self::MemoryProtection,
                    Self::ShimLock,
                    Self::UnicodeCollation,
                    Self::GraphicsOutput,
                    Self::Pointer,
                    Self::Serial,
                    Self::Output,
                ]
            }
            pub fn guid(&self) -> uefi::Guid {
                match self {
                    Self::Input => {
                        <uefi::proto::console::text::Input as uefi::Identify>::GUID
                    }
                    Self::DebugSupport => {
                        <uefi::proto::debug::DebugSupport as uefi::Identify>::GUID
                    }
                    Self::DevicePath => {
                        <uefi::proto::device_path::DevicePath as uefi::Identify>::GUID
                    }
                    Self::DevicePathFromText => {
                        <uefi::proto::device_path::text::DevicePathFromText as uefi::Identify>::GUID
                    }
                    Self::DevicePathToText => {
                        <uefi::proto::device_path::text::DevicePathToText as uefi::Identify>::GUID
                    }
                    Self::LoadedImage => {
                        <uefi::proto::loaded_image::LoadedImage as uefi::Identify>::GUID
                    }
                    Self::BlockIO => {
                        <uefi::proto::media::block::BlockIO as uefi::Identify>::GUID
                    }
                    Self::DiskIo2 => {
                        <uefi::proto::media::disk::DiskIo2 as uefi::Identify>::GUID
                    }
                    Self::DiskIo => {
                        <uefi::proto::media::disk::DiskIo as uefi::Identify>::GUID
                    }
                    Self::FileInfo => {
                        <uefi::proto::media::file::FileInfo as uefi::Identify>::GUID
                    }
                    Self::FileSystemInfo => {
                        <uefi::proto::media::file::FileSystemInfo as uefi::Identify>::GUID
                    }
                    Self::FileSystemVolumeLabel => {
                        <uefi::proto::media::file::FileSystemVolumeLabel as uefi::Identify>::GUID
                    }
                    Self::SimpleFileSystem => {
                        <uefi::proto::media::fs::SimpleFileSystem as uefi::Identify>::GUID
                    }
                    Self::PartitionInfo => {
                        <uefi::proto::media::partition::PartitionInfo as uefi::Identify>::GUID
                    }
                    Self::BaseCode => {
                        <uefi::proto::network::pxe::BaseCode as uefi::Identify>::GUID
                    }
                    Self::MpServices => {
                        <uefi::proto::pi::mp::MpServices as uefi::Identify>::GUID
                    }
                    Self::Rng => <uefi::proto::rng::Rng as uefi::Identify>::GUID,
                    Self::MemoryProtection => {
                        <uefi::proto::security::MemoryProtection as uefi::Identify>::GUID
                    }
                    Self::ShimLock => {
                        <uefi::proto::shim::ShimLock as uefi::Identify>::GUID
                    }
                    Self::UnicodeCollation => {
                        <uefi::proto::string::unicode_collation::UnicodeCollation as uefi::Identify>::GUID
                    }
                    Self::GraphicsOutput => {
                        <uefi::proto::console::gop::GraphicsOutput as uefi::Identify>::GUID
                    }
                    Self::Pointer => {
                        <uefi::proto::console::pointer::Pointer as uefi::Identify>::GUID
                    }
                    Self::Serial => {
                        <uefi::proto::console::serial::Serial as uefi::Identify>::GUID
                    }
                    Self::Output => {
                        <uefi::proto::console::text::Output as uefi::Identify>::GUID
                    }
                }
            }
        }
    }
    mod prelude {
        pub(super) use super::{guid_to_name, TableTy, UpdateResult, View};
        pub use uefi::proto::console::text::{Key, ScanCode};
        pub use uefi_services::{print, println};
    }
    use core::ffi::c_void;
    use uefi::proto::console::text::Key;
    type TableTy = uefi::table::SystemTable<uefi::table::Boot>;
    pub struct State {
        table: TableTy,
        view: View,
    }
    enum View {
        ConfigTable(config_table::ConfigTableView),
        ProtoTable(proto_table::ProtoTableView),
    }
    impl State {
        pub fn new(
            _image: uefi::Handle,
            table: uefi::table::SystemTable<uefi::table::Boot>,
        ) -> Self {
            Self {
                table,
                view: View::ConfigTable(config_table::ConfigTableView::default()),
            }
        }
        pub fn run(mut self) -> ! {
            let input_event = unsafe {
                self.table.stdin().wait_for_key_event().unsafe_clone()
            };
            loop {
                self.table.stdout().clear().unwrap();
                self.view.show(&self.table);
                loop {
                    let key = loop {
                        if let Some(key) = self.table.stdin().read_key().unwrap() {
                            break key;
                        }
                        self.table
                            .boot_services()
                            .wait_for_event(&mut [unsafe { input_event.unsafe_clone() }])
                            .unwrap();
                    };
                    match self.view.update(&self.table, key) {
                        UpdateResult::None => {}
                        UpdateResult::Render => break,
                        UpdateResult::NewView(view) => {
                            self.view = view;
                            break;
                        }
                    }
                }
            }
        }
    }
    pub(self) enum UpdateResult {
        None,
        Render,
        NewView(View),
    }
    impl View {
        pub(self) fn try_from_guid_and_ptr(
            guid: uefi::Guid,
            ptr: *const c_void,
        ) -> Option<Self> {
            None
        }
        fn show(&mut self, table: &TableTy) {
            match self {
                View::ConfigTable(view) => view.show(table),
                View::ProtoTable(view) => view.show(table),
            }
        }
        fn update(&mut self, table: &TableTy, key: Key) -> UpdateResult {
            match self {
                View::ConfigTable(view) => view.update(table, key),
                View::ProtoTable(view) => view.update(table, key),
            }
        }
    }
    pub fn guid_to_name(guid: uefi::Guid) -> Option<&'static str> {
        use uefi::Identify;
        Some(
            match guid {
                uefi::proto::console::text::Input::GUID => "text::Input",
                uefi::proto::debug::DebugSupport::GUID => "debug::DebugSupport",
                uefi::proto::device_path::DevicePath::GUID => "device_path::DevicePath",
                uefi::proto::device_path::text::DevicePathFromText::GUID => {
                    "device_path::text::DevicePathFromText"
                }
                uefi::proto::device_path::text::DevicePathToText::GUID => {
                    "device_path::text::DevicePathToText"
                }
                uefi::proto::loaded_image::LoadedImage::GUID => {
                    "loaded_image::LoadedImage"
                }
                uefi::proto::media::block::BlockIO::GUID => "media::block::BlockIO",
                uefi::proto::media::disk::DiskIo2::GUID => "media::disk::DiskIo2",
                uefi::proto::media::disk::DiskIo::GUID => "media::disk::DiskIo",
                uefi::proto::media::file::FileInfo::GUID => "media::file::FileInfo",
                uefi::proto::media::file::FileSystemInfo::GUID => {
                    "media::file::FileSystemInfo"
                }
                uefi::proto::media::file::FileSystemVolumeLabel::GUID => {
                    "media::file::FileSystemVolumeLabel"
                }
                uefi::proto::media::fs::SimpleFileSystem::GUID => {
                    "media::fs::SimpleFileSystem"
                }
                uefi::proto::media::partition::PartitionInfo::GUID => {
                    "media::partition::PartitionInfo"
                }
                uefi::proto::network::pxe::BaseCode::GUID => "network::pxe::BaseCode",
                uefi::proto::pi::mp::MpServices::GUID => "pi::mp::MpServices",
                uefi::proto::rng::Rng::GUID => "rng::Rng",
                uefi::proto::security::MemoryProtection::GUID => {
                    "security::MemoryProtection"
                }
                uefi::proto::shim::ShimLock::GUID => "shim::ShimLock",
                uefi::proto::string::unicode_collation::UnicodeCollation::GUID => {
                    "string::unicode_collation::UnicodeCollation"
                }
                uefi::proto::console::gop::GraphicsOutput::GUID => "gop::GraphicsOutput",
                uefi::proto::console::pointer::Pointer::GUID => {
                    "console::pointer::Pointer"
                }
                uefi::proto::console::serial::Serial::GUID => "console::serial::Serial",
                uefi::proto::console::text::Output::GUID => "console::text::Output",
                uefi::table::cfg::ACPI2_GUID => "table::cfg::ACPI2_GUID",
                uefi::table::cfg::ACPI_GUID => "table::cfg::ACPI_GUID",
                uefi::table::cfg::DEBUG_IMAGE_INFO_GUID => {
                    "table::cfg::DEBUG_IMAGE_INFO_GUID"
                }
                uefi::table::cfg::DXE_SERVICES_GUID => "table::cfg::DXE_SERVICES_GUID",
                uefi::table::cfg::HAND_OFF_BLOCK_LIST_GUID => {
                    "table::cfg::HAND_OFF_BLOCK_LIST_GUID"
                }
                uefi::table::cfg::LZMA_COMPRESS_GUID => "table::cfg::LZMA_COMPRESS_GUID",
                uefi::table::cfg::MEMORY_STATUS_CODE_RECORD_GUID => {
                    "table::cfg::MEMORY_STATUS_CODE_RECORD_GUID"
                }
                uefi::table::cfg::MEMORY_TYPE_INFORMATION_GUID => {
                    "table::cfg::MEMORY_TYPE_INFORMATION_GUID"
                }
                uefi::table::cfg::PROPERTIES_TABLE_GUID => {
                    "table::cfg::PROPERTIES_TABLE_GUID"
                }
                uefi::table::cfg::SMBIOS3_GUID => "table::cfg::SMBIOS3_GUID",
                uefi::table::cfg::SMBIOS_GUID => "table::cfg::SMBIOS_GUID",
                uefi::table::cfg::TIANO_COMPRESS_GUID => {
                    "table::cfg::TIANO_COMPRESS_GUID"
                }
                EFI_MEMORY_ATTRIBUTES_TABLE => "EfiMemoryAttributesTable",
                _ => return None,
            },
        )
    }
    const EFI_MEMORY_ATTRIBUTES_TABLE: uefi::Guid = uefi::Guid::from_bytes([
        0x1d,
        0x91,
        0xfa,
        0xdc,
        0xeb,
        0x26,
        0x9f,
        0x46,
        0xa2,
        0x20,
        0x38,
        0xb7,
        0xdc,
        0x46,
        0x12,
        0x20,
    ]);
}
use core::panic::PanicInfo;
use uefi::prelude::entry;
#[export_name = "efi_main"]
extern "efiapi" fn efi_main(
    image: uefi::Handle,
    mut system_table: uefi::table::SystemTable<uefi::table::Boot>,
) -> uefi::Status {
    unsafe {
        system_table.boot_services().set_image_handle(image);
    }
    unsafe {
        uefi::alloc::init(system_table.boot_services());
    }
    system_table.stdout().clear().unwrap();
    uefi_services::init(&mut system_table).unwrap();
    view::State::new(image, system_table).run();
}
const _: extern "efiapi" fn(
    ::uefi::Handle,
    ::uefi::table::SystemTable<::uefi::table::Boot>,
) -> ::uefi::Status = efi_main;
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
