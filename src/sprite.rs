use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use graphics::math::{Matrix2d, Scalar, Vec2d};
use graphics::types::SourceRectangle;
use graphics::{self, Graphics, ImageSize};

#[derive(Clone)]
pub struct FrameSet {
    pub repeat: bool,
    pub frame_time: f64,
    pub source: Vec<SourceRectangle>,
}

/// A sprite is a texture with some properties.
pub struct Sprite<I: ImageSize> {
    id: Uuid,

    visible: bool,

    anchor: Vec2d,

    position: Vec2d,
    rotation: Scalar,
    scale: Vec2d,
    color: [f32; 3],

    flip_x: bool,
    flip_y: bool,

    opacity: f32,

    children: Vec<Sprite<I>>,
    children_index: HashMap<Uuid, usize>,

    src_rect: Option<SourceRectangle>,
    texture: Rc<I>,

    frames: Option<FrameSet>,
    frames_followup: Option<String>,
    frame_sets: HashMap<String, FrameSet>,
    frame_idx: usize,
    frame_delta: f64,
}

impl<I: ImageSize> Sprite<I> {
    /// Crate sprite from a texture
    pub fn from_texture(texture: Rc<I>) -> Sprite<I> {
        Sprite {
            id: Uuid::new_v4(),

            visible: true,

            anchor: [0.5, 0.5],

            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],
            color: [1.0, 1.0, 1.0],

            flip_x: false,
            flip_y: false,

            opacity: 1.0,

            texture: texture,
            src_rect: None,

            frames: None,
            frames_followup: None,
            frame_idx: 0,
            frame_delta: 0.0,
            frame_sets: HashMap::new(),

            children: Vec::new(),
            children_index: HashMap::new(),
        }
    }

    /// Create sprite from a rectangle selection of a texture
    pub fn from_texture_rect(texture: Rc<I>, src_rect: SourceRectangle) -> Sprite<I> {
        Sprite {
            id: Uuid::new_v4(),

            visible: true,

            anchor: [0.5, 0.5],

            position: [0.0, 0.0],
            rotation: 0.0,
            scale: [1.0, 1.0],
            color: [1.0, 1.0, 1.0],

            flip_x: false,
            flip_y: false,

            opacity: 1.0,

            frames: None,
            frames_followup: None,
            frame_idx: 0,
            frame_delta: 0.0,
            frame_sets: HashMap::new(),

            texture: texture,
            src_rect: From::from(src_rect),

            children: Vec::new(),
            children_index: HashMap::new(),
        }
    }

    /// Get the sprite's id
    #[inline(always)]
    pub fn id(&self) -> Uuid {
        self.id.clone()
    }

    /// Whether or not the sprite is visible
    pub fn get_visible(&self) -> bool {
        self.visible
    }

    /// Set the sprite's visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Get the sprite's anchor point
    ///
    /// The value is normalized. Default value is [0.5, 0.5] (the center of texture)
    #[inline(always)]
    pub fn get_anchor(&self) -> (Scalar, Scalar) {
        (self.anchor[0], self.anchor[1])
    }

    /// Set the sprite's anchor point
    #[inline(always)]
    pub fn set_anchor(&mut self, x: Scalar, y: Scalar) {
        self.anchor = [x, y];
    }

    /// Get the sprite's position
    #[inline(always)]
    pub fn get_position(&self) -> (Scalar, Scalar) {
        (self.position[0], self.position[1])
    }

    /// Set the sprite's position
    #[inline(always)]
    pub fn set_position(&mut self, x: Scalar, y: Scalar) {
        self.position = [x, y];
    }

    /// Set the sprite's draw color (tint)
    #[inline(always)]
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b];
    }

    /// get the sprite's color.s
    #[inline(always)]
    pub fn get_color(&self) -> (f32, f32, f32) {
        (self.color[0], self.color[1], self.color[2])
    }

    /// Get the sprite's rotation (in degree)
    #[inline(always)]
    pub fn get_rotation(&self) -> Scalar {
        self.rotation
    }

    /// Set the sprite's rotation (in degree)
    #[inline(always)]
    pub fn set_rotation(&mut self, deg: Scalar) {
        self.rotation = deg;
    }

    /// Get the sprite's scale
    #[inline(always)]
    pub fn get_scale(&self) -> (Scalar, Scalar) {
        (self.scale[0], self.scale[1])
    }

    /// Set the sprite's scale
    #[inline(always)]
    pub fn set_scale(&mut self, sx: Scalar, sy: Scalar) {
        self.scale = [sx, sy];
    }

    /// Whether or not the sprite is flipped horizontally.
    ///
    /// It only flips the texture of the sprite,
    /// and not the texture of the sprite’s children.
    ///
    /// Also, flipping the texture doesn’t alter the `anchor`.
    ///
    /// If you want to flip the `anchor` too,
    /// and/or to flip the children too use: sprite.scale.x *= -1;
    #[inline(always)]
    pub fn get_flip_x(&self) -> bool {
        self.flip_x
    }

    /// Flip the sprite
    #[inline(always)]
    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x;
    }

    /// Whether or not the sprite is flipped vertically.
    ///
    /// It only flips the texture of the sprite,
    /// and not the texture of the sprite’s children.
    ///
    /// Also, flipping the texture doesn’t alter the `anchor`.
    ///
    /// If you want to flip the `anchor` too,
    /// and/or to flip the children too use: sprite.scale.y *= -1;
    #[inline(always)]
    pub fn get_flip_y(&self) -> bool {
        self.flip_y
    }

    /// Flip the sprite
    #[inline(always)]
    pub fn set_flip_y(&mut self, flip_y: bool) {
        self.flip_y = flip_y;
    }

    /// Get the sprite's opacity
    #[inline(always)]
    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }

    /// Set the sprite's opacity
    #[inline(always)]
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
    }

    /// Get the sprite's source rectangle
    #[inline(always)]
    pub fn get_src_rect(&self) -> Option<SourceRectangle> {
        self.src_rect
    }

    /// Set the sprite's source rectangle
    #[inline(always)]
    pub fn set_src_rect(&mut self, src_rect: SourceRectangle) {
        self.src_rect = From::from(src_rect);
    }

    /// Get the sprite's texture
    #[inline(always)]
    pub fn get_texture(&self) -> &Rc<I> {
        &self.texture
    }

    /// Set the sprite's texture
    #[inline(always)]
    pub fn set_texture(&mut self, texture: Rc<I>) {
        self.texture = texture;
    }

    /// Add a sprite as the child of this sprite, return the added sprite's id.
    pub fn add_child(&mut self, sprite: Sprite<I>) -> Uuid {
        let id = sprite.id();
        self.children.push(sprite);
        self.children_index
            .insert(id.clone(), self.children.len() - 1);
        id
    }

    /// Remove the child by `id` from this sprite's children or grandchild
    pub fn remove_child(&mut self, id: Uuid) -> Option<Sprite<I>> {
        if let Some(index) = self.children_index.remove(&id) {
            let removed = self.children.remove(index);
            // Removing a element of vector will alter the index,
            // update the mapping from uuid to index.
            for i in index..self.children.len() {
                let uuid = self.children[i].id();
                self.children_index.insert(uuid, i);
            }
            Some(removed)
        } else {
            for child in &mut self.children {
                if let Some(c) = child.remove_child(id.clone()) {
                    return Some(c);
                }
            }
            None
        }
    }

    /// Find the child by `id` from this sprite's children or grandchild
    pub fn child(&self, id: Uuid) -> Option<&Sprite<I>> {
        if let Some(index) = self.children_index.get(&id) {
            Some(&self.children[*index])
        } else {
            for child in &self.children {
                if let Some(c) = child.child(id.clone()) {
                    return Some(c);
                }
            }
            None
        }
    }

    /// Find the child by `id` from this sprite's children or grandchild, mutability
    pub fn child_mut(&mut self, id: Uuid) -> Option<&mut Sprite<I>> {
        if let Some(index) = self.children_index.get(&id) {
            Some(&mut self.children[*index])
        } else {
            for child in &mut self.children {
                if let Some(c) = child.child_mut(id.clone()) {
                    return Some(c);
                }
            }
            None
        }
    }

    /// Get the sprite's children
    #[inline(always)]
    pub fn children(&self) -> &Vec<Sprite<I>> {
        &self.children
    }

    /// Draw this sprite and its children
    pub fn draw<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B) {
        use graphics::*;

        if !self.visible {
            return;
        }

        let (tex_w, tex_h) = self.texture.get_size();
        let tex_w = tex_w as f64;
        let tex_h = tex_h as f64;
        let source_rectangle = match self.frames {
            None => self.src_rect.unwrap_or({
                let (w, h) = (tex_w, tex_h);
                [0.0, 0.0, w as f64, h as f64]
            }),
            Some(ref frame) => frame.source[self.frame_idx],
        };
        let anchor = [
            self.anchor[0] * source_rectangle[2],
            self.anchor[1] * source_rectangle[3],
        ];

        let transformed = t
            .trans(self.position[0], self.position[1])
            .rot_deg(self.rotation)
            .scale(self.scale[0], self.scale[1]);

        let mut model = transformed;

        if self.flip_x {
            model = model
                .trans(source_rectangle[2] - 2.0 * anchor[0], 0.0)
                .flip_h();
        }

        if self.flip_y {
            model = model
                .trans(0.0, source_rectangle[3] - 2.0 * anchor[1])
                .flip_v();
        }

        let ref draw_state: graphics::DrawState = Default::default();

        // for debug: bounding_box
        //model.rgb(1.0, 0.0, 0.0).draw(b);

        graphics::Image::new()
            .color([self.color[0], self.color[1], self.color[2], self.opacity])
            .rect([
                -anchor[0],
                -anchor[1],
                source_rectangle[2],
                source_rectangle[3],
            ])
            .maybe_src_rect(self.src_rect) // FIXME: check if this affects frame sets
            .draw(&*self.texture, draw_state, model, b);

        // for debug: anchor point
        //c.trans(self.position[0], self.position[1]).rect(-5.0, -5.0, 10.0, 10.0).rgb(0.0, 0.0, 1.0).draw(b);

        for child in &self.children {
            child.draw(transformed, b);
        }
    }

    /// Draw this sprite and its children with color
    pub fn draw_tinted<B: Graphics<Texture = I>>(&self, t: Matrix2d, b: &mut B, c: [f32; 3]) {
        use graphics::*;

        if !self.visible {
            return;
        }

        let (tex_w, tex_h) = self.texture.get_size();
        let tex_w = tex_w as f64;
        let tex_h = tex_h as f64;
        let source_rectangle = match self.frames {
            None => self.src_rect.unwrap_or({
                let (w, h) = (tex_w, tex_h);
                [0.0, 0.0, w as f64, h as f64]
            }),
            Some(ref frame) => frame.source[self.frame_idx],
        };
        let anchor = [
            self.anchor[0] * source_rectangle[2],
            self.anchor[1] * source_rectangle[3],
        ];

        let transformed = t
            .trans(self.position[0], self.position[1])
            .rot_deg(self.rotation)
            .scale(self.scale[0], self.scale[1]);

        let mut model = transformed;

        if self.flip_x {
            model = model
                .trans(source_rectangle[2] - 2.0 * anchor[0], 0.0)
                .flip_h();
        }

        if self.flip_y {
            model = model
                .trans(0.0, source_rectangle[3] - 2.0 * anchor[1])
                .flip_v();
        }

        let ref draw_state: graphics::DrawState = Default::default();

        // for debug: bounding_box
        //model.rgb(1.0, 0.0, 0.0).draw(b);

        graphics::Image::new()
            .color([c[0], c[1], c[2], self.opacity])
            .rect([
                -anchor[0],
                -anchor[1],
                source_rectangle[2],
                source_rectangle[3],
            ])
            .maybe_src_rect(self.src_rect) // FIXME: check if this affects frame sets
            .draw(&*self.texture, draw_state, model, b);

        // for debug: anchor point
        //c.trans(self.position[0], self.position[1]).rect(-5.0, -5.0, 10.0, 10.0).rgb(0.0, 0.0, 1.0).draw(b);

        for child in &self.children {
            child.draw_tinted(transformed, b, c);
        }
    }

    /// Update the frame delta and act accordingly
    pub fn update(&mut self, dt: f64) {
        if self.frames.is_some() {
            let mut followup: Option<String> = None;
            if let Some(ref frame) = self.frames {
                self.frame_delta += dt;
                if self.frame_delta > frame.frame_time {
                    self.frame_delta = 0.0;
                    if self.frame_idx == frame.source.len() - 1 {
                        if let Some(ref next) = self.frames_followup {
                            self.frame_idx = 0;
                            followup = Some(next.clone());
                        }
                        if frame.repeat {
                            self.frame_idx = 0;
                        }
                    }
                }
            }
            if let Some(next) = followup {
                self.play(&next, None);
            }
        }
    }

    /// Start an animated frameset
    pub fn play(&mut self, name: &str, followup: Option<&str>) {
        if self.frame_sets.contains_key(name) {
            self.frames = Some(self.frame_sets.get(name).unwrap().clone());
        }
        match followup {
            None => self.frames_followup = None,
            Some(next) => self.frames_followup = Some(next.to_owned()),
        }
    }

    /// Add an animated frameset
    pub fn add_frameset(
        &mut self,
        name: &str,
        repeat: bool,
        frame_time: f64,
        source: Vec<SourceRectangle>,
    ) {
        if !self.frame_sets.contains_key(name) {
            self.frame_sets.insert(
                name.to_owned(),
                FrameSet {
                    repeat: repeat,
                    frame_time: frame_time,
                    source: source,
                },
            );
        }
    }

    /// Add an horizontal aligned animated frameset
    pub fn add_frameset_horizontal(
        &mut self,
        name: &str,
        repeat: bool,
        frame_time: f64,
        source: SourceRectangle,
        count: u32,
    ) {
        let mut srcs: Vec<SourceRectangle> = Vec::new();
        for i in 0..count {
            let x = i as f64 * source[2];
            srcs.push([x, source[1], source[2], source[3]]);
        }
        self.add_frameset(name, repeat, frame_time, srcs);
    }

    /// Get the sprite's bounding box
    pub fn bounding_box(&self) -> graphics::types::Rectangle {
        let (w, h) = self.texture.get_size();
        let source_rectangle = match self.frames {
            None => self.src_rect.unwrap_or({
                let (w, h) = (w, h);
                [0.0, 0.0, w as f64, h as f64]
            }),
            Some(ref frame) => frame.source[self.frame_idx],
        };
        let sprite_w = source_rectangle[2] * self.scale[0];
        let sprite_h = source_rectangle[3] * self.scale[1];

        [
            self.position[0] - self.anchor[0] * sprite_w,
            self.position[1] - self.anchor[1] * sprite_h,
            sprite_w,
            sprite_h,
        ]
    }
}
