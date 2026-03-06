## MODIFIED Requirements

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
