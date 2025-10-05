# Logo Configuration Guide

rfetch supports displaying custom image logos instead of ASCII art when your terminal supports it.

## Configuration

Add these options to your `~/.config/rfetch/config.toml`:

```toml
# Optional: Path to your custom logo image
# If not set or file doesn't exist, ASCII art will be used
# custom_logo_path = "/path/to/your/logo.png"

# Logo dimensions (pixels)
logo_width = 30    # Width in character cells (default: 30)
logo_height = 20   # Height in character cells (default: 20)
```

## Supported Formats

- PNG
- JPEG/JPG  
- WebP
- GIF (static and animated)
- Many other formats supported by the `image` crate

## Terminal Compatibility

**Full Image Support:**
- Kitty terminal (best support)
- iTerm2 (macOS)
- WezTerm
- Some newer terminals with graphics protocol support

**Fallback:**
- All other terminals will automatically use ASCII art
- No configuration needed - fallback is automatic

## Recommended Logo Dimensions

- **Pixel size**: 120x120 to 240x240 pixels work well
- **Aspect ratio**: Square (1:1) looks best
- **File size**: Keep under 1MB for fast loading
- **Format**: PNG recommended for logos with transparency

## Example Configuration

```toml
# Example with custom Arch Linux logo
custom_logo_path = "~/.config/rfetch/arch-logo.png"
logo_width = 25
logo_height = 25
```

The logo will be displayed at the specified dimensions and automatically fall back to ASCII art if:
- The terminal doesn't support images
- The image file doesn't exist
- The image fails to load

## Testing Your Setup

1. Set `custom_logo_path` to your image
2. Run `rfetch`
3. If you see ASCII art, either your terminal doesn't support images or the path is incorrect
4. Check that the file path exists and the terminal supports graphics protocols