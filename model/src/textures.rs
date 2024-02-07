use super::Texture;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub struct Textures {
    pub frames: Vec<TextureFrame>,
}

impl Textures {
    pub fn get_default(&self) -> &Texture {
        self.get(TexturesGetBuilder::default())
    }

    pub fn get(
        &self,
        TexturesGetBuilder {
            frame,
            pattern_x,
            pattern_y,
            pattern_z,
            layer,
        }: TexturesGetBuilder,
    ) -> &Texture {
        let f = &self.frames[frame];
        let z = &f.patterns_z[pattern_z % f.patterns_z.len()];
        let y = &z.patterns_y[pattern_y % z.patterns_y.len()];
        let x = &y.patterns_x[pattern_x % y.patterns_x.len()];
        let l = &x.layers[layer];
        &l.texture
    }

    pub fn get_all(&self) -> Vec<Texture> {
        let mut textures = Vec::new();
        for frame in self.frames.iter() {
            for pattern_z in frame.patterns_z.iter() {
                for pattern_y in pattern_z.patterns_y.iter() {
                    for pattern_x in pattern_y.patterns_x.iter() {
                        for layer in pattern_x.layers.iter() {
                            textures.push(layer.texture.clone());
                        }
                    }
                }
            }
        }
        textures
    }
}

#[derive(Default)]
pub struct TexturesGetBuilder {
    pub frame: usize,
    pub pattern_x: usize,
    pub pattern_y: usize,
    pub pattern_z: usize,
    pub layer: usize,
}

impl TexturesGetBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn frame(mut self, frame: usize) -> Self {
        self.frame = frame;
        self
    }

    pub fn pattern_x(mut self, pattern_x: usize) -> Self {
        self.pattern_x = pattern_x;
        self
    }

    pub fn pattern_y(mut self, pattern_y: usize) -> Self {
        self.pattern_y = pattern_y;
        self
    }

    pub fn pattern_z(mut self, pattern_z: usize) -> Self {
        self.pattern_z = pattern_z;
        self
    }

    pub fn layer(mut self, layer: usize) -> Self {
        self.layer = layer;
        self
    }
}

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub struct TextureFrame {
    pub patterns_z: Vec<TexturePatternZ>,
}

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub struct TexturePatternZ {
    pub patterns_y: Vec<TexturePatternY>,
}

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub struct TexturePatternY {
    pub patterns_x: Vec<TexturePatternX>,
}

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub struct TexturePatternX {
    pub layers: Vec<TextureLayer>,
}

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub struct TextureLayer {
    pub texture: Texture,
}
