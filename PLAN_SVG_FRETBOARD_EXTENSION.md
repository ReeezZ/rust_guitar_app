# Plan: Extending SVG Fretboard with Note Knowledge and Scale Display

## Overview

The goal is to extend the current SVG fretboard (`SvgFretboard`) to support:

1. **Interactive fret clicking** with callback events containing fret coordinates
2. **Note mapping** from fret coordinates to actual musical notes
3. **Scale display** capability similar to the existing `FretboardScaleDisplay` component
4. **Clean separation of concerns** between the core SVG fretboard rendering and the musical logic

## Current State Analysis

### Existing Components
- **`SvgFretboard`**: Pure visual fretboard component focused on rendering and zoom functionality
- **`FretboardScaleDisplay`**: Uses the old HTML/CSS fretboard with `FretboardModel` for scale visualization
- **`Fretboard` (HTML/CSS version)**: Has interactive clicking with `FretClickEvent` callbacks

### Music Library Capabilities
- **`Note`**: Enum with `add_steps()` method for chromatic progression
- **`Scale`**: Supports different scale types with `contains_note()` method
- **`FretboardModel`**: Manages fret states, tuning, and note-to-coordinate mapping

### Key Interfaces from Existing Code
```rust
// From the HTML/CSS fretboard
#[derive(Clone, Copy, Debug)]
pub struct FretClickEvent {
  pub note: Note,
  pub coord: FretCoord,
  pub fret_state: FretState,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FretCoord {
  pub string_idx: u8,
  pub fret_idx: u8,
}
```

## Proposed Architecture

### Phase 1: Make SVG Fretboard Interactive

#### 1.1 Add Click Event Support to SvgFretboard
- Add `on_fret_clicked` callback prop to `SvgFretboard`
- Create clickable overlay areas for each fret position
- Emit fret coordinates (string_idx, fret_idx) when clicked

#### 1.2 Create FretClickEvent for SVG Context
```rust
#[derive(Clone, Copy, Debug)]
pub struct SvgFretClickEvent {
  pub coord: FretCoord,
  // Note: No note or fret_state here - that's for wrapper components
}
```

#### 1.3 Implementation Details
- Add invisible clickable `<rect>` elements over each fret position
- Calculate proper string and fret indices from click coordinates
- Handle the nut (fret 0) clicks separately
- Respect the zoom/visible range when calculating coordinates

### Phase 2: Create Note-Aware Wrapper Component

#### 2.1 Create SvgFretboardWithNotes Component
```rust
#[component]
pub fn SvgFretboardWithNotes(
  // All SvgFretboard props passed through
  start_fret: Signal<usize>,
  end_fret: Signal<usize>,
  
  // New props for musical functionality
  #[prop(optional)] tuning: Option<Signal<Vec<Note>>>,
  #[prop(optional)] on_note_clicked: Option<Callback<FretClickEvent>>,
  
  // Pass through all other SvgFretboard optional props
  #[prop(optional, into)] num_strings: Option<Signal<u8>>,
  // ... other props
) -> impl IntoView
```

#### 2.2 Note Mapping Logic
- Use standard tuning by default: `[E, A, D, G, B, E]`
- Calculate note from coordinates: `tuning[string_idx].add_steps(fret_idx)`
- Convert `SvgFretClickEvent` to `FretClickEvent` with note information

### Phase 3: Create Scale Display Wrapper

#### 3.1 Create SvgFretboardScaleDisplay Component
```rust
#[component]
pub fn SvgFretboardScaleDisplay(
  // Scale configuration
  #[prop()] root_note: ReadSignal<Note>,
  #[prop()] scale_type: ReadSignal<ScaleType>,
  
  // Fretboard configuration
  start_fret: Signal<usize>,
  end_fret: Signal<usize>,
  #[prop(optional)] num_frets: Option<ReadSignal<u8>>,
  
  // Optional callback for interactions
  #[prop(optional)] on_fret_clicked: Option<Callback<FretClickEvent>>,
  
  // All other SvgFretboard props passed through
) -> impl IntoView
```

#### 3.2 Scale Visualization Strategy
Two possible approaches:

