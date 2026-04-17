## ADDED Requirements

### Requirement: Mona Sans is embedded and applied globally

The app SHALL embed Mona Sans as a variable font and apply it as the primary font family on the document body, so all text inherits it without per-component overrides.

#### Scenario: Font renders without network access

- **WHEN** the app is launched with no internet connection
- **THEN** all UI text SHALL render in Mona Sans (not a system fallback)

#### Scenario: Font family cascades to all components

- **WHEN** any component renders text without an explicit font-family override
- **THEN** that text SHALL render in Mona Sans

### Requirement: Optical sizing adapts automatically to font size

The app SHALL enable `font-optical-sizing: auto` globally so the optical-size axis adjusts letterform contrast and spacing based on each element's rendered font size, including SVG text elements in the stats views.

#### Scenario: Large display text uses display-optimized forms

- **WHEN** the timer display renders at ~2.8rem
- **THEN** the font SHALL use high optical-size letterforms (tighter, higher contrast)

#### Scenario: Small SVG labels use body-optimized forms

- **WHEN** chart axis labels render at 8–9px
- **THEN** the font SHALL use low optical-size letterforms (more open, better legibility)

### Requirement: Shortcut display uses Mona Sans Mono

The keyboard shortcut input component SHALL use Mona Sans Mono as its font family, maintaining the monospaced character needed for shortcut key alignment while matching the overall Mona Sans family.

#### Scenario: Shortcut keys render in monospaced Mona Sans

- **WHEN** the Shortcuts settings section displays a bound shortcut
- **THEN** the key combination text SHALL render in Mona Sans Mono
