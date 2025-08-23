# Weapon Graphics Guide

This directory should contain weapon attack effect sprites for the Bevy roguelike game.

## Required Asset Files

### Melee Weapons
- `sword_swing.png` - Curved arc showing sword motion (32x32px)
- `sword_glow.png` - Sword with glowing effect for attack animation (32x32px)

### Ranged Weapons  
- `magic_fireball.png` - Fireball or energy blast spell effect (16x16px)
- `arrow.png` - Arrow projectile for ranged attacks (16x16px)

### Magic Effects
- `spell_impact.png` - Magic spell impact/explosion effect (32x32px)
- `energy_blast.png` - Energy projectile effect (16x16px)

## Pixel Art Requirements

- **Style**: 2D pixel art suitable for roguelike games
- **Colors**: Clear, high contrast colors that stand out against dungeon backgrounds
- **Transparency**: Use PNG format with transparent backgrounds
- **Animation**: Static sprites (animation handled by code scaling/rotation)

## Color Palette Suggestions

### Sword Effects
- Blade: Light silver/white (#E8E8E8)
- Glow: Golden yellow (#FFD700)
- Motion blur: Semi-transparent white

### Magic Effects
- Fire: Orange (#FF8C00) to red (#FF0000) gradient
- Energy: Electric blue (#00BFFF) to white center
- Impact: Bright white core with colored outer ring

### Arrow
- Shaft: Brown wood (#8B4513)
- Fletching: Gray feathers (#696969)
- Arrowhead: Silver metal (#C0C0C0)

## Alternative Asset Sources

If you cannot create pixel art yourself, consider:

1. **Free Resources**:
   - OpenGameArt.org
   - itch.io (free game assets)
   - Kenney.nl (CC0 licensed assets)

2. **Pixel Art Tools**:
   - Aseprite (paid, professional)
   - GIMP (free)
   - Piskel (free, web-based)
   - Pixaki (iOS)

3. **AI Generation**:
   - Use AI image generators with prompts like "16x16 pixel art sword swing effect transparent background"
   - Post-process to ensure proper transparency and pixel perfect edges