use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*, Win32::UI::WindowsAndMessaging::*,
};

pub mod color;
pub mod renderer;
pub mod vertex;
pub mod window;
