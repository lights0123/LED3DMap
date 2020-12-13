use image::{ImageBuffer, Luma, Rgba};
use imageproc::definitions::Image;
use js_sys::{Error as JsError, Uint8Array};
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
use serde::Serialize;
use std::iter::once;
use wasm_bindgen::prelude::*;
use wasm_typescript_definition::TypescriptDefinition;

mod utils;

fn data_to_img(width: u32, height: u32, img: Uint8Array) -> Image<Rgba<u8>> {
    let data: Vec<u8> = img.to_vec();
    ImageBuffer::from_vec(width, height, data).unwrap()
}

#[derive(Serialize, TypescriptDefinition)]
#[serde(rename_all = "camelCase")]
struct FrameInfo {
    x: u64,
    y: u64,
    max_brightness: u8,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "FrameInfo")]
    pub type FrameInfoJs;
}

#[wasm_bindgen]
pub struct BaseImage {
    image: Image<Luma<u8>>,
}

#[wasm_bindgen]
impl BaseImage {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, img: Uint8Array) -> BaseImage {
        BaseImage {
            image: led_map::read_base_frame(&data_to_img(width, height, img)),
        }
    }

    pub fn from_init(width: u32, height: u32, img: Uint8Array) -> BaseImage {
        BaseImage {
            image: ImageBuffer::from_vec(width, height, img.to_vec()).unwrap(),
        }
    }

    pub fn compute_frame(&self, width: u32, height: u32, img: Uint8Array) -> FrameInfoJs {
        serde_wasm_bindgen::to_value(
            &led_map::compute_light_pos(&self.image, &data_to_img(width, height, img))
                .map(|(x, y, max_brightness)| FrameInfo {
                    x,
                    y,
                    max_brightness,
                })
                .unwrap_or(FrameInfo {
                    x: 0,
                    y: 0,
                    max_brightness: 0,
                }),
        )
        .unwrap()
        .into()
    }

    pub fn inner(&self) -> Uint8Array {
        (&*self.image).into()
    }
}

#[wasm_bindgen]
pub fn lin_reg(x: Vec<f64>, y: Vec<f64>) -> Result<f64, JsValue> {
    let data = RegressionDataBuilder::new()
        .build_from(once(("x", x)).chain(once(("y", y))))
        .map_err(|e| JsError::new(&e.to_string()))?;
    let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula("x ~ y")
        .fit()
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(model.parameters.regressor_values[0])
}
