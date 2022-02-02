# Notes for this project
Gods we should have started a notes file ages ago.

Okay so what do we want it to do now...
1) Command line input
    1a) 'md_puppy init' create folders using directory_processing::_init_directories()
    1b) 'md_puppy build' copy static folder over using
        directory_processing::_copy_static() and then add pages and categories by
        traversing all folders in 'content'
2) Nav header based on categories
    2a) Add category 'null', '', or 'draft' for not displaying a category
3) Add image processing for dithering images
    3a) https://crates.io/crates/image
        https://docs.rs/image/latest/image/imageops/colorops/fn.dither.html
        https://endtimes.dev/why-you-should-dither-images/
4) Add page previews created on build
    4a) https://og-image.vercel.app/
