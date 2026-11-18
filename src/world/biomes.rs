use bevy::prelude::*;

use crate::world::biomes::BiomeType::{Desert, Mountains, Ocean, Plains, SpikyMountains};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BiomeType {
    Plains,
    Desert,
    Ocean,
    Mountains,
    SpikyMountains,
}
#[derive(Clone)]
pub struct Biome {
    pub name: BiomeType,
    pub target_noise: f32, // Wo liegt es im Biom Noise, von -1 bis 1
    pub base_height: f32,
    pub amplitude: f32, // Wie hoch sind Berge
    pub feature_frequency: f32, // Wie oft kommen berge
    pub ridge_factor: f32, // 0.0 weiche Hügel, 1.0 scharfte Spitzen
}

impl Biome {
    // Tarrainhöhe für das Biom
    pub fn calculate_height(
        &self,
        base_noise: f32,
        detail_noise: f32,
        large_noise: f32,
    ) -> f32 {
        match self.name {
            Mountains | SpikyMountains => {
                // Große zusammenhängende Gebirgsmassen
                let mass = (large_noise * 0.5 + 0.5).powf(1.5);
    
                // Mittlere Strukturen innerhalb der Gebirge
                let medium = (base_noise * 0.5 + 0.5);
    
                // Kleine Oberflächendetails
                let detail = detail_noise;
    
                // Große Massen dominieren die Landschaft
                let mountain_shape =
                    mass * 0.75 +
                    medium * 0.20 +
                    detail * 0.05;
    
                // Spiky Mountains bekommen etwas mehr Ridge-Struktur
                let ridge = if self.name == SpikyMountains {
                    (1.0 - detail_noise.abs()).powf(2.0)
                } else {
                    0.0
                };
    
                let shape = mountain_shape + ridge * self.ridge_factor * 0.2;
    
                self.base_height + shape * self.amplitude
            }
    
            _ => {
                let blended_noise = lerp(
                    base_noise,
                    detail_noise,
                    self.ridge_factor
                );
    
                self.base_height + blended_noise * self.amplitude
            }
        }
    }
}
#[derive(Clone)]
pub struct BiomeRegistry {
    pub biomes: Vec<Biome>,
}
// Das muss besser gemacht werden!!
impl BiomeRegistry {
    pub fn new() -> Self {
        let mut biomes = vec![
            Biome {
                name: Plains,
                target_noise: 0.0,
                base_height: 50.,
                amplitude: 20.,
                feature_frequency: 0.002,
                ridge_factor: 0.1,
            },
            Biome {
                name: Mountains,
                target_noise: 0.6,
                base_height: 70.,
                amplitude: 45.,
                feature_frequency: 0.002,
                ridge_factor: 0.5,
            },
            Biome {
                name: SpikyMountains,
                target_noise: 0.9,
                base_height: 85.,
                amplitude: 55.,
                feature_frequency: 0.002,
                ridge_factor: 0.8,
            },
        ];
        // Nach dem Target Noise sortieren
        biomes.sort_by(|a, b| a.target_noise.partial_cmp(&b.target_noise).unwrap());
        Self {
            biomes
        }
    }
    // Nächsten beiden Biome + t
    pub fn sample(&self, noise_val: f32) -> (&Biome, &Biome, f32) {
        let val = noise_val.clamp(-1.0, 1.0);

        // 1. Check für außerhalb des Spektrums
        if val <= self.biomes.first().unwrap().target_noise {
            return (&self.biomes[0], &self.biomes[0], 0.0);
        }
        if val >= self.biomes.last().unwrap().target_noise {
            let last = self.biomes.len() - 1;
            return (&self.biomes[last], &self.biomes[last], 0.0);
        }

        for i in 0..self.biomes.len() - 1 {
            let b1 = &self.biomes[i];
            let b2 = &self.biomes[i + 1];

            if val >= b1.target_noise && val <= b2.target_noise {
                let range = b2.target_noise - b1.target_noise;
                let raw_t = (val - b1.target_noise) / range;
                let t = smoothstep(raw_t); // S-Kurve
                return (b1, b2, t)
            }
        }
        (&self.biomes[0], &self.biomes[0], 0.0)
    }
}

pub fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
