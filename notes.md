# Notes for this project

# Basic Functionality
[x] Command line input
    [x] 'md_puppy init' create folders using directory_processing::_init_directories()
    [x] 'md_puppy build' copy static folder over using
        directory_processing::_copy_static() and then add pages and categories by
        traversing all folders in 'content'
[x] Nav header based on categories
    [x] Category 'null', '', or 'draft' for not displaying a category
    [ ] Automatically create a default index page if one does not exist.
[x] Index pages for the categories?
    [x] Index created as {{ Index: <category> }} and then produces a <li> of links
[x] Image embeds
    - I think that the library already takes care of that.
[ ] Config file?
    - Useful for determining project root
    - Could set it to have a default location to pull static folder; instead
      of always pulling from repo.
    - Base url for <base> tags might not be too useful...

## Fun features
[ ] Add image processing for dithering images
    - https://crates.io/crates/image
    - https://docs.rs/image/latest/image/imageops/colorops/fn.dither.html
    - https://endtimes.dev/why-you-should-dither-images/
[ ] Add page previews created on build
    - https://og-image.vercel.app/
[ ] Embedding Youtube videos