**Option A: Note Overlay System**
- Render `SvgFretboardWithNotes` as base
- Add SVG overlay circles/dots for scale notes
- Different visual styles for root notes vs. other scale notes

**Option B: Integration with Existing FretState System**
- Create a lightweight state management system similar to `FretboardModel`
- Use existing `FretState` and `FretStateColor` enums
- Render colored circles directly in the SVG

#### 3.3 Visual Design
- Root note: Red circle with note name
- Other scale notes: Blue/green circles with note names
- Non-scale notes: Hidden or very faint
- Hover effects for interactive feedback

### Phase 4: Component Hierarchy and Integration

#### 4.1 Component Relationships
```
SvgFretboard (base, visual only)
├── SvgFretboardWithNotes (adds note awareness)
    ├── SvgFretboardScaleDisplay (adds scale visualization)
```

#### 4.2 Props Flow
- Base visual props flow down to `SvgFretboard`
- Musical props (tuning, scales) handled at appropriate levels
- Callbacks bubble up and get enriched with note information

#### 4.3 Backward Compatibility
- Existing `SvgFretboard` usage remains unchanged
- New components are additive, not replacing

## Implementation Steps

### Step 1: Basic Interactivity
1. Add clickable overlay system to `SvgFretboard`
2. Implement coordinate calculation from click positions
3. Add `on_fret_clicked` callback prop
4. Test with simple coordinate logging

### Step 2: Note Mapping
1. Create `SvgFretboardWithNotes` component
2. Implement note calculation logic
3. Create enriched `FretClickEvent` with note information
4. Test note accuracy across all strings and frets

### Step 3: Scale Display
1. Choose visualization approach (overlay vs. integrated)
2. Create `SvgFretboardScaleDisplay` component
3. Implement scale note highlighting
4. Add visual differentiation for root notes

### Step 4: Integration and Polish
1. Update existing pages to use new components
2. Add comprehensive documentation
3. Performance optimization
4. Visual refinements and animations

## Technical Considerations

### Coordinate Mapping Challenges
- SVG coordinates need to map correctly to fret positions
- Zoom and pan functionality must be preserved
- Click detection accuracy across different screen sizes

### Performance Considerations
- Avoid recreating heavy calculations on every render
- Use memos for expensive coordinate transformations

### State Management
- Keep musical state separate from visual state
- Use signals appropriately for reactive updates
- Avoid prop drilling through multiple component layers

### Testing Strategy
- Unit tests for coordinate mapping functions
- Component tests for click event handling
- Integration tests for scale display accuracy
- Visual regression tests for rendering consistency

## Future Enhancements

Out of scope for now, but should be considered.

### Advanced Musical Features
- Interval highlighting (show intervals from a selected note)
- Chord visualization
- Custom tuning support (7-string, bass, etc.)

### User Experience Improvements
- Animation effects for note appearance/disappearance
- Audio feedback on note clicks
- Touch/mobile optimization

### Educational Features
- Note name display options (sharps vs. flats preference)
- Scale degree numbers (1, 2, 3, etc.)
- Mode visualization (relative to scale degrees)
- Practice mode with note hiding/revealing

## Migration Path

### For Existing Code
1. Keep current `FretboardScaleDisplay` working during development
2. Create new SVG-based version alongside existing components
3. Gradually migrate pages to use SVG version
  3.1 Check if we still require all pages. 
4. Deprecate old HTML/CSS version once feature parity achieved

### API Compatibility
- Maintain similar prop interfaces where possible
- Use same `Note`, `Scale`, and coordinate types
- Keep callback signatures compatible
- Provide migration guide for users

## Conclusion

This plan provides a structured approach to extending the SVG fretboard with musical intelligence while maintaining clean separation of concerns. The phased approach allows for incremental development and testing, ensuring each layer works correctly before building the next one.

The key insight is to build layers of abstraction:
1. **Visual layer**: Pure SVG rendering (already exists)
2. **Interactive layer**: Click detection and coordinate mapping
3. **Musical layer**: Note awareness and scale visualization
4. **Application layer**: Specific use cases and integrations

This architecture will be flexible enough to support future enhancements while keeping the codebase maintainable and testable.
