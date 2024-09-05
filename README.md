# Amiigen CLI

Commandline tool that works with amiibo. Produces amiibo in the same format as amiitool. 

Generating requires a valid tag UID that is 7 or 9 bytes. It checks the validness by ensuring the first byte is 04. 
Enter amiibo IDs and tag UIDs without the 0x. The app directly reads the bytes, with no spaces. It also expects a key file.
It does NOT check integrety of the key file. 

This key file can be dumped from a switch or found online. I will not explain this any further. 

I get the amiibo ids from [here](https://hax0kartik.github.io/amiibo-generator/). The bin files produced by 
this website don't work, even when encrypted. The ids are valid though.

Usage:
```
amiigen-cli encrypt -i <INPUT_BIN> -k <KEY_RETAIL> -o <OUTPUT_BIN>
amiigen-cli decrypt -i <INPUT_BIN> -k <KEY_RETAIL> -o <OUTPUT_BIN>
```
`-i` and `-o` may be omitted. If they are omitted it reads from stdin and prints to stdout. Be careful, stdout printing can clobber
your shell!

```
amiigen-cli generate -k <KEY_RETAIL> --id <AMIIBO_ID> --uid <TAG_ID> -o <OUTPUT_BIN>
amiigen-cli generate-raw --id <AMIIBO_ID> --uid <TAG_ID> -o <OUTPUT_BIN>
```
`-o` may be omitted. If it is omitted the result is printed to stdout. 
generate is the same as generate raw then converting the resulting bin with encrypt. 
If you don't have a tag id, you'll have to use a dummy one like 04A84501500001. Tools like tagmiibo and emuiibo don't care about the tag UID.


