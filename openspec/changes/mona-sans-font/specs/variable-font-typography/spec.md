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

### Requirement: Timer display weight is 350
The timer countdown display SHALL render at font-weight 350, providing greater stroke presence than the previous 300 while retaining a light, elegant quality.

#### Scenario: Timer digits render at weight 350
- **WHEN** the timer window is visible at any timer state
- **THEN** the countdown digits SHALL render at font-weight 350

### Requirement: Timer display width animates with timer state
The timer countdown display SHALL animate its font-stretch between a condensed value when the timer is running and a slightly expanded value when paused or idle, using a CSS transition so the change is smooth.

#### Scenario: Font condenses when timer starts
- **WHEN** the timer transitions from paused or idle to running
- **THEN** font-stretch SHALL animate to 95% over ~400ms

#### Scenario: Font expands when timer pauses
- **WHEN** the timer transitions from running to paused
- **THEN** font-stretch SHALL animate to 103% over ~400ms

#### Scenario: Font is expanded when timer is idle
- **WHEN** the app first opens and the timer has not been started
- **THEN** font-stretch SHALL be 103%

### Requirement: Compact mode applies condensed font width
When the timer window is in compact mode, the timer display SHALL use a condensed font-stretch (85%) to fit the reduced layout, overriding the running/paused state width.

#### Scenario: Compact mode sets condensed width
- **WHEN** the window is resized below the compact threshold (w < 300 or h < 300)
- **THEN** font-stretch SHALL be 85% regardless of timer state

#### Scenario: Exiting compact mode restores state-driven width
- **WHEN** the window is resized above the compact threshold
- **THEN** font-stretch SHALL revert to the running (95%) or paused/idle (103%) value

### Requirement: Shortcut display uses Mona Sans Mono
The keyboard shortcut input component SHALL use Mona Sans Mono as its font family, maintaining the monospaced character needed for shortcut key alignment while matching the overall Mona Sans family.

#### Scenario: Shortcut keys render in monospaced Mona Sans
- **WHEN** the Shortcuts settings section displays a bound shortcut
- **THEN** the key combination text SHALL render in Mona Sans Mono
