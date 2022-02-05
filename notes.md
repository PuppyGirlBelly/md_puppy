# Notes for this project
Gods we should have started a notes file ages ago.

~~TODO: For some reason the parser will halt parsing after '---' and other strings. Need to investigate.~~
GUESS WHAT! IT WAS ALL BECAUSE I FORGOT TO LIMIT HOW MANY TIMES IT SPLITS EACH
INSTANCE OF '---' AND I SPENT A DAY DEBUGGING THE LIBRARY RATHER THAN ASSUMING
I MADE SOME DUMBASS MISTAKE. 

THE LESSON LEARNED; GREP A STRING THAT'S GIVING YOU TROUBLE RATHER THAN
ASSUMING IT'S A LIBRARY

# Basic Functionality
[x] Command line input
    [x] 'md_puppy init' create folders using directory_processing::_init_directories()
    [x] 'md_puppy build' copy static folder over using
        directory_processing::_copy_static() and then add pages and categories by
        traversing all folders in 'content'
[ ] Nav header based on categories
    [ ] Category 'null', '', or 'draft' for not displaying a category
[ ] Index pages for the categories?
    [ ] Index created as {{ Index: <category> }} and then produces a <li> of links
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
