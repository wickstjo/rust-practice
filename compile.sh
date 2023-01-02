# READ FILENAME FROM ARGS
FILE_NAME=$1

# CLEAR & COMPILE
clear
rustc my_first_project/src/$FILE_NAME.rs -o outputs/$FILE_NAME.out

# RUN OUTPUT FILE
./outputs/$FILE_NAME.out