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

use super::color::*;
use super::vertex::*;

#[derive(Debug)]
pub struct Renderer {
    hwnd: HWND,
    device: ID3D11Device1,
    context: ID3D11DeviceContext1,
    swap_chain: IDXGISwapChain1,
    render_view: ID3D11RenderTargetView,
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
    input_layout: ID3D11InputLayout,
    vertex_buffer: ID3D11Buffer,
}

unsafe impl Sync for Renderer {}
unsafe impl Send for Renderer {}

impl Renderer {
    pub fn new(
        hwnd: HWND,
        device: ID3D11Device1,
        context: ID3D11DeviceContext1,
        swap_chain: IDXGISwapChain1,
    ) -> Result<Self> {
        unsafe {
            let render_view = {
                let frame_buffer = swap_chain.GetBuffer::<ID3D11Texture2D>(0)?;
                let mut buffer_view = None;
                device.CreateRenderTargetView(&frame_buffer, None, Some(&mut buffer_view))?;
                buffer_view.unwrap()
            };

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

            let vertex_data = [
                Triangle::new([
                    Vertex::new(Vec2::new([0.0, 0.50]), Color::new([1., 0., 0., 1.])),
                    Vertex::new(Vec2::new([0.25, 0.25]), Color::new([1., 0., 0., 1.])),
                    Vertex::new(Vec2::new([-0.25, 0.25]), Color::new([1., 0., 0., 1.])),
                ]),
                Triangle::new([
                    Vertex::new(Vec2::new([0.0, -0.50]), Color::new([0., 1., 0., 1.])),
                    Vertex::new(Vec2::new([0.25, -0.75]), Color::new([1., 0., 0., 1.])),
                    Vertex::new(Vec2::new([-0.25, -0.75]), Color::new([0., 0., 1., 1.])),
                ]),
            ];

            let (vertex_buffer, offset) = {
                let offset = 0;

                let vertex_buffer_desc = D3D11_BUFFER_DESC {
                    ByteWidth: vertex_data.len() as u32 * Triangle::size(),
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

                (buffer.unwrap(), offset)
            };

            Ok(Self {
                hwnd,
                device,
                context,
                swap_chain,
                render_view,
                vertex_shader,
                pixel_shader,
                input_layout,
                vertex_buffer,
            })
        }
    }

    pub fn new_frame(&self) -> Result<()> {
        unsafe {
            let mut rect = RECT::default();
            GetClientRect(self.hwnd, &mut rect)?;

            let viewport = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: (rect.right - rect.left) as f32,
                Height: (rect.bottom - rect.top) as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };

            self.context.RSSetViewports(Some(&[viewport]));

            self.context
                .OMSetRenderTargets(Some(&[Some(self.render_view.clone())]), None);

            self.context
                .IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            self.context.IASetInputLayout(&self.input_layout);

            self.context.VSSetShader(&self.vertex_shader, None);
            self.context.PSSetShader(&self.pixel_shader, None);

            let offset = 0;

            self.context.IASetVertexBuffers(
                0,
                1,
                Some(&Some(self.vertex_buffer.clone())),
                Some(&Triangle::stride()),
                Some(&offset),
            );

            // TODO make this dynamic
            self.context.Draw(6, 0);
        }

        Ok(())
    }
}

const D3D_LEVELS: [D3D_FEATURE_LEVEL; 1] = [D3D_FEATURE_LEVEL_11_0];
