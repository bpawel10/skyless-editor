use super::TfsProject;
use crate::{
    load::{self, Error, Load},
    parse::{self, dat, otb, otbm, spr, Parse},
    transport::Transport,
};
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine};
use futures::{
    future::{join_all, try_join_all},
    Future,
};
use image::{imageops::overlay, DynamicImage, GenericImage, ImageOutputFormat, RgbaImage};
use itertools::Itertools;
use model::{
    attributes, Attribute, Entity, Item, Position, Texture, TextureFrame, TextureLayer,
    TexturePatternX, TexturePatternY, TexturePatternZ, Textures, Tile,
};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs::File,
    io::Cursor,
    pin::{pin, Pin},
    sync::Arc,
};
use transport::Message;

#[async_trait]
impl Load for TfsProject {
    async fn load(
        &self,
        transport: Arc<impl Transport + Send + Sync + 'static>,
    ) -> Result<(), Error> {
        println!("Loading SPR");
        let spr: spr::Document = File::open(&self.spr_path)?.parse()?;
        println!("Loading DAT");
        let dat: dat::Document = File::open(&self.dat_path)?.parse()?;
        println!("Loading OTB");
        let otb: otb::Document = File::open(&self.otb_path)?.parse()?;
        println!("Loading OTBM");
        let otbm: otbm::Document = otbm::Reader {
            path: &self.otbm_path,
            dat: &dat,
            otb: &otb,
        }
        .parse()?;

        let tiles: Vec<_> = otbm.map.tiles;

        println!("Sending items");

        transport
            .transport(Message::ItemsCount(dat.items.len()))
            .await;

        let items_vec: Vec<_> = dat.items.into_values().collect();
        items_vec.into_par_iter().chunks(100).for_each(|items| {
            let transport = transport.clone();
            let items = items
                .into_iter()
                .map(|item| Item {
                    id: item.id,
                    name: None,
                    ground: item.ground,
                    stackable: item.stackable,
                    splash: item.splash,
                    fluid_container: item.fluid_container,
                    draw_offset: item.draw_offset,
                    height_offset: item.height_offset,
                    textures: Self::get_item_textures(&item.textures, &spr.sprites),
                })
                .collect();
            tauri::async_runtime::spawn(async move {
                transport.transport(Message::Items(items)).await;
            });
        });

        let tiles_len = tiles.len();

        println!("Sending map, tiles: {}", tiles_len);

        transport.transport(Message::MapTilesCount(tiles_len)).await;

        tiles.into_par_iter().chunks(10_000).for_each(|tiles| {
            let transport = transport.clone();
            let tiles = tiles
                .into_iter()
                .map(
                    |otbm::Tile {
                         position: otbm::Position { x, y, z, .. },
                         things,
                     }| {
                        (
                            Position(x, y, z),
                            Tile {
                                entities: things.into_iter().map(Self::thing_to_entity).collect(),
                                ..Default::default()
                            },
                        )
                    },
                )
                .collect();
            tauri::async_runtime::spawn(async move {
                transport.transport(Message::MapTiles(tiles)).await;
            });
        });

        Ok(())
    }
}

impl TfsProject {
    fn get_item_textures(
        textures: &dat::Textures,
        sprites: &HashMap<u16, spr::SpriteBytes>,
    ) -> Textures {
        let mut sprite_index = 0;
        let mut frames = Vec::new();
        for _ in 0..textures.frames {
            let mut patterns_z = Vec::new();
            for _ in 0..textures.patterns_z {
                let mut patterns_y = Vec::new();
                for _ in 0..textures.patterns_y {
                    let mut patterns_x = Vec::new();
                    for _ in 0..textures.patterns_x {
                        let mut image = DynamicImage::ImageRgba8(RgbaImage::new(
                            (textures.width as usize * spr::SPRITE_SIZE)
                                .try_into()
                                .unwrap(),
                            (textures.height as usize * spr::SPRITE_SIZE)
                                .try_into()
                                .unwrap(),
                        ));
                        let mut layers = Vec::new();

                        for _ in 0..textures.layers {
                            for height in 0..textures.height {
                                for width in 0..textures.width {
                                    let sprite_id = textures.sprites[sprite_index];
                                    sprite_index += 1;
                                    let sprite = if sprite_id >= 2
                                        && sprite_id as usize <= sprites.len() + 2
                                    {
                                        sprites.get(&sprite_id)
                                    } else {
                                        None
                                    };

                                    if let Some(sprite_pixels) = sprite.cloned() {
                                        let sprite_image = DynamicImage::ImageRgba8(
                                            RgbaImage::from_raw(
                                                spr::SPRITE_SIZE.try_into().unwrap(),
                                                spr::SPRITE_SIZE.try_into().unwrap(),
                                                From::from(sprite_pixels),
                                            )
                                            .unwrap(),
                                        );

                                        overlay(
                                            &mut image,
                                            &sprite_image,
                                            ((textures.width - width - 1) as usize
                                                * spr::SPRITE_SIZE)
                                                .try_into()
                                                .unwrap(),
                                            ((textures.height - height - 1) as usize
                                                * spr::SPRITE_SIZE)
                                                .try_into()
                                                .unwrap(),
                                        )
                                    }
                                }
                            }
                        }

                        let mut buffer = Cursor::new(Vec::new());
                        image.write_to(&mut buffer, ImageOutputFormat::Png);
                        let image_base64 = general_purpose::STANDARD.encode(buffer.get_ref());

                        layers.push(TextureLayer {
                            texture: Texture {
                                width: image.width().try_into().unwrap(),
                                height: image.height().try_into().unwrap(),
                                image: image_base64,
                                rgba_bytes: image.into_bytes(),
                            },
                        });
                        patterns_x.push(TexturePatternX { layers });
                    }
                    patterns_y.push(TexturePatternY { patterns_x });
                }
                patterns_z.push(TexturePatternZ { patterns_y });
            }
            frames.push(TextureFrame { patterns_z });
        }
        Textures { frames }
    }

    fn thing_to_entity(otbm::Thing { attributes }: otbm::Thing) -> Entity {
        let mut entity = Entity::new();
        for attribute in attributes.into_iter() {
            match attribute {
                otbm::Attribute::Item(id) => {
                    entity
                        .attributes
                        .insert("item".to_string(), Attribute::Item(attributes::Item(id)));
                }
                otbm::Attribute::Container(things) => {
                    entity.attributes.insert(
                        "container".to_string(),
                        Attribute::Container(attributes::Container(
                            things.into_iter().map(Self::thing_to_entity).collect(),
                        )),
                    );
                }
                otbm::Attribute::Count(count) => {
                    entity.attributes.insert(
                        "count".to_string(),
                        Attribute::Count(attributes::Count(count)),
                    );
                }
                otbm::Attribute::Fluid(fluid) => {
                    entity.attributes.insert(
                        "fluid".to_string(),
                        Attribute::Fluid(attributes::Fluid(fluid)),
                    );
                }
            }
        }
        entity
    }
}
