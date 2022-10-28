# Amiigen CLI

Commandline tool that works with amiibo. Produces amiibo in the same format as amiitool. 

Generating requires a valid tag UID that is 7 or 9 bytes. It checks the validness by ensuring the first byte is 04. 
Enter amiibo IDs and tag UIDs without the 0x. The app directly reads the bytes, with no spaces. It also expects a key file.
It does NOT check integrety of the key file. 

This key file can be dumped from a switch or found online. I will not explain this any further. 

I get the amiibo ids from [here](https://hax0kartik.github.io/amiibo-generator/). The bin files produced by 
this website don't work, even when encrypted. The ids are valid though.
