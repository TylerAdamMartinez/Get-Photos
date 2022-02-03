#!/bin/bash

echo "Enter in folder name: "
read FolderName
FolderPath=`pwd`
echo "made /${FolderName} folder in ${FolderPath}"
mkdir $FolderName

IMG_TYPE[0]="*.jpg"
IMG_TYPE[1]="*.jpeg"
IMG_TYPE[2]="*.png"
IMG_TYPE[3]="*.gif"
IMG_TYPE[4]="*.tiff"
IMG_TYPE[5]="*.psd"
IMG_TYPE[6]="*.pdf"
IMG_TYPE[7]="*.eps"
IMG_TYPE[8]="*.ai"
IMG_TYPE[9]="*.raw"

for i in "${IMG_TYPE[@]}"
do
    echo "Finding ${i} file types..."
    cd $FolderName
    mkdir ${i:2}
    cd ..
    find . -type f -name $i | xargs -I{} cp {} $FolderName/${i:2}
    echo "Been copied to ${FolderPath}/${FolderName}/${i:2}"
done

