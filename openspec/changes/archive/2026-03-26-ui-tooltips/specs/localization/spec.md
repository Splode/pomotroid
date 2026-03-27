## ADDED Requirements

### Requirement: Tooltip strings in all locales

All tooltip i18n keys introduced by the `ui-tooltips` change SHALL be present in every supported locale message file (`src/messages/*.json`). Non-English translations MAY be machine-translated for the initial release.

The following keys SHALL be added:

| Key | English value |
|---|---|
| `tooltip_settings` | `"Open Settings"` |
| `tooltip_statistics` | `"Open Statistics"` |
| `tooltip_restart_round` | `"Restart the current round from the beginning."` |
| `tooltip_skip` | `"Skip to the next round."` |
| `tooltip_reset` | `"Reset the timer to the first work round. Current session progress will be cleared."` |
| `tooltip_mute` | `"Mute alert sounds"` |
| `tooltip_unmute` | `"Unmute alert sounds"` |
| `tooltip_verbose_logging` | `"Enables detailed debug logging. Use when reporting issues. Log files are accessible via Open Log Folder in Advanced settings."` |
| `tooltip_dial_countdown` | `"When enabled, the progress arc counts down to zero instead of filling up."` |
| `tooltip_auto_start_work` | `"Automatically start the next work round when a break ends."` |
| `tooltip_auto_start_break` | `"Automatically start the break when a work round completes."` |
| `tooltip_websocket` | `"Enables a local WebSocket server for external integrations such as stream overlays. Disabled by default."` |
| `tooltip_round_counter` | `"Current work round out of the total rounds before a long break."` |
| `tooltip_round_counter_session` | `"Continuous session round count. Resets only when the timer is reset."` |

Note: `system_tray_gnome_hint` already exists in all locales and is reused by the Linux-only `TooltipInfo` icon on the System Tray toggle; no new key is needed for it.

#### Scenario: All tooltip keys present in non-English locales

- **WHEN** a non-English message file is loaded
- **THEN** every tooltip key listed above SHALL have a corresponding entry in that file
