use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::audio::*;

use serde::Deserialize;

use super::{
    Parser, Project, Sprite, SpriteSnapshot, Tokenizer
};

#[derive(Deserialize)]
struct StageConfig {
    backdrops: Vec<String>,
}

#[derive(Deserialize)]
struct SoundConfig {
    name: String,
    file: String,
}

#[derive(Deserialize)]
struct SpriteConfig {
    name: String,
    code: String,
    costumes: Vec<String>,
    sounds: Vec<SoundConfig>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Deserialize)]
struct ProjectConfig {
    stage: StageConfig,
    sprites: Vec<SpriteConfig>,
}

pub struct Runtime {
    pub project: Project,
}

impl Runtime {
    pub async fn new(file_path: &str) -> Self {
        let raw = std::fs::read_to_string(file_path).unwrap();
        let config: ProjectConfig = toml::from_str(&raw).unwrap();

        let mut project = Project::new();
        for sprite in config.sprites {
            let mut textures = vec![];
            for path in sprite.costumes {
                let tex = load_texture(&path).await.unwrap_or_else(|_| {
                    panic!("Failed to load texture: {}. Make sure the path is correct. Relative paths are allowed.", path)
                });
                textures.push(tex);
            }

            let mut sounds = vec![];
            for sound in sprite.sounds {
                let sound_data = load_sound(&sound.file).await.unwrap_or_else(|_| {
                    panic!("Failed to load sound: {}. Make sure the path is correct. Relative paths are allowed.", sound.file)
                });
                sounds.push((sound.name, sound_data));
            }

            let sounds = sounds.into_iter().collect::<HashMap<_, _>>();

            let code = std::fs::read_to_string(&sprite.code).expect("Failed to read sprite code");

            let mut tokenizer = Tokenizer::new(code.clone());
            let tokens = tokenizer.tokenize_full();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse();

            let s = Sprite::new(sprite.name.clone(), textures, sounds, ast, sprite.w, sprite.h, sprite.x, sprite.y);

            project.sprites.push(s);
        }
        
        for path in config.stage.backdrops {
            let tex = load_texture(&path).await.unwrap();
            project.stage.backdrops.push(tex);
        }

        Self {
            project,
        }
    }

    pub async fn run(&mut self) {
        let camera = Camera2D {
            target: vec2(0.0, 0.0),
            zoom: vec2(1.0 / screen_width(), 1.0 / screen_height()),
            ..Default::default()
        };
        loop {
            set_camera(&camera);
            clear_background(WHITE);
            
            let mut sprites = std::mem::take(&mut self.project.sprites);
            
            let snapshots: Vec<SpriteSnapshot> = sprites.iter().map(|s| s.into()).collect();

            for sprite in &mut sprites {
                sprite.step(&mut self.project, &snapshots);
            }

            self.project.sprites = sprites;
            self.project.sprites.sort_by(|a, b| a.layer.cmp(&b.layer));

            self.project.draw();

            draw_text(format!("FPS: {}", get_fps()).as_str(), -screen_width() + 20.0, -screen_height() + 70.0, 64.0, BLACK);

            next_frame().await;
        }
    }
}
