# Bidirectional Sync

## Overview

The bidirectional sync system keeps your Blueprint visual representation and code files in perfect synchronization. You can edit either the visual Blueprint or the code file, and changes are automatically reflected in the other.

## How It Works

```
Code File (.pwn)
    ↕ (Parser)
AST (Abstract Syntax Tree)
    ↕ (Converters)
Blueprint Graph
    ↕ (Generator)
Code File (.pwn)
```

### Sync Flow

1. **Code → Blueprint**: Code changes are parsed into AST, then converted to Blueprint nodes
2. **Blueprint → Code**: Blueprint changes are converted to AST, then generated as code

## Sync Modes

### Auto-Sync (Default)

- **Code Changes**: Automatically updates Blueprint when code file changes
- **Blueprint Changes**: Automatically updates code when Blueprint changes
- **Debounce**: 500ms delay to prevent excessive updates

### Manual Sync

- Disable auto-sync in preferences
- Use "Sync" button to manually trigger synchronization
- Useful for large files or when you want control

## Conflict Resolution

When both code and Blueprint are edited simultaneously, conflicts may occur.

### Conflict Detection

The system detects conflicts by:
- Comparing code hash with last known hash
- Comparing Blueprint hash with last known hash
- Detecting simultaneous changes

### Resolution Strategies

#### Code Wins
- Code changes take precedence
- Blueprint is regenerated from code
- Use when code is the source of truth

#### Blueprint Wins
- Blueprint changes take precedence
- Code is regenerated from Blueprint
- Use when visual editing is primary

#### Ask User
- System prompts you to choose
- Shows what changed in each
- You decide which to keep

#### Merge (Experimental)
- Attempts to merge both changes
- Preserves code structure and Blueprint layout
- May require manual adjustment

## Sync Preferences

Configure sync behavior in settings:

```typescript
interface SyncPreferences {
  autoSyncOnCodeChange: boolean;      // Auto-sync code → Blueprint
  autoSyncOnBlueprintChange: boolean; // Auto-sync Blueprint → code
  conflictResolution: "code_wins" | "blueprint_wins" | "ask_user" | "merge";
  debounceDelay: number;              // Delay in milliseconds
  preservePositions: boolean;         // Keep node positions when syncing
}
```

## File Watching

The system watches for file changes:

- **Code Files**: Monitored via Tauri file watcher
- **Blueprint Files**: Monitored when saved
- **External Edits**: Detected automatically

## Position Preservation

When syncing from code to Blueprint:

- **Preserve Positions**: Node positions are maintained if possible
- **Auto-Layout**: New nodes are positioned automatically
- **Smart Placement**: Related nodes are placed near each other

## Sync State

Track sync status:

- **Last Code Hash**: Hash of last synced code
- **Last Blueprint Hash**: Hash of last synced Blueprint
- **Sync Timestamps**: When each side was last synced
- **Sync Status**: Whether sync is in progress

## Best Practices

### For Code-First Workflow

1. Edit code in external editor
2. Let auto-sync update Blueprint
3. Use Blueprint for visualization
4. Set conflict resolution to "code_wins"

### For Blueprint-First Workflow

1. Edit visually in Blueprint editor
2. Let auto-sync update code
3. Use code for final adjustments
4. Set conflict resolution to "blueprint_wins"

### For Mixed Workflow

1. Use "ask_user" conflict resolution
2. Sync manually when switching between modes
3. Keep changes small and focused
4. Review conflicts carefully

## Troubleshooting

### Sync Not Working

- Check file watcher is enabled
- Verify file permissions
- Check for parse errors in code
- Ensure Blueprint is valid

### Conflicts Appearing Frequently

- Increase debounce delay
- Use manual sync mode
- Work in one mode at a time
- Save frequently

### Lost Changes

- Check conflict resolution settings
- Review sync history if available
- Use version control (Git) for safety
- Export Blueprint before major changes

## Advanced: Custom Sync Logic

You can extend the sync system:

1. **Custom Converters**: Add language-specific converters
2. **Sync Hooks**: Add callbacks for sync events
3. **Conflict Handlers**: Implement custom conflict resolution
4. **Sync Filters**: Filter what gets synced

## Example Workflow

```
1. Create Blueprint visually
   → Auto-sync generates code
   
2. Edit code in external editor
   → Auto-sync updates Blueprint
   
3. Make visual changes
   → Auto-sync updates code
   
4. Conflict detected
   → User chooses "code_wins"
   → Blueprint regenerated
```
