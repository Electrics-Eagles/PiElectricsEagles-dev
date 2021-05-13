#/usr/bin

rm -rf last.file

rm -rf path.file

touch last.file
touch path.file

echo 0 > last.file




for z in {1..1}

do
	mkdir  $z
	echo $z > path.file
	RESULT=$(awk "BEGIN {printf \"%.3f\",${z}/1000}")
	echo $RESULT
	./Test_Filter $RESULT
done

for f in {1..1}
do
	echo $f
	cd $f
	convert -delay 20 -loop 0 *.png $f.gif
	cd -
done

mkdir gifs
for f in {1..1}
do
	echo $f

	
	sudo cp $f/$f.gif ./gifs/$f.gif
	
done


	
	
