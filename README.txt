Authors: 
    Peter Finch & Abraham Chango

Brightness Implementation:
    Correctly implemented:
        Brightness takes in one command line argument that is a filepath to a portable graymap image in pbm, pgm, and ppm.
        Brightness returns the average brightness in the image by converting each pixel from Rgba<u8> to Luma<u8>
        If no argument is given brightness exits and prints an error to stderr.
        If a portable graymap image is promised but not delivered brightness will halt and print on stderr "Not graymap file.".

    Not implemented:
        N/A: All requirements were implemented

Fgroups Implementation:
    Correctly implemented:
        Fgroups prints fingerprint groups in the following format:
            If there are no fingerprint groups, print nothing.
            If there is exactly one fingerprint group, print it.
            If there are multiple fingerprint groups, print them seperated by newlines.
        Fgroups handles badly formed input lines by discarding the line and writing a message to stderr.
        Fgroups handles fingerprints of more than 512 characters by discarding the line and writing a message to stderr.
        Fgroups will not crash due to repeating names.
        Fgroups can process 5000 lines in 0.5 seconds, so 10,000 lines per second, meaning ~600,000 lines per minute assuming linear relationship.

    Not implemented:
        

Fgroups Problem Solving:
    Sorting large amounts of data by a categorical trait i.e.
    - A hospital could sort a large number of patients based on what condition they have
    - A researcher has a dataset of butterfly attributes. The researcher wants to examine the attributes more closely and needs to sort his dataset by species.
    - A sociologist could use fgroups sort a dataset by demographic charcteristics of high birthrates and high infant death rates in societies with minimal technology

Time Spent on Project:
    Approximately 22 hours.