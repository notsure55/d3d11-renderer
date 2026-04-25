#[allow(unused_imports)]
use super::window::Window;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct3D::Fxc::*, Win32::Graphics::Direct3D::*,
    Win32::Graphics::Direct3D11::*, Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
    Win32::Graphics::Gdi::*, Win32::System::LibraryLoader::*, Win32::UI::WindowsAndMessaging::*,
};

pub struct Renderer {
    pub window: Window,
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    swap_chain: IDXGISwapChain,
}

impl Renderer {
    pub fn run(window: Window) -> Result<()> {
        unsafe {
            let mut device = None;
            let mut context = None;

            let (device, context) = {
                D3D11CreateDevice(
                    None,
                    D3D_DRIVER_TYPE_HARDWARE,
                    HMODULE(std::ptr::null_mut()),
                    D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                    Some(&D3D_LEVELS),
                    D3D11_SDK_VERSION,
                    Some(&mut device),
                    None,
                    Some(&mut context),
                )?;

                (
                    device.unwrap().cast::<ID3D11Device1>()?,
                    context.unwrap().cast::<ID3D11DeviceContext1>()?,
                )
            };

            let swap_chain = {
                let dxgi_device = device.clone().cast::<IDXGIDevice1>()?;
                let dxgi_adapter = dxgi_device.GetAdapter()?;
                let adapter_desc = dxgi_adapter.GetDesc()?;
                let factory = dxgi_adapter.GetParent::<IDXGIFactory2>()?;

                let swap_chain_desc = DXGI_SWAP_CHAIN_DESC1 {
                    Width: 0,
                    Height: 0,
                    Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    SampleDesc: DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0,
                    },
                    BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                    BufferCount: 2,
                    Scaling: DXGI_SCALING_STRETCH,
                    SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
                    AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED,
                    Flags: 0,
                    ..Default::default()
                };

                factory.CreateSwapChainForHwnd(
                    &device,
                    window.hwnd,
                    &swap_chain_desc,
                    None,
                    None,
                )?
            };

            let render_view = {
                let frame_buffer = swap_chain.GetBuffer::<ID3D11Texture2D>(0)?;
                let mut buffer_view = None;
                device.CreateRenderTargetView(&frame_buffer, None, Some(&mut buffer_view))?;
                buffer_view.unwrap()
            };

            dbg!(&render_view);

            let (vs_blob, vertex_shader) = {
                let mut vs_blob = None;
                let mut error_blob = None;

                D3DCompileFromFile(
                    w!("c:/Users/shwarztoter/projects/coding/rust/renderer/shaders.hlsl"),
                    None,
                    None,
                    s!("vs_main"),
                    s!("vs_5_0"),
                    0,
                    0,
                    &mut vs_blob,
                    Some(&mut error_blob),
                )?;

                if let Some(error) = error_blob {
                    let ptr = error.GetBufferPointer() as *const u8;
                    let size = error.GetBufferSize();
                    let msg = std::slice::from_raw_parts(ptr, size);
                    panic!("VS compile error: {}", std::str::from_utf8(msg).unwrap());
                }

                let vs_blob = vs_blob.unwrap();

                let mut vertex_shader = None;

                let bytes = std::slice::from_raw_parts(
                    vs_blob.GetBufferPointer() as _,
                    vs_blob.GetBufferSize() as _,
                );

                device.CreateVertexShader(bytes, None, Some(&mut vertex_shader))?;

                (vs_blob, vertex_shader.unwrap())
            };

            let (ps_blob, pixel_shader) = {
                let mut ps_blob = None;
                let mut error_blob = None;

                D3DCompileFromFile(
                    w!("c:/Users/shwarztoter/projects/coding/rust/renderer/shaders.hlsl"),
                    None,
                    None,
                    s!("ps_main"),
                    s!("ps_5_0"),
                    0,
                    0,
                    &mut ps_blob,
                    Some(&mut error_blob),
                )?;

                if let Some(error) = error_blob {
                    let ptr = error.GetBufferPointer() as *const u8;
                    let size = error.GetBufferSize();
                    let msg = std::slice::from_raw_parts(ptr, size);
                    panic!("VS compile error: {}", std::str::from_utf8(msg).unwrap());
                }

                let ps_blob = ps_blob.unwrap();

                let mut pixel_shader = None;

                let bytes = std::slice::from_raw_parts(
                    ps_blob.GetBufferPointer() as _,
                    ps_blob.GetBufferSize() as _,
                );

                device.CreatePixelShader(bytes, None, Some(&mut pixel_shader))?;

                (ps_blob, pixel_shader.unwrap())
            };

            let input_layout = {
                let input_elem_desc = [
                    D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: s!("POS"),
                        SemanticIndex: 0,
                        Format: DXGI_FORMAT_R32G32_FLOAT,
                        InputSlot: 0,
                        AlignedByteOffset: 0,
                        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0,
                    },
                    D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: s!("COL"),
                        SemanticIndex: 0,
                        Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                        InputSlot: 0,
                        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
                        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0,
                    },
                ];

