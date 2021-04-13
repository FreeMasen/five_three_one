print_ticks () {
    echo "\`\`\`$1" >> ./README.md
}

print_usage() {
    echo "" >> ./README.md
    echo "# $1" >> ./README.md
    echo "" >> ./README.md
    print_ticks "sh"
    cargo run -- "$(echo "$1"  | tr '[:upper:]' '[:lower:]')" --help >> ./README.md    
    print_ticks ""
}

print_usage "Init"
print_usage "Next"
print_usage "Generate"

