# msg-in-png
Learning Rust by following the walk through from __https://picklenerd.github.io/pngme_book/introduction.html__

## What are we making?
We're making a command line program that lets you hide secret messages in PNG files. Your program will have four commands:

- Encode a message into a PNG file
- Decode a message stored in a PNG file
- Remove a message from a PNG file
- Print a list of PNG chunks that can be searched for messages

If that sounds scary and beyond your ability then this guide is _definitely_ for you. 
If you know how to write code, and you know your Rust basics, you can totally do this.
We're not going to implement any sort of image decoding. The part of the PNG spec we're tackling is surprisingly simple.