                let bytes = std::slice::from_raw_parts(
                    vs_blob.GetBufferPointer() as _,
                    vs_blob.GetBufferSize() as _,
                );

                let mut layout = None;

                device.CreateInputLayout(&input_elem_desc, bytes, Some(&mut layout))?;

                layout.unwrap()
            };

            let (vertex_buffer, num_verts, stride, offset) = {
                let vertex_data: [f32; 18] = [
                    0.0, 0.5, 0., 1., 0., 1., 0.5, -0.5, 1., 0., 0., 1., -0.5, -0.5, 0., 0., 1., 1.,
                ];

                let stride = 6 * std::mem::size_of::<f32>();
                let num_verts = std::mem::size_of_val(&vertex_data) / stride;
                let offset = 0;

                let vertex_buffer_desc = D3D11_BUFFER_DESC {
                    ByteWidth: std::mem::size_of_val(&vertex_data) as u32,
                    Usage: D3D11_USAGE_IMMUTABLE,
                    BindFlags: D3D11_BIND_VERTEX_BUFFER.0 as u32,
                    ..Default::default()
                };

                let vertex_subresource_data = D3D11_SUBRESOURCE_DATA {
                    pSysMem: vertex_data.as_ptr() as _,
                    ..Default::default()
                };

                let mut buffer = None;

                device.CreateBuffer(
                    &vertex_buffer_desc,
                    Some(&vertex_subresource_data),
                    Some(&mut buffer),
                )?;

                (buffer.unwrap(), num_verts as u32, stride as u32, offset)
            };

            loop {
                if window.process_msg().is_err() {
                    break;
                }

                let background_color = [0.1, 0.2, 0.6, 1.0];
                context.ClearRenderTargetView(&render_view, &background_color);

                let mut rect = RECT::default();
                GetClientRect(window.hwnd, &mut rect)?;

                let viewport = D3D11_VIEWPORT {
                    TopLeftX: 0.0,
                    TopLeftY: 0.0,
                    Width: (rect.right - rect.left) as f32,
                    Height: (rect.bottom - rect.top) as f32,
                    MinDepth: 0.0,
                    MaxDepth: 1.0,
                };

                context.RSSetViewports(Some(&[viewport]));

                context.OMSetRenderTargets(Some(&[Some(render_view.clone())]), None);

                context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
                context.IASetInputLayout(&input_layout);

                context.VSSetShader(&vertex_shader, None);
                context.PSSetShader(&pixel_shader, None);

                context.IASetVertexBuffers(
                    0,
                    1,
                    Some(&Some(vertex_buffer.clone())),
                    Some(&stride),
                    Some(&offset),
                );

                context.Draw(num_verts, 0);

                swap_chain.Present(1, DXGI_PRESENT(0));
            }

            Ok(())
        }
    }
}

const D3D_LEVELS: [D3D_FEATURE_LEVEL; 1] = [D3D_FEATURE_LEVEL_11_0];
