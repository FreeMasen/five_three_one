
echo "<!DOCTYPE html>\
<html>\
<head>\
<title>531 Reports</title>\
</head>\
<body>\
<h1>531 Reports</h1>\
<ul>\
" > reports/index.html
for file in reports/*
do
    stem=$(basename -s .html "$file")
    if [ "$stem" != "index" ]; then
        echo "<li><a href=""$stem".html">$stem</a></li>\
        " >> reports/index.html
    fi
done

echo "</ul></body></html>" >> reports/index.html
