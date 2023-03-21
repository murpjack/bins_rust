# Never forget when it's bin day again!

Print a symbol [in the Vim statusline] for which bins being collected.

## Data shape
```
  let bin_day = {
    "date": "12345678890",
    "bins": ["recycling"]
  }
```
Note- my local council don't make bin day data easy to access. 
As a result I wrote a quick & dirty script to scrape from the council website.

## Future features: 
* Integrate with Vim, as this is not currently implemented.
* Make this plugin better for other users 
  - ie reshape data to be easier to understand, compatible with other bin data & types 
 
