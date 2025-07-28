# Migration TODO - Simplified Plan

## DELETE (don't migrate):
- **`src/pages/guitar_v1.rs`** - Legacy hardcoded page (38 lines, German title, no interaction)
- **`src/pages/fretboard_playground.rs`** - Testing page (62 lines, "for testing features and ideas")  
- **`src/pages/fretboard_viewer.rs`** - Inferior to `svg_fretboard_scale.rs` (same functionality, worse UI)
- **`src/components/fretboard_scale_display.rs`** - Will be unused after deletions

## MIGRATE:
- **`src/pages/fretboard_trainer.rs`** - The only one worth keeping (interval training)

## REMOVE AFTER MIGRATION:
- `src/components/fretboard.rs` - Old fretboard component
- Update navbar to remove deleted page links
- Consider if `src/models/fretboard_model.rs` is still needed

## Migration Approach for Trainer

**Abandon `FretState::Colored` pattern** - it mixes concerns:
- ❌ Model controls visual presentation (Red/Green/Blue colors)
- ❌ Limited to 3 hardcoded colors
- ❌ Doesn't match SVG overlay approach

**Use SVG overlay approach instead:**
```rust
// Old way (mixed concerns)
model.set_fret_state(coord, FretState::Colored(FretStateColor::Green));

// New way (separated concerns)
let reference_note_coord = signal(Some(coord));
let error_coords = signal(vec![coord1, coord2]);

<SvgFretboardTrainer 
  reference_note=reference_note_coord
  error_notes=error_coords 
/>
```

## Benefits
- ✅ Cleaner architecture (model = logic, component = visuals)
- ✅ Consistent with existing SVG components  
- ✅ More flexible styling options
- ✅ Much less code to maintain (1 migration vs 4)
