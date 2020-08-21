# silviaPiPID

This is currenlty a work in progress of some test code. I hope to break it down into more re-usable library like components, instead of just modules. So far I have tree modules. joy_pad, fb, and canvas.

1. joy_pad - is a generic button handler where each button takes an analog pin, and each button pulls Low. 

2. fb - is a low level interface for loading graphic primitives onto the frame buffer. It assumes a 16 bit model at the moment and should eventually handle Rects, Filled Rects, Bitmaps, Vertical Lines, Horizontal Lines, Single Pixels, (maybe) Elipses, (maybe) rounded corner Rects, Fonts.

3. canvas - is the layer handling "compositor" system. It builds on top of the fb layer and allows manipulation of multiple items in groups, and orders the final output, prior to flushing the fb buffer to the display.
