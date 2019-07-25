#!/bin/bash
# Open a file with rstudio without locking terminal

rstudio $1 &> /dev/null & disown
