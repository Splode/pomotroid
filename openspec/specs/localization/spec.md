## Requirements

### Requirement: Paraglide-based message catalog

The system SHALL use Paraglide JS v2 (`@inlang/paraglide-js`) to manage all user-visible strings. All strings SHALL be defined in message files (`messages/<locale>.json`) and accessed through generated type-safe message functions (`m.<key>()`). The base locale SHALL be `en`.

#### Scenario: Type-safe message access

- **WHEN** a developer references a message key that does not exist
- **THEN** TypeScript SHALL report a compile error

#### Scenario: Unused message tree-shaking

- **WHEN** the app is built for production
- **THEN** message functions for unused keys SHALL be eliminated from the bundle

### Requirement: Supported locales at launch

The system SHALL ship eight locales: English (`en`, base), Spanish (`es`), French (`fr`), German (`de`), Japanese (`ja`), Chinese Simplified (`zh`), Portuguese (`pt`), and Turkish (`tr`). Non-English locales MAY be machine-translated. All locale message files SHALL contain translations for every key defined in `messages/en.json`.

#### Scenario: All keys present in non-English locales

- **WHEN** a non-English message file is loaded
- **THEN** every key defined in `messages/en.json` SHALL have a corresponding entry

#### Scenario: Fallback to English for missing keys

- **WHEN** a message key is missing in the active locale's file
- **THEN** the English string SHALL be displayed as a fallback

#### Scenario: Turkish locale is selectable

- **WHEN** the user opens the language picker in Settings → System
- **THEN** Turkish SHALL appear as an option and selecting it SHALL display all UI strings in Turkish

### Requirement: Automatic locale detection

The system SHALL default to `language = 'auto'`. When `'auto'` is active, the locale SHALL be resolved from `navigator.language` by matching the closest supported locale (prefix match). If no match is found, the locale SHALL fall back to `en`.

#### Scenario: Exact locale match

- **WHEN** `navigator.language` is `'fr'`
- **THEN** the active locale SHALL be `fr`

#### Scenario: Region-qualified locale match

- **WHEN** `navigator.language` is `'de-AT'`
- **THEN** the active locale SHALL be `de`

#### Scenario: Unsupported locale fallback

- **WHEN** `navigator.language` is `'zh-CN'`
- **THEN** the active locale SHALL fall back to `en`

### Requirement: User language override

The system SHALL allow the user to override the detected locale via a language dropdown in the System settings section. The selected locale SHALL be persisted as the `language` setting and applied immediately without requiring an app restart.

#### Scenario: User selects a specific language

- **WHEN** the user selects `'fr'` from the language dropdown
- **THEN** all UI strings in both windows SHALL immediately display in French

#### Scenario: User resets to automatic detection

- **WHEN** the user selects `'Auto'` from the language dropdown
- **THEN** `language` is saved as `'auto'` and the locale is re-resolved from `navigator.language`

### Requirement: Locale applied in both windows

The system SHALL apply the active locale in both the main timer window and the settings window. When the `language` setting changes, both windows SHALL re-call `setLocale()` in response to the `settings:changed` event.

#### Scenario: Language change propagates to both windows

- **WHEN** the user changes the language setting while the settings window is open
- **THEN** both the timer window and the settings window SHALL update their displayed strings

### Requirement: Tooltip strings in all locales

All tooltip i18n keys SHALL be present in every supported locale message file. Non-English translations MAY be machine-translated. The following keys are defined:

| Key                             | English value                                                                                                                    |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `tooltip_settings`              | `"Open Settings"`                                                                                                                |
| `tooltip_statistics`            | `"Open Statistics"`                                                                                                              |
| `tooltip_restart_round`         | `"Restart the current round from the beginning."`                                                                                |
| `tooltip_skip`                  | `"Skip to the next round."`                                                                                                      |
| `tooltip_reset`                 | `"Reset the timer to the first work round. Current session progress will be cleared."`                                           |
| `tooltip_mute`                  | `"Mute alert sounds"`                                                                                                            |
| `tooltip_unmute`                | `"Unmute alert sounds"`                                                                                                          |
| `tooltip_round_counter`         | `"Current work round out of the total rounds before a long break."`                                                              |
| `tooltip_round_counter_session` | `"Continuous session round count. Resets only when the timer is reset."`                                                         |
| `tooltip_verbose_logging`       | `"Enables detailed debug logging. Use when reporting issues. Log files are accessible via Open Log Folder in Settings → About."` |
| `tooltip_websocket`             | `"Enables a local WebSocket server for external integrations such as stream overlays. Disabled by default."`                     |

Note: `system_tray_gnome_hint` is reused by the Linux-only `TooltipInfo` icon on the System Tray toggle; it predates this feature and requires no new key.

#### Scenario: All tooltip keys present in non-English locales

- **WHEN** a non-English message file is loaded
- **THEN** every tooltip key listed above SHALL have a corresponding entry in that file

---

### Requirement: Translated desktop notifications

The system SHALL send desktop notifications with titles and bodies constructed from translated Paraglide message strings. Notification string construction SHALL happen on the frontend; the Rust backend SHALL receive a pre-translated `title` and `body`.

#### Scenario: Work round complete notification in active locale

- **WHEN** a work round completes and the active locale is `fr`
- **THEN** the notification title and body SHALL be in French

#### Scenario: Notification Rust command accepts arbitrary title and body

- **WHEN** `notification_show(title, body)` is called from the frontend
- **THEN** Rust SHALL display the notification with the provided title and body without any string construction
