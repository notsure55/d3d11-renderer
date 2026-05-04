use anyhow::Result;

use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*, Win32::Graphics::Direct3D::Fxc::*, Win32::Graphics::Direct3D::*,
    Win32::Graphics::Direct3D11::*, Win32::Graphics::DirectWrite::*,
    Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*, Win32::UI::WindowsAndMessaging::*,
};

use windows_core::w;

#[derive(Debug)]
pub struct TextRenderer {
    d2d_factory: ID2D1Factory,
    render_target: ID2D1RenderTarget,
    write_factory: IDWriteFactory,
    text: IDWriteTextFormat,
    brush: ID2D1SolidColorBrush,
}

impl TextRenderer {
    pub fn new(hwnd: HWND, swap_chain: &IDXGISwapChain1) -> Result<Self> {
        unsafe {
            let factory =
                D2D1CreateFactory::<ID2D1Factory>(D2D1_FACTORY_TYPE_SINGLE_THREADED, None).unwrap();

            let render_target = {
                let props = D2D1_RENDER_TARGET_PROPERTIES {
                    r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                    pixelFormat: D2D1_PIXEL_FORMAT {
                        format: DXGI_FORMAT_UNKNOWN,
                        alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                    },
                    dpiX: 0.0,
                    dpiY: 0.0,
                    usage: D2D1_RENDER_TARGET_USAGE_NONE,
                    minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
                };

                let frame_buffer = swap_chain.GetBuffer::<IDXGISurface>(0).unwrap();

                let render_target = factory
                    .CreateDxgiSurfaceRenderTarget(&frame_buffer, &props)
                    .unwrap();

                render_target
            };

            let write_factory =
                DWriteCreateFactory::<IDWriteFactory>(DWRITE_FACTORY_TYPE_SHARED).unwrap();

            let text_format = write_factory
                .CreateTextFormat(
                    w!("Gabriola"),
                    None,
                    DWRITE_FONT_WEIGHT_REGULAR,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    72.0,
                    w!("en-us"),
                )
                .unwrap();

            text_format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER);
            text_format.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER);

            let color = D2D1_COLOR_F {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            };

            let brush = render_target.CreateSolidColorBrush(&color, None).unwrap();

            Ok(Self {
                d2d_factory: factory,
                render_target,
                write_factory,
                text: text_format,
                brush,
            })
        }
    }
    pub fn draw(&self) {
        unsafe {
            self.render_target.BeginDraw();

            let layoutRect = D2D_RECT_F {
                left: 700.0,
                top: 700.0,
                right: 800.0,
                bottom: 800.0,
            };

            self.render_target.DrawText(
                w!("Hello").as_wide(),
                &self.text,
                &layoutRect,
                &self.brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }
    }
}
