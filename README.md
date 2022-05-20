# what is tif?

tif is image file format focused on the terminal. its a pretty simple image file format!

# what does this library does

## parsing 

there are two ways of parsing with libtif
``` rust
let image = TifImage::parse_from_file("my_image.tif")?;
```

and 

``` rust
let image = TifImage::parse_from_bytes(vec![])?;
```

you'll use `parse_from_bytes` either way, but parse_from_file is for lazy people like.

## saving

to save a tif file you just need one line of code

``` rust
    let your_file_as_bytes = your_tif_image.save();
```

# how to create tif images

if you wanna create tif images programatically, you'll need to create that struct by yourself, initializing all the fields by yourself. this library isnt meant for that kind of thing!

but if you wanna create tif images through your terminal, you can use my program [tif_editor](https://github.com/pietro222222/tif_editor)
