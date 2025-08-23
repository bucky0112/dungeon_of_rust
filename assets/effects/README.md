# Effects Directory

This directory is for visual effect sprites and animations.

## Planned Assets

### Impact Effects
- `sword_impact.png` - Sparkles/clash effect when sword hits something
- `magic_explosion.png` - Burst effect when magic projectile hits
- `arrow_hit.png` - Small dust/debris when arrow impacts

### Trail Effects  
- `magic_trail.png` - Glowing trail behind magic projectiles
- `arrow_trail.png` - Subtle streak behind fast arrows

### UI Effects
- `damage_number.png` - Template for floating damage numbers
- `heal_effect.png` - Green plus or heart for healing effects

## Implementation Notes

The current weapon system in `/src/systems/attack.rs` includes:

1. **WeaponEffect Component**: Tracks lifetime and animation type
2. **Automatic Asset Loading**: Checks for sprite files and falls back to shapes
3. **Animation Systems**: 
   - Sword: Scaling and fade-out animation
   - Magic: Rotation and flicker effects  
   - Arrow: Directional rotation
4. **Projectile Movement**: Physics-based projectile motion for ranged weapons

## Integration Tips

- Effects should complement the 16x16 or 32x32 pixel character sprites
- Use transparent backgrounds (PNG format)
- Consider the game's color palette for consistency
- Test effects against dark dungeon backgrounds for visibility