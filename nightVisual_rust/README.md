# Night Visual Rust

Rust implementation of the Night Visual library. Both visual data i.e. image and video is to be supported by this library. Since, it is an ongoing project, only images are supported for now and that also in limits. All the limitations and planned features are listed at the end.

## Image

Exposes three main structs -
* `ImageConfig` - Stores all the configurations regarding the dark mode operation. 
* `ImageBuffer` - The image buffer containing width, height and the pixel data of given image
* `ImageContext` - The context object that exposes the main `apply_dark_mode()` function.

### ImageConfig

`ImageConfig` is used by the `ImageContext` instance to apply different strategies for dark mode. It exposes various methods to create new instance and set new configurations.

| Config name | Default Value | Description |
| ----------- | ------------- | ----------- |
| `indicate_existing_colors` | `false` | Used to decide whether to indicate existing colors of pixels |
| `dark_mode_policy` | `DarkModePolicy::INVERSE` | Which dark mode policy should be followed |

#### Methods/Functions

##### ImageConfig::new()

Returns a new instance of ImageConfig with default values
```rust
let image_config = ImageConfig::new();
```

##### ImageConfig::set_indicate_existing_colors(on: bool)

Sets the `set_indicate_existing_colors` config for the `ImageConfig` instance on which this method is called. Returns the modified `ImageConfig` instance.
```rust
let image_config = image_config.set_indicate_existing_colors(true);
```

##### ImageConfig::set_dark_mode_policy(policy: DarkModePolicy)

Sets the `darK_mode_policy` config for the `ImageConfig` instance on which this method is called. Returns the modified `ImageConfig` instance.
```rust
let image_config = image_config.set_dark_mode_policy(DarkModePolicy::INVERSE);
```

##### ImageConfig::indicate_existing_colors()

Returns the value of `indicate_existing_colors` config of the `ImageConfig` instance on which it is called.
```rust
let indicate_existing_colors_config: bool = image_config.indicate_existing_colors();
```

##### ImageConfig::dark_mode_policy()

Returns the policy set for the `ImageConfig` instance on which it is called.
```rust
let dark_mode_policy_config: &DarkModePolicy = image_config.dark_mode_policy();
```

##### ImageConfig::create_context()

Creates and returns a new instance of `ImageContext` struct. The `ImageConfig` instance on which this method is called is moved to the new `ImageContext` instance. Returned `ImageContext` instance contains only the image_config. `Buffer` needs to be set using `ImageContext::set_buffer()` method.

```rust
let image_context = image_config.create_context();
```

### ImageBuffer

`ImageBuffer` contains the width and height of the input image and a vector of `Pixel`s representing the pixel data of the image. It implements the `ImgView` trait.

#### Methods/Functions

##### ImageBuffer::new_from_raw_data(width: u32, height: u32, data: &[u8])

Creates and returns a new instance of `ImageBuffer` instance. Takes three parameters -
* `width` - Width of the image.
* `height` - Height of the image.
* `data` - The raw data containing pixel component values in this pattern - [pixel1_r, pixel1_g, pixel1_b, pixel1_a, pixel2_r, pixel2_g ...]

##### ImageBuffer::new_from_pixels(width: u32, height: u32, data: Vec<Pixel>)

Creates and Returns a new instance of `ImageBuffer` instance. `data` is a vector of `Pixel`s.

## Limitations

* Video input still not supported.
* Only one dark_mode_policy is supported for now.
* There are many edge cases which requires dedicated dark_mode_policy to get resolved.

This is an ongoing project, so will address the limitations.