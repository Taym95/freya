### corner_radius & corner_smoothing

The `corner_radius` attribute let's you smooth the corners of the element, with `corner_smoothing` you can give a "squircle" effect.

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            corner_radius: "10",
            corner_smoothing: "75%"
        }
    )
}
```