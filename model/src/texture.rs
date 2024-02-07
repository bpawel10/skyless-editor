use rkyv::{Archive, Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub struct Texture {
    pub width: u16,
    pub height: u16,
    #[wasm_bindgen(getter_with_clone)]
    pub image: String,
    #[wasm_bindgen(skip)]
    pub rgba_bytes: Vec<u8>,
}

impl Texture {
    fn pixels(&self) -> Vec<Pixel> {
        self.rgba_bytes
            .chunks(4)
            .map(|pixel| TryInto::<[u8; 4]>::try_into(pixel).unwrap().into())
            .collect::<Vec<Pixel>>()
    }

    pub fn rgba_bytes(&self, padding: usize) -> Vec<u8> {
        let mut pixels_grid: Vec<_> = self
            .pixels()
            .chunks(self.width as usize)
            .map(|row| row.to_vec())
            .collect();

        let width = self.width + (padding * 2) as u16;
        let height = self.height + (padding * 2) as u16;

        let first_row = pixels_grid.first().unwrap().clone();
        let last_row = pixels_grid.last().unwrap().clone();

        pixels_grid.splice(0..0, vec![first_row; padding]);
        pixels_grid.append(&mut vec![last_row; padding]);

        for row in 0..pixels_grid.len() {
            let first_pixel_in_row = pixels_grid[row].first().unwrap().clone();
            let last_pixel_in_row = pixels_grid[row].last().unwrap().clone();

            pixels_grid[row].splice(0..0, vec![first_pixel_in_row; padding]);
            pixels_grid[row].append(&mut vec![last_pixel_in_row; padding]);
        }

        let rgba_bytes = pixels_grid
            .into_iter()
            .flatten()
            .map(|pixel| Into::<[u8; 4]>::into(pixel).to_vec())
            .flatten()
            .collect();

        rgba_bytes
    }
}

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl From<Pixel> for [u8; 4] {
    fn from(
        Pixel {
            red,
            green,
            blue,
            alpha,
        }: Pixel,
    ) -> Self {
        [red, green, blue, alpha]
    }
}

impl Into<Pixel> for [u8; 4] {
    fn into(self) -> Pixel {
        Pixel {
            red: self[0],
            green: self[1],
            blue: self[2],
            alpha: self[3],
        }
    }
}
