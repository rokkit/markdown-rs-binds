# [markdown-rs](https://github.com/wooorm/markdown-rs) iOS bindings

## Build

```sh
make build
```

## Use

```swift
func render() -> String {
    return String(cString: render_html("## Hello, *world*!"))
}
```
