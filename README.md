dirsplitter
===========

The `split` binary takes as input a large directory of images. If the number of images exceeds
a value, subdirectories are created and the images are moved to those directories.

If a directory cannot be created or file cannot be moved, the process is aborted.

The `unsplit` binary undoes the splitting performed by the `split` binary. 

If a file cannot be moved or a directory cannot be deleted, the error is logged and 
the process continues.